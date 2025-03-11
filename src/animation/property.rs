use glam::{Quat, Vec3, Vec4};

use crate::{
    point_definition::{BasePointDefinition, PointDefinition},
    values::{base_provider_context::BaseProviderContext, value::BaseValue},
};

pub enum ValueProperty {
    Float(f32),
    Vec3(Vec3),
    Vec4(Vec4),
    Quat(Quat),
    None,
}
impl ValueProperty {
    pub fn set_null(&mut self) {
        *self = ValueProperty::None;
    }

    pub fn update_value(&mut self, value: BaseValue) {
        *self = match value {
            BaseValue::Float(value) => ValueProperty::Float(value),
            BaseValue::Vector3(value) => ValueProperty::Vec3(value),
            BaseValue::Vector4(value) => ValueProperty::Vec4(value),
            BaseValue::Quaternion(value) => ValueProperty::Quat(value),
        };
    }

    pub fn get_value(&self) -> BaseValue {
        match self {
            ValueProperty::Float(value) => BaseValue::Float(*value),
            ValueProperty::Vec3(value) => BaseValue::Vector3(*value),
            ValueProperty::Vec4(value) => BaseValue::Vector4(*value),
            ValueProperty::Quat(value) => BaseValue::Quaternion(*value),
            ValueProperty::None => BaseValue::Float(0.0),
        }
    }
}

pub struct PathProperty {
    pub time: f32,
    pub prev_point: Option<BasePointDefinition>,
    pub point: Option<BasePointDefinition>,
}

impl PathProperty {
    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<BasePointDefinition>) {
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

