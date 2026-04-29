use glam::{Quat, Vec3};
use smallvec::SmallVec;

use crate::{base_provider_context::BaseProviderContext, base_value::BaseValue, prelude::ValueProvider, quaternion_utils::QuaternionUtilsExt};

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

        // convert to euler angles if the source is a quaternion, 
        // since partial provider only works on vector components and not quaternions directly
        let is_quaternion = matches!(values, BaseValue::Quaternion(_));

        let values = match values {
            BaseValue::Quaternion(q) => q.to_unity_euler_degrees().into(),
            _ => values,
        };

        let v = SmallVec::<[f32; 4]>::from_iter(self.parts.iter().map(|&part| values[part]));
        let result = BaseValue::from_slice(&v, false);

        if is_quaternion {
            // if the source was a quaternion, convert the result back to a quaternion
            let euler_vec = Vec3::new(result[0], result[1], result[2]);
            BaseValue::from(Quat::from_unity_euler_degrees(&euler_vec))
        } else {
            result
        }
    }
}
