use crate::{base_provider_context::BaseProviderContext, values::value::BaseValue};

use super::{PointDefinition, base_point_definition::BasePointDefinition};

#[derive(Default, Clone)]
pub struct PointDefinitionInterpolation<'a> {
    pub time: f32,
    pub prev_point: Option<&'a BasePointDefinition>,
    pub point: Option<&'a BasePointDefinition>,
}

impl<'a> PointDefinitionInterpolation<'a> {
    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<&'a BasePointDefinition>) {
        self.time = 0.0;
        self.prev_point = self.point.take();
        self.point = new_point_data;
    }

    pub fn interpolate(&self, time: f32, context: &BaseProviderContext) -> Option<BaseValue> {
        match (&self.prev_point, &self.point) {
            (Some(prev_point_data), Some(point_data)) => {
                let a = prev_point_data.interpolate(time, context).0;
                let b = point_data.interpolate(time, context).0;

                let result = BaseValue::lerp(a, b, self.time);

                Some(result)
            }
            (None, Some(point_data)) => Some(point_data.interpolate(time, context).0),
            _ => None,
        }
    }
}
