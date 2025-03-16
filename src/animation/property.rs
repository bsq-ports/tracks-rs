use std::time::SystemTime;

use crate::{
    point_definition::point_definition_interpolation::PointDefinitionInterpolation,
    values::value::BaseValue,
};



#[derive(Clone, Copy)]
pub struct ValueProperty {
    value: Option<BaseValue>,
    pub last_updated: SystemTime,
}

pub type PathProperty<'a> = PointDefinitionInterpolation<'a>;

impl ValueProperty {
    pub fn mark_updated(&mut self) {
        self.last_updated = SystemTime::now();
    }

    pub fn get_value(&self) -> Option<BaseValue> {
        self.value
    }

    pub fn set_value(&mut self, value: Option<BaseValue>) {
        self.value = value;
        self.mark_updated();
    }
}

impl Default for ValueProperty {
    fn default() -> Self {
        ValueProperty {
            value: None,
            last_updated: SystemTime::now(),
        }
    }
}
