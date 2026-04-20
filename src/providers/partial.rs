use smallvec::SmallVec;

use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: SmallVec<[f32; 4]>,
    pub(crate) parts: SmallVec<[usize; 4]>,
}

impl PartialProviderValues {
    pub fn new(
        source: impl Into<SmallVec<[f32; 4]>>,
        parts: impl Into<SmallVec<[usize; 4]>>,
    ) -> Self {
        Self {
            source: source.into(),
            parts: parts.into(),
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        let v = SmallVec::<[f32; 4]>::from_iter(self.parts.iter().map(|&part| self.source[part]));
        BaseValue::from_slice(&v, false)
    }
}
