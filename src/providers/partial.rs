use smallvec::SmallVec;

use crate::{
    base_provider_context::BaseProviderContext, prelude::ValueProvider,
    providers::ValueProviderValues,
};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: Box<ValueProvider>,
    pub(crate) parts: SmallVec<[usize; 4]>,
}

impl PartialProviderValues {
    pub fn new(source: impl Into<ValueProvider>, parts: impl Into<SmallVec<[usize; 4]>>) -> Self {
        Self {
            source: Box::new(source.into()),
            parts: parts.into(),
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues {
        let values = self.source.values(context);

        ValueProviderValues::from_iter(self.parts.iter().map(|&part| values[part]))
    }
}
