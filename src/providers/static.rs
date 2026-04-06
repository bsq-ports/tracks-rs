use std::borrow::Cow;

use smallvec::SmallVec;

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub(crate) values: SmallVec<[f32; 4]>,
}

impl StaticValues {
    pub fn new(values: &[f32]) -> Self {
        Self {
            values: SmallVec::from_slice(values),
        }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        std::borrow::Cow::Borrowed(&self.values)
    }
}
