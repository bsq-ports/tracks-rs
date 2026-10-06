use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        ModifierLike,
        operation::Operation,
        quaternion_modifier::{QuaternionModifier, QuaternionValues},
    },
};
use glam::Quat;

use super::PointDataLike;

#[derive(Debug, Clone)]
pub struct QuaternionPointData {
    base_modifier: QuaternionModifier,
    easing: Functions,
    time: f32,
}

impl QuaternionPointData {
    pub fn new(
        point: QuaternionValues,
        time: f32,
        modifiers: Vec<QuaternionModifier>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: QuaternionModifier::new(point, modifiers, Operation::None),
            easing,
            time,
        }
    }
}

impl PointDataLike<Quat> for QuaternionPointData {
    fn get_easing(&self) -> Functions {
        self.easing
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        self.base_modifier.get_modified_point(context)
    }
}
