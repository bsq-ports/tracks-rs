
use smallvec::SmallVec;

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: SmallVec<[f32; 4]>,
    pub(crate) parts: Vec<usize>,
}

impl PartialProviderValues {
    pub fn new(source: impl Into<SmallVec<[f32; 4]>>, parts: Vec<usize>) -> Self {
        Self { source: source.into(), parts }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        
        self
            .parts
            .iter()
            .map(|&part| self.source[part])
            .collect::<_>()
    }
}
