use glam::FloatExt;
use smallvec::{SmallVec, smallvec};

use super::UpdateableValues;

use crate::{base_provider_context::BaseProviderContext, providers::ValueProviderValues};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct SmoothProvidersValues {
    pub(crate) source_provider: crate::providers::ValueProvider,
    pub(crate) mult: f32,
    pub(crate) values: SmallVec<[f32; 4]>,
}

impl SmoothProvidersValues {
    // Initialize from a source provider; sample initial length from context
    pub fn new(
        source_provider: crate::providers::ValueProvider,
        mult: f32,
        context: &BaseProviderContext,
    ) -> Self {
        let src = source_provider.values(context);
        Self {
            source_provider,
            mult,
            values: smallvec![0.0; src.len()],
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> ValueProviderValues {
        ValueProviderValues::from_slice(self.values.as_slice())
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        let mult_delta = self.mult * delta;
        let mult_delta_clamped = mult_delta.clamp(0.0, 1.0);

        let src: ValueProviderValues = self.source_provider.values(context);
        for (i, value) in self.values.iter_mut().enumerate() {
            if i < src.len() {
                *value = value.lerp(src[i], mult_delta_clamped);
            }
        }
    }
}
