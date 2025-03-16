use std::borrow::Cow;

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct StaticValues {
    // TODO: SWITCH TO SMALL VEC
    pub(crate) values: Vec<f32>,
}

impl StaticValues {
    pub fn new(values: Vec<f32>) -> Self {
        Self { values }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        std::borrow::Cow::Borrowed(&self.values)
    }
}
