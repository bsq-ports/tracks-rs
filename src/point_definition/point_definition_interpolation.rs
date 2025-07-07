use crate::{
    base_provider_context::BaseProviderContext, ffi::types::WrapBaseValueType,
    values::value::BaseValue,
};

use super::{PointDefinition, base_point_definition::BasePointDefinition};

#[derive(Default, Clone, Debug)]
pub struct PointDefinitionInterpolation<'a> {
    pub time: f32,
    // use refs here to avoid mass cloning
    pub prev_point: Option<&'a BasePointDefinition>,
    pub point: Option<&'a BasePointDefinition>,
    ty: WrapBaseValueType,
}

impl<'a> PointDefinitionInterpolation<'a> {
    pub fn new(point: Option<&'a BasePointDefinition>, ty: WrapBaseValueType) -> Self {
        PointDefinitionInterpolation {
            time: 0.0,
            prev_point: None,
            point,
            ty,
        }
    }

    pub fn empty(ty: WrapBaseValueType) -> Self {
        PointDefinitionInterpolation {
            time: 0.0,
            prev_point: None,
            point: None,
            ty,
        }
    }

    pub fn get_type(&self) -> WrapBaseValueType {
        self.ty
    }

    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<&'a BasePointDefinition>) {
        self.time = 0.0;
        self.prev_point = self.point.take();
        self.point = new_point_data;

        if let Some(point_data) = &self.point {
            assert!(
                point_data.get_type() == self.ty,
                "PointDefinitionInterpolation type mismatch: expected {:?}, got {:?}",
                self.ty,
                point_data.get_type()
            );
        }
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
