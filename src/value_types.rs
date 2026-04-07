use glam::{Quat, Vec2, Vec3, Vec4};

use crate::{
    base_value::{BaseValue, WrapBaseValueType},
    quaternion_utils::QuaternionUtilsExt,
};

pub trait Lerpable: Sized {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self;
}

pub trait ValueType: Lerpable + Default + Copy + PartialEq + PartialEq<BaseValue> {
    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;
    const VALUE_COUNT: usize;

    fn base_type() -> WrapBaseValueType;

    fn from_translate_slice(values: &[f32]) -> Self;
    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self;

    fn from_slice(values: &[f32]) -> Self;

    fn into_base_value(self) -> BaseValue;
    fn from_base_value(value: BaseValue) -> Option<Self>;
}

pub trait LinearValueType:
    ValueType
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
}

// impl ValueType for  {
//     type Value = BaseValue;
//     const VALUE_COUNT: usize = 4; // Max count among all BaseValue variants
// }

impl Lerpable for f32 {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl ValueType for f32 {
    const VALUE_COUNT: usize = 1;

    fn from_slice(values: &[f32]) -> Self {
        values[0]
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        values[0]
    }

    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        values[0]
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Float
    }

    fn into_base_value(self) -> BaseValue {
        BaseValue::Float(self)
    }
    fn from_base_value(value: BaseValue) -> Option<Self> {
        match value {
            BaseValue::Float(f) => Some(f),
            _ => None,
        }
    }
}
impl LinearValueType for f32 {}

impl Lerpable for Vec2 {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}
impl PartialEq<BaseValue> for f32 {
    fn eq(&self, other: &BaseValue) -> bool {
        match other {
            BaseValue::Float(f) => *self == *f,
            _ => false,
        }
    }
}

impl ValueType for Vec3 {
    const VALUE_COUNT: usize = 3;

    fn from_slice(values: &[f32]) -> Self {
        Vec3::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec3::from_slice(values)
    }

    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        Vec3::from_array(values)
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        unreachable!("Vec3 is not a valid base type for BaseValue")
    }

    fn into_base_value(self) -> BaseValue {
        BaseValue::Vector3(self)
    }

    fn from_base_value(value: BaseValue) -> Option<Self> {
        match value {
            BaseValue::Vector3(v) => Some(v),
            _ => None,
        }
    }
}
impl LinearValueType for Vec3 {}

impl Lerpable for Vec3 {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl Lerpable for Vec4 {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl PartialEq<BaseValue> for Vec3 {
    fn eq(&self, other: &BaseValue) -> bool {
        match other {
            BaseValue::Vector3(v) => *self == *v,
            _ => false,
        }
    }
}

impl ValueType for Vec4 {
    const VALUE_COUNT: usize = 4;

    fn from_slice(values: &[f32]) -> Self {
        Vec4::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec4::from_slice(values)
    }

    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        Vec4::from_array(values)
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Vec4
    }

    fn into_base_value(self) -> BaseValue {
        BaseValue::Vector4(self)
    }

    fn from_base_value(value: BaseValue) -> Option<Self> {
        match value {
            BaseValue::Vector4(v) => Some(v),
            _ => None,
        }
    }
}

impl LinearValueType for Vec4 {}

impl Lerpable for BaseValue {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        match (a, b) {
            (BaseValue::Float(a), BaseValue::Float(b)) => {
                BaseValue::Float(f32::value_lerp(a, b, t))
            }
            (BaseValue::Vector3(a), BaseValue::Vector3(b)) => {
                BaseValue::Vector3(Vec3::value_lerp(a, b, t))
            }
            (BaseValue::Vector4(a), BaseValue::Vector4(b)) => {
                BaseValue::Vector4(Vec4::value_lerp(a, b, t))
            }
            (BaseValue::Quaternion(a), BaseValue::Quaternion(b)) => {
                // For quaternions, we should use slerp instead of lerp for proper interpolation
                BaseValue::Quaternion(Quat::value_lerp(a, b, t))
            }
            _ => panic!("Cannot lerp between different BaseValue variants"),
        }
    }
}

impl PartialEq<BaseValue> for Vec4 {
    fn eq(&self, other: &BaseValue) -> bool {
        match other {
            BaseValue::Vector4(v) => *self == *v,
            _ => false,
        }
    }
}

impl ValueType for BaseValue {
    const VALUE_COUNT: usize = 4;

    fn from_slice(values: &[f32]) -> Self {
        match values.len() {
            1 => BaseValue::Float(values[0]),
            2 => BaseValue::Vector3(Vec3::new(values[0], values[1], 0.0)),
            3 => BaseValue::Vector3(Vec3::new(values[0], values[1], values[2])),
            4 => BaseValue::Vector4(Vec4::new(values[0], values[1], values[2], values[3])),
            _ => panic!("Invalid number of values for BaseValue: {}", values.len()),
        }
    }
    fn from_translate_array(_values: [f32; Self::VALUE_COUNT]) -> Self {
        unreachable!(
            "from_translate_array should not be called for BaseValue, as it does not have a fixed number of components"
        );
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        BaseValue::Vector4(Vec4::new(values[0], values[1], values[2], values[3]))
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Unknown
    }

    fn into_base_value(self) -> BaseValue {
        self
    }

    fn from_base_value(value: BaseValue) -> Option<Self> {
        Some(value)
    }
}

impl ValueType for Quat {
    const VALUE_COUNT: usize = 4;
    // For translation, we only use the Euler angles (x, y, z)

    fn from_slice(values: &[f32]) -> Self {
        Quat::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Quat::from_unity_euler_degrees(&Vec3::from_slice(values))
    }

    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        Quat::from_unity_euler_degrees(&Vec3::from_slice(&values))
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Quat
    }

    fn into_base_value(self) -> BaseValue {
        BaseValue::Quaternion(self)
    }

    fn from_base_value(value: BaseValue) -> Option<Self> {
        match value {
            BaseValue::Quaternion(q) => Some(q),
            _ => None,
        }
    }
}

impl Lerpable for Quat {
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a.slerp(b, t)
    }
}

impl PartialEq<BaseValue> for Quat {
    fn eq(&self, other: &BaseValue) -> bool {
        match other {
            BaseValue::Quaternion(q) => *self == *q,
            _ => false,
        }
    }
}