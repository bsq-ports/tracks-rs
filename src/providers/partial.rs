use smallvec::SmallVec;

use crate::{
    base_provider_context::BaseProviderContext, prelude::ValueProvider,
    types::base_value::BaseValue,
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
    fn values(&self, context: &BaseProviderContext) -> BaseValue {
        let values = self.source.values(context);

        // convert to euler angles if the source is a quaternion,
        // since partial provider only works on vector components and not quaternions directly
        let is_quaternion = matches!(values, BaseValue::Quaternion(_));

        let v = SmallVec::<[f32; 4]>::from_iter(self.parts.iter().map(|&part| values[part]));
        BaseValue::from_slice(&v, is_quaternion)
    }
}
