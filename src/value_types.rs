use glam::{Vec3, Vec4};
use smallvec::SmallVec;

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

    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }

    fn to_smallvec(self) -> SmallVec<[f32; Self::VALUE_COUNT]>
    where
        [(); Self::VALUE_COUNT]:,
        [f32; Self::VALUE_COUNT]: smallvec::Array;
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

    fn to_smallvec(self) -> SmallVec<[f32; Self::VALUE_COUNT]>
    where
        [(); Self::VALUE_COUNT]:,
    {
        SmallVec::from([self])
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

    fn to_smallvec(self) -> SmallVec<[f32; Self::VALUE_COUNT]>
    where
        [(); Self::VALUE_COUNT]:,
        [f32; Self::VALUE_COUNT]: smallvec::Array,
    {
        SmallVec::from([self.x, self.y, self.z])
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

    fn to_smallvec(self) -> SmallVec<[f32; Self::VALUE_COUNT]>
    where
        [(); Self::VALUE_COUNT]:,
        [f32; Self::VALUE_COUNT]: smallvec::Array,
    {
        SmallVec::from([self.x, self.y, self.z, self.w])
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

    fn to_smallvec(self) -> SmallVec<[f32; Self::VALUE_COUNT]>
    where
        [(); Self::VALUE_COUNT]:,
        [f32; Self::VALUE_COUNT]: smallvec::Array,
    {
        match self {
            BaseValue::Float(f) => SmallVec::from([f, 0.0, 0.0, 0.0]),
            BaseValue::Vector3(v) => SmallVec::from([v.x, v.y, v.z, 0.0]),
            BaseValue::Vector4(v) => SmallVec::from([v.x, v.y, v.z, v.w]),
            BaseValue::Quaternion(q) => SmallVec::from([q.x, q.y, q.z, q.w]),
        }
    }
}
