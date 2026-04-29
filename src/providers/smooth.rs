use super::UpdateableValues;

use crate::{base_provider_context::BaseProviderContext, types::base_value::BaseValue};

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
            BaseValue::Float(_) => BaseValue::Float(0.0),
            BaseValue::Vector2(_) => BaseValue::Vector2(glam::Vec2::ZERO),
            BaseValue::Vector3(_) => BaseValue::Vector3(glam::Vec3::ZERO),
            BaseValue::Vector4(_) => BaseValue::Vector4(glam::Vec4::ZERO),
            BaseValue::Quaternion(_) => BaseValue::Quaternion(glam::Quat::IDENTITY.into()),
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
        self.values = BaseValue::lerp(self.values, src, mult_delta.clamp(0.0, 1.0));
    }
}
