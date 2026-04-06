use std::borrow::Cow;

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) parts: Vec<usize>,
}

impl PartialProviderValues {
    pub fn new(source: Vec<f32>, parts: Vec<usize>) -> Self {
        Self { source, parts }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let new_values = self
            .parts
            .iter()
            .map(|&part| self.source[part])
            .collect::<Vec<f32>>();
        Cow::Owned(new_values)
    }
}
