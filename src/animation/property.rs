use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    time::SystemTime,
};

use crate::{
    ffi::types::WrapBaseValueType,
    point_definition::point_definition_interpolation::PointDefinitionInterpolation,
    values::value::BaseValue,
};

// pub type ValuePropertyCell = Cell<ValueProperty>;
// pub type PathPropertyCell<'a> = Cell<PointDefinitionInterpolation<'a>>;

// pub type ValuePropertyShared = Rc<ValuePropertyCell>;
// pub type PathPropertyShared<'a> = Rc<PathPropertyCell<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueProperty {
    value: Option<BaseValue>,
    ty: WrapBaseValueType, // This field is used to store the type of the value
    pub last_updated: SystemTime,
}

pub type PathProperty<'a> = PointDefinitionInterpolation<'a>;

impl ValueProperty {
    pub fn new(value: Option<BaseValue>, ty: WrapBaseValueType) -> Self {
        if let Some(ref v) = value {
            assert!(
                v.get_type() == ty,
                "Value type mismatch: expected {:?}, got {:?}",
                ty,
                v.get_type()
            );
        }
        ValueProperty {
            value,
            ty,
            last_updated: SystemTime::now(),
        }
    }

    pub fn empty(ty: WrapBaseValueType) -> Self {
        ValueProperty {
            value: None,
            ty,
            last_updated: SystemTime::now(),
        }
    }

    pub fn mark_updated(&mut self) {
        self.last_updated = SystemTime::now();
    }

    pub fn get_value(&self) -> Option<BaseValue> {
        self.value
    }

    pub fn get_type(&self) -> WrapBaseValueType {
        self.ty
    }

    pub fn set_value(&mut self, value: Option<BaseValue>) {
        if let Some(ref v) = value
            && self.ty != WrapBaseValueType::Unknown
        {
            assert!(
                v.get_type() == self.ty,
                "Value type mismatch: expected {:?}, got {:?}",
                self.ty,
                v.get_type()
            );
        }
        self.value = value;
        self.mark_updated();
    }
}

impl Default for ValueProperty {
    fn default() -> Self {
        ValueProperty {
            value: None,
            ty: WrapBaseValueType::Unknown, // Default to Unknown type
            last_updated: SystemTime::now(),
        }
    }
}
