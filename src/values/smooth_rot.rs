use std::borrow::Cow;

use super::UpdateableValues;

use crate::{
    base_provider_context::BaseProviderContext, modifiers::quaternion_modifier::TRACKS_EULER_ROT,
};

use super::AbstractValueProvider;

use glam::Quat;

#[derive(Clone, Debug)]
pub struct SmoothRotationProvidersValues {
    pub(crate) rotation_values: Quat,
    pub(crate) mult: f32,
    pub(crate) last_quaternion: Quat,
    pub(crate) values: [f32; 3],
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
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        std::borrow::Cow::Borrowed(&self.values)
    }
}

impl UpdateableValues for SmoothRotationProvidersValues {
    fn update(&mut self, delta: f32) {
        self.last_quaternion = self
            .last_quaternion
            .slerp(self.rotation_values, delta * self.mult);

        let euler = self.last_quaternion.to_euler(TRACKS_EULER_ROT);

        self.values = [
            euler.0.to_degrees(),
            euler.1.to_degrees(),
            euler.2.to_degrees(),
        ];
    }
}
