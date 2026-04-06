use glam::{Vec2, Vec3, Vec4};

use crate::base_value::{BaseValue, WrapBaseValueType};

pub trait ValueType:
    Default
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;
    const VALUE_COUNT: usize;

    fn base_type() -> WrapBaseValueType;

    fn from_translate_slice(values: &[f32]) -> Self;
    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self;

    fn from_slice(values: &[f32]) -> Self;

    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

// impl ValueType for  {
//     type Value = BaseValue;
//     const VALUE_COUNT: usize = 4; // Max count among all BaseValue variants
// }

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
}

impl ValueType for Vec2 {
    const VALUE_COUNT: usize = 2;

    fn from_slice(values: &[f32]) -> Self {
        Vec2::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec2::from_slice(values)
    }

    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        Vec2::from_array(values)
    }

    type Array
        = [f32; Self::VALUE_COUNT]
    where
        [(); Self::VALUE_COUNT]:;

    fn base_type() -> WrapBaseValueType {
        unreachable!("Vec2 is not a valid base type for BaseValue")
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
}
