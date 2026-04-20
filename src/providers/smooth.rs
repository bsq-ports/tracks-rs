use super::{UpdateableValues, clamp_lerp};

use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct SmoothProvidersValues {
    pub(crate) source_provider: crate::providers::ValueProvider,
    pub(crate) mult: f32,
    pub(crate) values: BaseValue,
}

impl SmoothProvidersValues {
    // Initialize from a source provider; sample initial length from context
    pub fn new(
        source_provider: crate::providers::ValueProvider,
        mult: f32,
        context: &BaseProviderContext,
    ) -> Self {
        // Initialize current values to zero/identity of the same shape as the source.
        let src = source_provider.values(context);
        let zero = match src {
            crate::base_value::BaseValue::Float(_) => crate::base_value::BaseValue::Float(0.0),
            crate::base_value::BaseValue::Vector2(_) => {
                crate::base_value::BaseValue::Vector2(glam::Vec2::ZERO)
            }
            crate::base_value::BaseValue::Vector3(_) => {
                crate::base_value::BaseValue::Vector3(glam::Vec3::ZERO)
            }
            crate::base_value::BaseValue::Vector4(_) => {
                crate::base_value::BaseValue::Vector4(glam::Vec4::ZERO)
            }
            crate::base_value::BaseValue::Quaternion(_) => {
                crate::base_value::BaseValue::Quaternion(glam::Quat::IDENTITY)
            }
        };

        Self {
            source_provider,
            mult,
            values: zero,
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        self.values
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        let mult_delta = self.mult * delta;
        let src = self.source_provider.values(context);
        for i in 0..self.values.len() {
            let target = src[i];
            self.values[i] = clamp_lerp(self.values[i], target, mult_delta);
        }
    }
}
