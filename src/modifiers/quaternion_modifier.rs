use super::{ModifierLike, operation::Operation, shared_has_base_provider};
use crate::prelude::{AbstractValueProvider, ValueProvider};
use crate::{base_provider_context::BaseProviderContext, quaternion_utils::QuaternionUtilsExt};
use glam::Vec3A;
use glam::{Quat, Vec3};

#[derive(Debug, Clone)]
pub enum QuaternionValues {
    // equivalents but different repr
    Static(Vec3, Quat),
    Dynamic(Vec<ValueProvider>),
}

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
        let has_base_provider =
            shared_has_base_provider(matches!(point, QuaternionValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }

    fn translate_euler(values: &[ValueProvider], context: &BaseProviderContext) -> Vec3 {
        let mut vec3 = Vec3::ZERO;

        // Collect values from each provider into a local variable and copy them into vec3
        // avoid allocations with Vec
        let mut count = 0usize;
        'outer: for provider in values {
            let vals = provider.values(context);
            for v in vals.iter() {
                if count >= Self::VALUE_COUNT {
                    break 'outer;
                }
                vec3[count] = *v;
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
            acc_a = match quat_point.get_operation() {
                Operation::Add => acc_a + v_a,
                Operation::Sub => acc_a - v_a,
                Operation::Mul => acc_a * v_a,
                Operation::Div => acc_a / v_a,
                Operation::None => v_a,
            };
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

        Quat::from_unity_euler_degrees(&Vec3::new(vector_point.x, vector_point.y, vector_point.z))
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
