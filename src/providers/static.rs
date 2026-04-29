
use smallvec::SmallVec;

use crate::{base_provider_context::BaseProviderContext, providers::ValueProviderValues};

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
    fn values(&self, _context: &BaseProviderContext) -> ValueProviderValues {
        ValueProviderValues::from_slice(self.values.as_slice())
    }
}
