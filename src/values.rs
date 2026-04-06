use glam::{Quat, Vec2, Vec3, Vec4};

use crate::{providers::value::BaseValue, quaternion_utils::QuaternionUtilsExt};

pub trait ValueType:
    Default
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
    const VALUE_COUNT: usize;

    fn from_translate_slice(values: &[f32]) -> Self;
}

// impl ValueType for  {
//     type Value = BaseValue;
//     const VALUE_COUNT: usize = 4; // Max count among all BaseValue variants
// }

impl ValueType for f32 {
    const VALUE_COUNT: usize = 1;

    fn from_translate_slice(values: &[f32]) -> Self {
        values[0]
    }
}

impl ValueType for Vec2 {
    const VALUE_COUNT: usize = 2;

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec2::new(values[0], values[1])
    }
}

impl ValueType for Vec3 {
    const VALUE_COUNT: usize = 3;

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec3::new(values[0], values[1], values[2])
    }
}

impl ValueType for Vec4 {
    const VALUE_COUNT: usize = 4;

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec4::new(values[0], values[1], values[2], values[3])
    }
}


impl ValueType for BaseValue {
    const VALUE_COUNT: usize = 4;

    fn from_translate_slice(values: &[f32]) -> Self {
        BaseValue::Vector4(Vec4::new(values[0], values[1], values[2], values[3]))
    }
}