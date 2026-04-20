use super::UpdateableValues;

use crate::{
    base_provider_context::BaseProviderContext, base_value::BaseValue,
    quaternion_utils::QuaternionUtilsExt,
};

use super::AbstractValueProvider;

use glam::{Quat, Vec3};
use log::warn;

#[derive(Clone, Debug)]
pub struct SmoothRotationProvidersValues {
    pub(crate) source_provider: crate::providers::ValueProvider,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: Vec3,
    warned: bool,
}

impl SmoothRotationProvidersValues {
    pub fn new(source_provider: crate::providers::ValueProvider, mult: f32) -> Self {
        Self {
            source_provider,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: Default::default(),
            warned: false,
        }
    }
}

impl AbstractValueProvider for SmoothRotationProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        BaseValue::Vector3(self.values)
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        let mult_delta = delta * self.mult;
        let src = self.source_provider.values(context);

        // If the source has 4 or more components, interpret as quaternion; otherwise, use identity
        let quat = match src {
            BaseValue::Quaternion(q) => q,
            BaseValue::Vector3(v) => Quat::from_unity_euler_degrees(&v),
            _ => {
                if !self.warned {
                    warn!(
                        "Source provider for SmoothRotationProvidersValues {:?} does not provide a quaternion or Vec3; using identity quaternion",
                        self.source_provider
                    );
                    self.warned = true;
                }
                Quat::IDENTITY
            }
        };

        self.last_quaternion = self.last_quaternion.slerp(quat, mult_delta.clamp(0.0, 1.0));

        let euler = self.last_quaternion.to_unity_euler_degrees();

        self.values = euler;
    }
}
