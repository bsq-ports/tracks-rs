use super::{ModifierLike, operation::Operation};
use super::{ModifierValues, shared_has_base_provider};
use crate::base_provider_context::BaseProviderContext;
use crate::providers::AbstractValueProvider;
use crate::value_types::ValueType;

#[derive(Debug, Clone)]
pub struct BasicModifier<T: ValueType> {
    values: ModifierValues<T>,
    has_base_provider: bool,
    modifiers: Vec<BasicModifier<T>>,
    operation: Operation,
}

impl<T: ValueType> BasicModifier<T>
where
    [(); T::VALUE_COUNT]:,
{
    pub fn new(
        point: ModifierValues<T>,
        modifiers: Vec<BasicModifier<T>>,
        operation: Operation,
    ) -> Self {
        let has_base_provider =
            shared_has_base_provider(matches!(point, ModifierValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }
}

impl<T: ValueType> ModifierLike<T> for BasicModifier<T>
where
    [(); T::VALUE_COUNT]:,
{
    const VALUE_COUNT: usize = T::VALUE_COUNT;

    fn get_modified_point(&self, context: &BaseProviderContext) -> T {
        let original_point = match &self.values {
            ModifierValues::Static(s) => *s,
            ModifierValues::Dynamic(value_providers) => {
                let mut values = [0.0; T::VALUE_COUNT];
                let mut i = 0;
                for value in value_providers {
                    for v in value.values(context) {
                        if i >= T::VALUE_COUNT {
                            break;
                        }
                        values[i] = v;
                        i += 1;
                    }
                }
                T::from_translate_array(values)
            }
        };
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_modified_point(context),
                Operation::Sub => acc - x.get_modified_point(context),
                Operation::Mul => acc * x.get_modified_point(context),
                Operation::Div => acc / x.get_modified_point(context),
                Operation::None => x.get_modified_point(context),
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
