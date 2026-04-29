use super::ValueProvider;

use crate::{
    base_provider_context::BaseProviderContext, providers::ValueProviderValues, quaternion_utils::QuaternionUtilsExt, value_types::ValueType
};

use super::AbstractValueProvider;

use glam::Quat;
use log::warn;
use smallvec::SmallVec;

/// Converts a quaternion source provider into euler angles in degrees, using the same convention as Unity (ZXY(Ex) order).
/// Receives a quaternion from the source provider and outputs the corresponding euler angles in degrees. If the source provider returns less than 4 values, it will log a warning and return a default rotation of (0, 0, 0) degrees.
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
    fn values(&self, _context: &BaseProviderContext) -> ValueProviderValues {
        let source = self.source.values(_context);
        if source.len() < 4 {
            warn!(
                "QuaternionProviderValues: Source provider returned less than 4 values, returning default rotation. Source values: {:?}",
                source
            );
            return ValueProviderValues::from_slice(&[0.0, 0.0, 0.0]);
        }
        let rotation = Quat::from_xyzw(source[0], source[1], source[2], source[3]);
        let euler = rotation.to_unity_euler_degrees();

        ValueProviderValues::from_slice(&[euler.x, euler.y, euler.z])
    }
}
