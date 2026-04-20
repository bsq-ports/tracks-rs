use super::ValueProvider;

use crate::{
    base_provider_context::BaseProviderContext, base_value::BaseValue,
    quaternion_utils::QuaternionUtilsExt,
};

use super::AbstractValueProvider;

use glam::Quat;
use log::warn;

#[derive(Clone, Debug)]
pub struct QuaternionProviderValues {
    pub(crate) source: Box<ValueProvider>,
}

impl QuaternionProviderValues {
    pub fn new(source: ValueProvider) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}

impl AbstractValueProvider for QuaternionProviderValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        let source = self.source.values(_context);

        let source = match source {
            BaseValue::Quaternion(q) => q,
            BaseValue::Vector4(v) => {
                // If the source is a Vector4, interpret as quaternion components
                Quat::from_xyzw(v.x, v.y, v.z, v.w)
            },
            BaseValue::Vector3(vec3) => {
                // If the source is a Vector3, interpret as Euler angles in degrees and convert to quaternion
                Quat::from_unity_euler_degrees(&vec3)
            },
            _ => {
                // If the source is not a quaternion or vector, return identity quaternion
                warn!(
                    "Source provider for QuaternionProviderValues {:?} does not provide a quaternion or vector; using identity quaternion",
                    self.source
                );
                Quat::IDENTITY
            }
        };


        BaseValue::Quaternion(source)
    }
}
