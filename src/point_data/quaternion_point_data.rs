use crate::{
    easings::functions::Functions,
    modifiers::{
        Modifier, ModifierBase,
        operation::Operation,
        quaternion_modifier::{QuaternionModifier, QuaternionValues},
    },
    base_provider_context::BaseProviderContext,
};
use glam::Quat;

use super::BasePointData;

pub struct QuaternionPointData {
    base_modifier: QuaternionModifier,
    easing: Functions,
    time: f32,
}

impl QuaternionPointData {
    pub fn new(
        point: QuaternionValues,
        time: f32,
        modifiers: Vec<Modifier>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: QuaternionModifier::new(point, modifiers, Operation::None),
            easing,
            time,
        }
    }
}

impl ModifierBase for QuaternionPointData {
    type Value = Quat;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> Quat {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> Quat {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }
}

impl BasePointData<Quat> for QuaternionPointData {
    fn get_easing(&self) -> Functions {
        self.easing.clone()
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> Quat {
        <Self as ModifierBase>::get_point(self, context)
    }
}
