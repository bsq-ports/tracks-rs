use smallvec::SmallVec;

use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue, prelude::ValueProvider};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: Box<ValueProvider>,
    pub(crate) parts: SmallVec<[usize; 4]>,
}

impl PartialProviderValues {
    pub fn new(
        source: impl Into<ValueProvider>,
        parts: impl Into<SmallVec<[usize; 4]>>,
    ) -> Self {
        Self {
            source: Box::new(source.into()),
            parts: parts.into(),
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values(&self, context: &BaseProviderContext) -> BaseValue {
        let values = self.source.values(context);
        let v = SmallVec::<[f32; 4]>::from_iter(self.parts.iter().map(|&part| values[part]));
        BaseValue::from_slice(&v, false)
    }
}
