use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        Modifier, ModifierBase,
        operation::Operation,
        vector3_modifier::{Vector3Modifier, Vector3Values},
    },
};
use glam::Vec3;

use super::BasePointData;

#[derive(Debug)]
pub struct Vector3PointData {
    base_modifier: Vector3Modifier,
    easing: Functions,
    pub smooth: bool,
    time: f32,
}

impl Vector3PointData {
    pub fn new(
        point: Vector3Values,
        smooth: bool,
        time: f32,
        modifiers: Vec<Modifier>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: Vector3Modifier::new(point, modifiers, Operation::None),
            easing,
            smooth,
            time,
        }
    }
}

impl ModifierBase for Vector3PointData {
    type Value = Vec3;
    const VALUE_COUNT: usize = 3;

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        self.base_modifier.get_point(context)
    }

    fn get_raw_point(&self) -> Vec3 {
        self.base_modifier.get_raw_point()
    }

    fn translate(&self, values: &[f32]) -> Vec3 {
        self.base_modifier.translate(values)
    }

    fn get_operation(&self) -> Operation {
        self.base_modifier.get_operation()
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }
}

impl BasePointData<Vec3> for Vector3PointData {
    fn get_easing(&self) -> Functions {
        self.easing
    }

    fn get_time(&self) -> f32 {
        self.time
    }

    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> Vec3 {
        <Self as ModifierBase>::get_point(self, context)
    }
}
