use super::{Modifier, ModifierBase, operation::Operation};
use super::{ModifierValues, shared_has_base_provider};
use crate::base_provider_context::BaseProviderContext;

pub type FloatValues = ModifierValues<f32>;

pub struct FloatModifier {
    values: FloatValues,
    has_base_provider: bool,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl FloatModifier {
    pub fn new(point: FloatValues, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        let has_base_provider =
            shared_has_base_provider(matches!(point, FloatValues::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for FloatModifier {
    type Value = f32;
    const VALUE_COUNT: usize = 1;

    fn get_point(&self, context: &BaseProviderContext) -> f32 {
        let original_point = match &self.values {
            FloatValues::Static(s) => *s,
            FloatValues::Dynamic(value_providers) => self.convert(value_providers, context),
        };
        self.modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_float(context),
                Operation::Sub => acc - x.get_float(context),
                Operation::Mul => acc * x.get_float(context),
                Operation::Div => acc / x.get_float(context),
                Operation::None => x.get_float(context),
            })
    }

    fn get_raw_point(&self) -> f32 {
        self.values.as_static_values().copied().unwrap_or_default()
    }

    fn translate(&self, values: &[f32]) -> f32 {
        values[0]
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn has_base_provider(&self) -> bool {
        self.has_base_provider
    }
}
