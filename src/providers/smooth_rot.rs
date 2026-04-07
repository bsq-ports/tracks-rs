use std::borrow::Cow;

use super::UpdateableValues;

use crate::{base_provider_context::BaseProviderContext, quaternion_utils::QuaternionUtilsExt};

use super::AbstractValueProvider;

use glam::{Quat, Vec3};
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct SmoothRotationProvidersValues {
    pub(crate) rotation_values: Quat,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: Vec3,
}

impl SmoothRotationProvidersValues {
    pub fn new(rotation_values: Quat, mult: f32) -> Self {
        Self {
            rotation_values,
            mult,
            last_quaternion: Quat::IDENTITY,
            values: Default::default(),
        }
    }
}

impl AbstractValueProvider for SmoothRotationProvidersValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        SmallVec::from(self.values.to_array().as_slice())
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self, delta: f32) {
        let mult_delta = delta * self.mult;
        self.last_quaternion = self.last_quaternion.slerp(self.rotation_values, mult_delta);

        let euler = self.last_quaternion.to_unity_euler_degrees();

        self.values = euler;
    }
}
