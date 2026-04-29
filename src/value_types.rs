use glam::FloatExt;
use glam::{Quat, Vec3, Vec4};

use crate::base_value::{BaseValue, WrapBaseValueType};

/// Represents a type that can be used as a value in the system, such as a float, vector, or quaternion.
/// This trait defines the necessary operations and conversions for these types, allowing them to be used
/// interchangeably in the animation system.
///
/// This is what allows us to have a unified way of handling different types of values (like float, vec3, vec4) in the system,
/// and to perform operations like interpolation, addition, etc. on them without needing to know the specific type
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

    #[inline]
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }

    #[inline]
    fn value_lerp_clamped(a: Self, b: Self, t: f32) -> Self {
        Self::value_lerp(a, b, t.clamp(0.0, 1.0))
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