use super::{ModifierLike, operation::Operation};
use super::{ModifierValues, shared_has_base_provider};
use crate::base_provider_context::BaseProviderContext;
use crate::providers::AbstractValueProvider;
use crate::value_types::{MAX_COMPONENTS, ValueType};

/// A basic, typed modifier that applies component-wise operations to point values.
///
/// `BasicModifier<T>` holds either static values or dynamic `ValueProvider`s,
/// a list of nested modifiers, and an `Operation` describing how to combine
/// nested modifier results with the base point. It implements `ModifierLike<T>`
/// for use by the point-definition parsing and evaluation machinery.
#[derive(Debug, Clone)]
pub struct BasicModifier<T: ValueType> {
    values: ModifierValues<T>,
    has_base_provider: bool,
    modifiers: Vec<BasicModifier<T>>,
    operation: Operation,
}

impl<T: ValueType> BasicModifier<T> {
    pub fn new(
        point: ModifierValues<T>,
        modifiers: Vec<BasicModifier<T>>,
        operation: Operation,
    ) -> Self {
        // fold fully static nested modifiers into the point, so evaluating it is a single match.
        // nested modifiers were folded when they were built, so a static subtree is already flat
        let (point, modifiers) = match point {
            ModifierValues::Static(value)
                if !modifiers.is_empty() && modifiers.iter().all(Self::is_flat_static) =>
            {
                let folded = modifiers
                    .iter()
                    .fold(value, |acc, m| m.operation.apply(acc, m.get_raw_point()));
                (ModifierValues::Static(folded), Vec::new())
            }
            point => (point, modifiers),
        };

        let has_base_provider =
            shared_has_base_provider(matches!(point, ModifierValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }

    /// A static value with no nested modifiers.
    fn is_flat_static(&self) -> bool {
        matches!(self.values, ModifierValues::Static(_)) && self.modifiers.is_empty()
    }
}

impl<T: ValueType> ModifierLike<T> for BasicModifier<T> {
    const VALUE_COUNT: usize = T::VALUE_COUNT;

    fn get_modified_point(&self, context: &BaseProviderContext) -> T {
        let original_point = match &self.values {
            ModifierValues::Static(s) => *s,
            ModifierValues::Dynamic(value_providers) => {
                let mut values = [0.0; MAX_COMPONENTS];
                let provided = value_providers.iter().flat_map(|p| p.values(context));
                for (slot, v) in values[..T::VALUE_COUNT].iter_mut().zip(provided) {
                    *slot = v;
                }
                T::from_translate_slice(&values[..T::VALUE_COUNT])
            }
        };
        self.modifiers.iter().fold(original_point, |acc, x| {
            x.get_operation().apply(acc, x.get_modified_point(context))
        })
    }

    fn get_raw_point(&self) -> T {
        self.values.as_static_values().copied().unwrap_or_default()
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
    use glam::{Vec3, Vec4};
    use smallvec::smallvec;

    use super::*;
    use crate::providers::{ValueProvider, r#static::StaticValues};

    type Make<T> = fn(ModifierValues<T>, Vec<BasicModifier<T>>, Operation) -> BasicModifier<T>;

    /// Builds the struct directly, skipping the folding in `new`.
    fn unfolded<T: ValueType>(
        values: ModifierValues<T>,
        modifiers: Vec<BasicModifier<T>>,
        operation: Operation,
    ) -> BasicModifier<T> {
        BasicModifier {
            values,
            has_base_provider: false,
            modifiers,
            operation,
        }
    }

    /// `point + a, * (b * a), / b, - a`, with one nested level.
    fn tree<T: ValueType>(make: Make<T>, point: T, a: T, b: T) -> BasicModifier<T> {
        let leaf = |v, op| make(ModifierValues::Static(v), vec![], op);
        make(
            ModifierValues::Static(point),
            vec![
                leaf(a, Operation::Add),
                make(
                    ModifierValues::Static(b),
                    vec![leaf(a, Operation::Mul)],
                    Operation::Mul,
                ),
                leaf(b, Operation::Div),
                leaf(a, Operation::Sub),
            ],
            Operation::None,
        )
    }

    fn assert_folds_to_same_value<T: ValueType + PartialEq + std::fmt::Debug>(
        point: T,
        a: T,
        b: T,
    ) {
        let ctx = BaseProviderContext::new();
        let folded = tree(BasicModifier::new, point, a, b);
        let expected = tree(unfolded, point, a, b);

        assert!(folded.modifiers.is_empty(), "static tree should fold");
        assert_eq!(
            folded.get_modified_point(&ctx),
            expected.get_modified_point(&ctx)
        );
    }

    #[test]
    fn static_modifiers_fold_to_the_same_value() {
        assert_folds_to_same_value(1.5_f32, 2.0, 4.0);
        assert_folds_to_same_value(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::splat(2.0),
            Vec3::new(4.0, 5.0, 6.0),
        );
        assert_folds_to_same_value(
            Vec4::splat(1.0),
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::splat(8.0),
        );
    }

    #[test]
    fn dynamic_values_are_not_folded() {
        let dynamic = || {
            ModifierValues::<f32>::Dynamic(smallvec![ValueProvider::Static(StaticValues::new(
                smallvec![2.0],
                false
            ))])
        };
        let static_leaf =
            || BasicModifier::new(ModifierValues::Static(1.0), vec![], Operation::Add);

        let dynamic_point = BasicModifier::new(dynamic(), vec![static_leaf()], Operation::None);
        assert_eq!(dynamic_point.modifiers.len(), 1);

        let dynamic_child = BasicModifier::new(
            ModifierValues::Static(1.0),
            vec![BasicModifier::new(dynamic(), vec![], Operation::Add)],
            Operation::None,
        );
        assert_eq!(dynamic_child.modifiers.len(), 1);
        assert_eq!(
            dynamic_child.get_modified_point(&BaseProviderContext::new()),
            3.0
        );
    }
}
