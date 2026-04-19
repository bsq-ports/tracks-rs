
use super::UpdateableValues;

use crate::{base_provider_context::BaseProviderContext, quaternion_utils::QuaternionUtilsExt};

use super::AbstractValueProvider;

use glam::{Quat, Vec3};
use log::warn;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct SmoothRotationProvidersValues {
    pub(crate) source_provider: crate::providers::ValueProvider,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: Vec3,
}

impl SmoothRotationProvidersValues {
    pub fn new(source_provider: crate::providers::ValueProvider, mult: f32) -> Self {
        Self {
            source_provider,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: Default::default(),
        }
    }
}

impl AbstractValueProvider for SmoothRotationProvidersValues {
    fn values(&self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        SmallVec::from(self.values.to_array().as_slice())
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        let mult_delta = delta * self.mult;
        let src = self.source_provider.values(context);
        
        // If the source has 4 or more components, interpret as quaternion; otherwise, use identity
        let quat = if src.len() >= 4 {
            Quat::from_xyzw(src[0], src[1], src[2], src[3])
        } else {
            warn!("Source provider for SmoothRotationProvidersValues does not have 4 components; using identity quaternion");
            Quat::IDENTITY
        };

        self.last_quaternion = self.last_quaternion.slerp(quat, mult_delta.clamp(0.0, 1.0));

        let euler = self.last_quaternion.to_unity_euler_degrees();

        self.values = euler;
    }
}
