
use smallvec::{SmallVec, smallvec};

use super::{UpdateableValues, clamp_lerp};

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct SmoothProvidersValues {
    pub(crate) source: SmallVec<[f32; 4]>,
    pub(crate) mult: f32,
    pub(crate) values: SmallVec<[f32; 4]>,
}

impl SmoothProvidersValues {
    // TODO: use a Vec4?
    pub fn new(source: impl Into<SmallVec<[f32; 4]>>, mult: f32) -> Self {
        let source = source.into();
        Self {
            mult,
            values: smallvec![0.0; source.len()],
            source,
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        SmallVec::from(self.values.as_slice())
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32) {
        let mult_delta = self.mult * delta;
        for i in 0..self.source.len() {
            self.values[i] = clamp_lerp(self.values[i], self.source[i], mult_delta);
        }
    }
}
