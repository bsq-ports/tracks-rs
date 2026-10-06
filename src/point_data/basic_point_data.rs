use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        ModifierLike, ModifierValues, basic_modifier::BasicModifier, operation::Operation,
    },
    value_types::ValueType,
};

use super::PointDataLike;

#[derive(Debug, Clone)]
pub struct BasicPointData<T: ValueType> {
    base_modifier: BasicModifier<T>,
    pub smooth: bool,
    /// Heck's `lerpHSV` flag: interpolate into this point in HSV. Only affects colours.
    pub hsv_lerp: bool,
    easing: Functions,
    time: f32,
}

impl<T: ValueType> BasicPointData<T> {
    pub fn new(
        point: ModifierValues<T>,
        time: f32,
        smooth: bool,
        modifiers: Vec<BasicModifier<T>>,
        easing: Functions,
    ) -> Self {
        Self {
            base_modifier: BasicModifier::new(point, modifiers, Operation::None),
            smooth,
            hsv_lerp: false,
            easing,
            time,
        }
    }

    /// Sets Heck's `lerpHSV` flag.
    pub fn with_hsv_lerp(mut self, hsv_lerp: bool) -> Self {
        self.hsv_lerp = hsv_lerp;
        self
    }
}

impl<T: ValueType> PointDataLike<T> for BasicPointData<T> {
    fn get_easing(&self) -> Functions {
        self.easing
    }

    fn get_time(&self) -> f32 {
        self.time
    }
    fn has_base_provider(&self) -> bool {
        self.base_modifier.has_base_provider()
    }

    fn get_point(&self, context: &BaseProviderContext) -> T {
        self.base_modifier.get_modified_point(context)
    }
}
