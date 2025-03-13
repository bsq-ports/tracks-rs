use std::{cell::RefCell, rc::Rc};


use crate::{
    point_definition::{BasePointDefinitionGlobal, PointDefinition},
    values::{base_provider_context::BaseProviderContext, value::BaseValue},
};

pub type ValueProperty = Option<BaseValue>;
pub type ValuePropertyGlobal = Rc<RefCell<ValueProperty>>;

pub type PathPropertyGlobal = Rc<RefCell<PathProperty>>;

#[derive(Default)]
pub struct PathProperty {
    pub time: f32,
    pub prev_point: Option<BasePointDefinitionGlobal>,
    pub point: Option<BasePointDefinitionGlobal>,
}

impl PathProperty {
    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<BasePointDefinitionGlobal>) {
        self.time = 0.0;
        self.prev_point = self.point.take();
        self.point = new_point_data;
    }

    pub fn interpolate(&self, time: f32, context: &BaseProviderContext) -> Option<BaseValue> {
        match (&self.prev_point, &self.point) {
            (Some(prev_point_data), Some(point_data)) => {
                let a = prev_point_data.borrow().interpolate(time, context).0;
                let b = point_data.borrow().interpolate(time, context).0;

                let result = BaseValue::lerp(a, b, self.time);

                Some(result)
            }
            (None, Some(point_data)) => Some(point_data.borrow().interpolate(time, context).0),
            _ => None,
        }
    }
}
