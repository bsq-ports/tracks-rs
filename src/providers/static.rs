use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue};

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct StaticValues {
    pub values: BaseValue,
}

impl StaticValues {
    pub fn new(values: impl Into<BaseValue>) -> Self {
        Self {
            values: values.into(),
        }
    }
}

impl AbstractValueProvider for StaticValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        self.values
    }
}
