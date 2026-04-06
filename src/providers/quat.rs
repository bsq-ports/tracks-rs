use std::borrow::Cow;

use super::ValueProvider;

use crate::{
    base_provider_context::BaseProviderContext,
    quaternion_utils::{QuaternionUtilsExt, TRACKS_EULER_ROT},
};

use super::AbstractValueProvider;

use glam::Quat;

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
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let source = self.source.values(_context);

        // We are receiving quaternion values directly here
        // TODO: Verify this!
        let rotation = Quat::from_xyzw(source[0], source[1], source[2], source[3]);
        let euler = rotation.to_unity_euler_degrees();

        euler.to_array().to_vec().into()
    }
}
