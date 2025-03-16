use std::borrow::Cow;

use tracing::info;

use crate::base_provider_context::BaseProviderContext;

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
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let value = context.get_values(&self.base);
        value.as_slice().to_vec().into()
    }
}
