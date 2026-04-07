use core::panic;
use std::time::SystemTime;

use glam::{Quat, Vec3, Vec4};

use crate::{
    base_value::{BaseValue, WrapBaseValueType},
    point_definition::{
        FloatPointDefinition, Vector4PointDefinition, base_point_definition::BasePointDefinition,
        point_definition_interpolation::{PointDefinitionInterpolation, PointDefinitionInterpolationLike},
        quaternion_point_definition::QuaternionPointDefinition,
        vector3_point_definition::Vector3PointDefinition,
    },
    value_types::ValueType,
};

// pub type ValuePropertyCell = Cell<ValueProperty>;
// pub type PathPropertyCell<'a> = Cell<PointDefinitionInterpolation<'a>>;

// pub type ValuePropertyShared = Rc<ValuePropertyCell>;
// pub type PathPropertyShared<'a> = Rc<PathPropertyCell<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct ValueProperty<T: ValueType> {
    value: Option<T>,
    pub last_updated: SystemTime,
}

pub type Vec3ValueProperty = ValueProperty<Vec3>;
pub type Vec4ValueProperty = ValueProperty<Vec4>;
pub type FloatValueProperty = ValueProperty<f32>;
pub type QuatValueProperty = ValueProperty<Quat>;
pub type BaseValueProperty = ValueProperty<BaseValue>;

pub type PathProperty<T, V> = PointDefinitionInterpolation<T, V>;
pub type PathPropertyLike = dyn PointDefinitionInterpolationLike;

pub type Vec3PathProperty = PathProperty<Vector3PointDefinition, Vec3>;
pub type Vec4PathProperty = PathProperty<Vector4PointDefinition, Vec4>;
pub type FloatPathProperty = PathProperty<FloatPointDefinition, f32>;
pub type QuatPathProperty = PathProperty<QuaternionPointDefinition, Quat>;
pub type BasePathProperty = PathProperty<BasePointDefinition, BaseValue>;

pub trait ValuePropertyLike {
    fn get_type(&self) -> WrapBaseValueType;
    fn get_base_value(&self) -> Option<BaseValue>;
    fn set_base_value(&mut self, value: Option<BaseValue>);
    fn updated(&self) -> SystemTime;

    fn copy_from(&mut self, other: &dyn ValuePropertyLike);
}

impl<T> ValueProperty<T>
where
    T: ValueType + PartialEq,
{
    pub fn new(value: Option<T>) -> Self {
        ValueProperty {
            value,
            last_updated: SystemTime::now(),
        }
    }

    pub fn empty() -> Self {
        ValueProperty {
            value: None,
            last_updated: SystemTime::now(),
        }
    }

    pub fn mark_updated(&mut self) {
        self.last_updated = SystemTime::now();
    }

    pub fn get_value(&self) -> Option<T> {
        self.value
    }

    pub fn set_value(&mut self, value: Option<T>) {
        let modified = self.value != value;
        self.value = value;
        if modified {
            self.mark_updated();
        }
    }
}

impl<V> ValuePropertyLike for ValueProperty<V>
where
    V: ValueType,
{
    fn get_type(&self) -> WrapBaseValueType {
        V::base_type()
    }

    fn get_base_value(&self) -> Option<BaseValue> {
        self.value.clone().map(|v| v.into_base_value())
    }

    fn set_base_value(&mut self, value: Option<BaseValue>) {
        let converted = value.and_then(V::from_base_value);
        let modified = self.value != converted;
        self.value = converted;
        if modified {
            self.mark_updated();
        }
    }

    fn updated(&self) -> SystemTime {
        self.last_updated
    }
    
    fn copy_from(&mut self, other: &dyn ValuePropertyLike) {
        if self.get_type() != other.get_type() {
            panic!("Cannot copy from a ValueProperty {} of a different type {}", self.get_type(), other.get_type());
        }
        let value = other.get_base_value();
        self.set_base_value(value);
    }
}

impl<T> Default for ValueProperty<T>
where
    T: ValueType,
{
    fn default() -> Self {
        ValueProperty {
            value: None,
            last_updated: SystemTime::now(),
        }
    }
}
