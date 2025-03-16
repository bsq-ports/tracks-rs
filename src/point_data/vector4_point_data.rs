use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        Modifier, ModifierBase,
        operation::Operation,
        vector4_modifier::{Vector4Modifier, Vector4Values},
    },
};
use glam::Vec4;

use super::BasePointData;

pub struct Vector4PointData {
    base_modifier: Vector4Modifier,
    easing: Functions,
    pub hsv_lerp: bool,
    time: f32,
}

impl Vector4PointData {
    pub fn new(
        point: Vector4Values,
        hsv_lerp: bool,
        time: f32,
        modifiers: Vec<Modifier>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: Vector4Modifier::new(point, modifiers, Operation::None),
            easing,
            hsv_lerp,
            time,
        }
    }
}

impl ModifierBase for Vector4PointData {
    type Value = Vec4;
    const VALUE_COUNT: usize = 4;

    fn get_point(&self, context: &BaseProviderContext) -> Vec4 {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> Vec4 {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> Vec4 {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }
}

impl BasePointData<Vec4> for Vector4PointData {
    fn get_easing(&self) -> Functions {
        self.easing
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> Vec4 {
        <Self as ModifierBase>::get_point(self, context)
    }
}
