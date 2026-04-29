use smallvec::smallvec;

use crate::{
    base_provider_context::BaseProviderContext, base_value::BaseValue,
    providers::ValueProviderValues, quaternion_utils::QuaternionUtilsExt,
};

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
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues {
        let value = context.get_values(&self.base);
        match value {
            BaseValue::Float(f) => smallvec![f],
            BaseValue::Vector3(v) => smallvec![v.x, v.y, v.z],
            BaseValue::Vector4(v) => smallvec![v.x, v.y, v.z, v.w],
            // quats are returned as euler angles in degrees, as that's more intuitive to work with for most use cases
            BaseValue::Quaternion(q) => {
                let euler = q.to_unity_euler_degrees();
                smallvec![euler.x, euler.y, euler.z]
            }
        }
    }

    fn is_rotation(&self, context: &BaseProviderContext) -> bool {
        // This is a bit hacky, but it allows the system to know that this provider is providing rotation values without needing an explicit type for it
        // It relies on the convention that quaternions are returned as euler angles in degrees, so if the source value is a quaternion, we consider this a rotation provider
        matches!(context.get_values(&self.base), BaseValue::Quaternion(_))
    }
}
