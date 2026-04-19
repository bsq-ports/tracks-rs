
use smallvec::{SmallVec, smallvec};

use super::{UpdateableValues, clamp_lerp};

use crate::base_provider_context::BaseProviderContext;

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
    fn values(&self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        SmallVec::from(self.values.as_slice())
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        let mult_delta = self.mult * delta;
        let src = self.source_provider.values(context);
        for i in 0..self.values.len() {
            let target = src.get(i).cloned().unwrap_or(0.0);
            self.values[i] = clamp_lerp(self.values[i], target, mult_delta);
        }
    }
}
