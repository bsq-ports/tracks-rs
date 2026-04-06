use glam::{Vec2, Vec3, Vec4};

use crate::providers::value::BaseValue;

pub trait ValueType:
    Default
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
    const VALUE_COUNT: usize;

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
    fn from_translate_array(values: [f32; Self::VALUE_COUNT]) -> Self {
        unreachable!(
            "from_translate_array should not be called for BaseValue, as it does not have a fixed number of components"
        );
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        BaseValue::Vector4(Vec4::new(values[0], values[1], values[2], values[3]))
    }
}
