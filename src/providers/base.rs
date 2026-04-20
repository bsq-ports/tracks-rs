use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue};

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
    fn values(&self, context: &BaseProviderContext) -> BaseValue {
        context.get_values(&self.base)
    }
}
