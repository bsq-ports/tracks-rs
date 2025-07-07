use super::{Modifier, ModifierBase, operation::Operation};
use super::{ModifierValues, shared_has_base_provider};
use crate::base_provider_context::BaseProviderContext;
use glam::Vec4;

pub type Vector4Values = ModifierValues<Vec4>;

#[derive(Debug)]
pub struct Vector4Modifier {
    values: Vector4Values,
    has_base_provider: bool,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl Vector4Modifier {
    pub fn new(point: Vector4Values, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        let has_base_provider =
            shared_has_base_provider(matches!(point, Vector4Values::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for Vector4Modifier {
    type Value = Vec4;
    const VALUE_COUNT: usize = 4;

    fn get_point(&self, context: &BaseProviderContext) -> Vec4 {
        let original_point = match &self.values {
            Vector4Values::Static(s) => *s,
            Vector4Values::Dynamic(value_providers) => self.convert(value_providers, context),
        };
        let result = self
            .modifiers
            .iter()
            .fold(original_point, |acc, x| match x.get_operation() {
                Operation::Add => acc + x.get_vector4(context),
                Operation::Sub => acc - x.get_vector4(context),
                Operation::Mul => acc * x.get_vector4(context),
                Operation::Div => acc / x.get_vector4(context),
                Operation::None => x.get_vector4(context),
            });
        result
    }

    fn get_raw_point(&self) -> Vec4 {
        match self.values {
            Vector4Values::Static(s) => s,
            _ => Vec4::ZERO,
        }
    }

    fn translate(&self, values: &[f32]) -> Vec4 {
        Vec4::new(values[0], values[1], values[2], values[3])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn has_base_provider(&self) -> bool {
        self.has_base_provider
    }
}
