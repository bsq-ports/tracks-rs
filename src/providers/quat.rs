use super::ValueProvider;

use crate::{
    base_provider_context::BaseProviderContext, base_value::BaseValue,
    quaternion_utils::QuaternionUtilsExt,
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
    fn values(&self, _context: &BaseProviderContext) -> BaseValue {
        let source = self.source.values(_context);

        // We are receiving quaternion values directly here
        // TODO: Verify this!
        let rotation = Quat::from_xyzw(source[0], source[1], source[2], source[3]);
        let euler = rotation.to_unity_euler_degrees();

        BaseValue::Vector3(euler)
    }
}
