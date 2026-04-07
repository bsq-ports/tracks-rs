use std::borrow::Cow;

use smallvec::SmallVec;

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub values: SmallVec<[f32; 4]>,
}

impl StaticValues {
    pub fn new(values: impl Into<SmallVec<[f32; 4]>>) -> Self {
        Self {
            values: values.into(),
        }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        self.values.clone()
    }
}
