
use smallvec::SmallVec;

use crate::{base_provider_context::BaseProviderContext, providers::ValueProviderValues};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct BaseProviderValues {
    pub(crate) base: String,
}

impl BaseProviderValues {
    pub fn new(base: String) -> Self {
        Self { base }
    }
}

impl AbstractValueProvider for BaseProviderValues {
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues {
        let value = context.get_values(&self.base);
        value.as_small_vec().into_iter().collect()
    }
}
