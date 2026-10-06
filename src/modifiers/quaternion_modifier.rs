use super::{ModifierLike, operation::Operation, shared_has_base_provider};
use crate::prelude::{AbstractValueProvider, ValueProvider};
use crate::value_types::ValueType;
use crate::{base_provider_context::BaseProviderContext, quaternion_utils::QuaternionUtilsExt};
use glam::Vec3A;
use glam::{Quat, Vec3};
use smallvec::SmallVec;

/// Representation of the quaternion modifier input values.
///
/// For quaternions we store either a `Static` (Euler angles + quaternion) pair
/// or `Dynamic` providers that yield Euler components at runtime. The static
/// representation keeps both the vector (euler) and quaternion forms for
/// convenience.
#[derive(Debug, Clone)]
pub enum QuaternionValues {
    /// Static Euler vector and its quaternion equivalent.
    Static(Vec3, Quat),
    /// Dynamic providers for Euler components.
    Dynamic(SmallVec<[ValueProvider; 1]>),
}

/// Quaternion-specific modifier. It evaluates to a `Quat` by converting
/// Euler-component results into a quaternion. Nested quaternion modifiers are
/// applied component-wise to the Euler vector before conversion.
#[derive(Debug, Clone)]
pub struct QuaternionModifier {
    values: QuaternionValues,
    has_base_provider: bool,
    modifiers: Vec<QuaternionModifier>,
    operation: Operation,
}

impl QuaternionModifier {
    pub fn new(
        point: QuaternionValues,
        modifiers: Vec<QuaternionModifier>,
        operation: Operation,
    ) -> Self {
        // fold fully static nested modifiers into the point, so evaluating it skips the euler conversion.
        // nested modifiers were folded when they were built, so a static subtree is already flat
        let (point, modifiers) = match point {
            QuaternionValues::Static(vector, _)
                if !modifiers.is_empty() && modifiers.iter().all(Self::is_flat_static) =>
            {
                let folded = modifiers.iter().fold(vector, |acc, m| match m.values {
                    QuaternionValues::Static(v, _) => m.operation.apply(acc, v),
                    QuaternionValues::Dynamic(_) => unreachable!("checked by is_flat_static"),
                });
                let quat = Quat::from_unity_euler_degrees(folded);
                (QuaternionValues::Static(folded, quat), Vec::new())
            }
            point => (point, modifiers),
        };

        let has_base_provider =
            shared_has_base_provider(matches!(point, QuaternionValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }

    /// A static value with no nested modifiers.
    fn is_flat_static(&self) -> bool {
        matches!(self.values, QuaternionValues::Static(_, _)) && self.modifiers.is_empty()
    }

    fn translate_euler(values: &[ValueProvider], context: &BaseProviderContext) -> Vec3 {
        let mut vec3 = Vec3::ZERO;

        // Collect values from each provider into a local variable and copy them into vec3
        // avoid allocations with Vec
        let mut count = 0usize;
        'outer: for provider in values {
            let vals = provider.values(context);
            for v in vals {
                if count >= Vec3::VALUE_COUNT {
                    break 'outer;
                }
                vec3[count] = v;
                count += 1;
            }
        }

        vec3
    }

    pub fn get_vector_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = match &self.values {
            QuaternionValues::Static(s, _) => *s,
            QuaternionValues::Dynamic(value_providers) => {
                Self::translate_euler(value_providers, context)
            }
        };
        // Use Vec3A for accumulation in hot inner loop then convert back
        let mut acc_a = Vec3A::from(original_point);
        for quat_point in &self.modifiers {
            let v_a = Vec3A::from(quat_point.get_vector_point(context));
            acc_a = quat_point.get_operation().apply(acc_a, v_a);
        }

        Vec3::from(acc_a)
    }
}

impl ModifierLike<Quat> for QuaternionModifier {
    const VALUE_COUNT: usize = 3;

    fn get_modified_point(&self, context: &BaseProviderContext) -> Quat {
        if self.modifiers.is_empty() && matches!(self.values, QuaternionValues::Static(_, _)) {
            return self.get_raw_point();
        }
        // modifiers applied to the point
        let vector_point = self.get_vector_point(context);

        Quat::from_unity_euler_degrees(Vec3::new(vector_point.x, vector_point.y, vector_point.z))
    }

    fn get_raw_point(&self) -> Quat {
        match self.values {
            QuaternionValues::Static(_, q) => q,
            _ => Quat::IDENTITY,
        }
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn has_base_provider(&self) -> bool {
        self.has_base_provider
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(
        make: fn(QuaternionValues, Vec<QuaternionModifier>, Operation) -> QuaternionModifier,
    ) -> QuaternionModifier {
        let euler = |v: Vec3| QuaternionValues::Static(v, Quat::from_unity_euler_degrees(v));
        make(
            euler(Vec3::new(10.0, 20.0, 30.0)),
            vec![
                make(euler(Vec3::new(5.0, 0.0, -5.0)), vec![], Operation::Add),
                make(
                    euler(Vec3::splat(2.0)),
                    vec![make(euler(Vec3::splat(0.5)), vec![], Operation::Mul)],
                    Operation::Mul,
                ),
            ],
            Operation::None,
        )
    }

    #[test]
    fn static_modifiers_fold_to_the_same_rotation() {
        let ctx = BaseProviderContext::new();
        let folded = tree(QuaternionModifier::new);
        let expected = tree(|values, modifiers, operation| QuaternionModifier {
            values,
            has_base_provider: false,
            modifiers,
            operation,
        });

        assert!(folded.modifiers.is_empty(), "static tree should fold");
        assert!(
            folded
                .get_modified_point(&ctx)
                .abs_diff_eq(expected.get_modified_point(&ctx), 1e-6)
        );
    }
}
