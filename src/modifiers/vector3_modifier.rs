use super::{Modifier, ModifierBase, operation::Operation};
use super::{ModifierValues, shared_has_base_provider};
use crate::base_provider_context::BaseProviderContext;
use glam::{Vec3, Vec3A};

pub type Vector3Values = ModifierValues<Vec3>;

#[derive(Debug)]
pub struct Vector3Modifier {
    values: Vector3Values,
    has_base_provider: bool,
    modifiers: Vec<Modifier>,
    operation: Operation,
}

impl Vector3Modifier {
    pub fn new(point: Vector3Values, modifiers: Vec<Modifier>, operation: Operation) -> Self {
        let has_base_provider =
            shared_has_base_provider(matches!(point, Vector3Values::Dynamic(_)), &modifiers);
        Self {
            values: point,
            has_base_provider,
            modifiers,
            operation,
        }
    }
}

impl ModifierBase for Vector3Modifier {
    type Value = Vec3;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        let original_point = match &self.values {
            Vector3Values::Static(s) => *s,
            Vector3Values::Dynamic(value_providers) => self.convert(value_providers, context),
        };
        // Use Vec3A for SIMD-friendly arithmetic in the hot path, then convert back to Vec3
        let result = self
            .modifiers
            .iter()
            .fold(Vec3A::from(original_point), |acc, x| {
                let op_vec = Vec3A::from(x.get_vector3(context));
                match x.get_operation() {
                    Operation::Add => acc + op_vec,
                    Operation::Sub => acc - op_vec,
                    Operation::Mul => acc * op_vec,
                    Operation::Div => acc / op_vec,
                    Operation::None => op_vec,
                }
            });
        Vec3::from(result)
    }

    fn get_raw_point(&self) -> Vec3 {
        self.values.as_static_values().copied().unwrap_or_default()
    }

    fn translate(&self, values: &[f32]) -> Vec3 {
        Vec3::new(values[0], values[1], values[2])
    }

    fn get_operation(&self) -> Operation {
        self.operation
    }

    fn has_base_provider(&self) -> bool {
        self.has_base_provider
    }
}
