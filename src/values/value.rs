use glam::FloatExt;

use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

use glam::Quat;
use glam::Vec3;

use glam::Vec4;

use crate::ffi::types::WrapBaseValueType;

///
/// Time based number
///
#[repr(transparent)]
#[derive(Clone, Debug, Copy)]
pub struct TimeValue(f32);

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum BaseValue {
    Float(f32),
    Vector3(Vec3),
    Vector4(Vec4),
    Quaternion(Quat),
}

impl Default for BaseValue {
    fn default() -> Self {
        BaseValue::Float(0.0)
    }
}

#[derive(Clone, Debug, Copy)]
pub enum BaseValueRef<'a> {
    Float(&'a f32),
    Vector3(&'a Vec3),
    Vector4(&'a Vec4),
    Quaternion(&'a Quat),
}

impl BaseValue {
    #[inline(always)]
    pub fn from_vec(value: Vec<f32>, quat: bool) -> BaseValue {
        Self::from_slice(value.as_slice(), quat)
    }

    pub fn from_slice(value: &[f32], quat: bool) -> BaseValue {
        match value.len() {
            1 => BaseValue::Float(value[0]),
            3 => BaseValue::Vector3(Vec3::new(value[0], value[1], value[2])),
            4 if quat => BaseValue::Quaternion(Quat::from_slice(value)),
            4 => BaseValue::Vector4(Vec4::new(value[0], value[1], value[2], value[3])),
            _ => panic!("Invalid value length"),
        }
    }
    pub fn as_float(&self) -> Option<f32> {
        match self {
            BaseValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<Vec3> {
        match self {
            BaseValue::Vector3(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec4(&self) -> Option<Vec4> {
        match self {
            BaseValue::Vector4(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_quat(&self) -> Option<Quat> {
        match self {
            BaseValue::Quaternion(v) => Some(*v),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            BaseValue::Float(_) => 1,
            BaseValue::Vector3(_) => 3,
            BaseValue::Vector4(_) => 4,
            BaseValue::Quaternion(_) => 4,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn as_slice(&self) -> &[f32] {
        match self {
            BaseValue::Float(v) => std::slice::from_ref(v),
            BaseValue::Vector3(v) => v.as_ref(),
            BaseValue::Vector4(v) => v.as_ref(),
            BaseValue::Quaternion(v) => v.as_ref(),
        }
    }

    pub fn lerp(a: BaseValue, b: BaseValue, t: f32) -> BaseValue {
        match (a, b) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => f32::lerp(v1, v2, t).into(),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => Vec3::lerp(v1, v2, t).into(),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => Vec4::lerp(v1, v2, t).into(),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // lerp or slerp?

                Quat::slerp(v1, v2, t).into()
            }
            _ => panic!("Invalid interpolation"),
        }
    }

    pub fn get_type(&self) -> WrapBaseValueType {
        match self {
            BaseValue::Float(_) => WrapBaseValueType::Float,
            BaseValue::Vector3(_) => WrapBaseValueType::Vec3,
            BaseValue::Vector4(_) => WrapBaseValueType::Vec4,
            BaseValue::Quaternion(_) => WrapBaseValueType::Quat,
        }
    }
}

impl BaseValueRef<'_> {
    pub fn as_float(&self) -> Option<&f32> {
        match self {
            BaseValueRef::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<&Vec3> {
        match self {
            BaseValueRef::Vector3(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_vec4(&self) -> Option<&Vec4> {
        match self {
            BaseValueRef::Vector4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_quat(&self) -> Option<&Quat> {
        match self {
            BaseValueRef::Quaternion(v) => Some(v),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            BaseValueRef::Float(_) => 1,
            BaseValueRef::Vector3(_) => 3,
            BaseValueRef::Vector4(_) => 4,
            BaseValueRef::Quaternion(_) => 4,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn as_slice(&self) -> &[f32] {
        match self {
            BaseValueRef::Float(v) => std::slice::from_ref(v),
            BaseValueRef::Vector3(v) => v.as_ref(),
            BaseValueRef::Vector4(v) => v.as_ref(),
            BaseValueRef::Quaternion(v) => v.as_ref(),
        }
    }
}

impl From<f32> for BaseValue {
    fn from(v: f32) -> Self {
        BaseValue::Float(v)
    }
}

impl From<Vec3> for BaseValue {
    fn from(v: Vec3) -> Self {
        BaseValue::Vector3(v)
    }
}

impl From<Vec4> for BaseValue {
    fn from(v: Vec4) -> Self {
        BaseValue::Vector4(v)
    }
}

impl From<Quat> for BaseValue {
    fn from(v: Quat) -> Self {
        BaseValue::Quaternion(v)
    }
}

// scalar ops

impl Mul<f32> for BaseValue {
    type Output = BaseValue;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            BaseValue::Float(v) => BaseValue::Float(v * rhs),
            BaseValue::Vector3(v) => BaseValue::Vector3(v * rhs),
            BaseValue::Vector4(v) => BaseValue::Vector4(v * rhs),
            BaseValue::Quaternion(v) => BaseValue::Quaternion(v * rhs),
        }
    }
}

impl Div<f32> for BaseValue {
    type Output = BaseValue;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            BaseValue::Float(v) => BaseValue::Float(v / rhs),
            BaseValue::Vector3(v) => BaseValue::Vector3(v / rhs),
            BaseValue::Vector4(v) => BaseValue::Vector4(v / rhs),
            BaseValue::Quaternion(v) => BaseValue::Quaternion(v / rhs),
        }
    }
}

impl Index<usize> for BaseValue {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            BaseValue::Float(f) => f,
            BaseValue::Vector3(v) => &v[index],
            BaseValue::Vector4(v) => &v[index],
            BaseValue::Quaternion(v) => match index {
                0 => &v.x,
                1 => &v.y,
                2 => &v.z,
                3 => &v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
}
impl IndexMut<usize> for BaseValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            BaseValue::Float(f) => f,
            BaseValue::Vector3(v) => &mut v[index],
            BaseValue::Vector4(v) => &mut v[index],
            BaseValue::Quaternion(v) => match index {
                0 => &mut v.x,
                1 => &mut v.y,
                2 => &mut v.z,
                3 => &mut v.w,
                _ => panic!("Invalid index for Quaternion"),
            },
        }
    }
}

impl IntoIterator for BaseValue {
    type Item = f32;
    type IntoIter = Box<dyn Iterator<Item = f32>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            BaseValue::Float(v) => Box::new([v].into_iter()),
            BaseValue::Vector3(v) => Box::new([v.x, v.y, v.z].into_iter()),
            BaseValue::Vector4(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
            BaseValue::Quaternion(v) => Box::new([v.x, v.y, v.z, v.w].into_iter()),
        }
    }
}

impl<'a> From<&'a BaseValue> for BaseValueRef<'a> {
    fn from(v: &'a BaseValue) -> Self {
        match v {
            BaseValue::Float(v) => BaseValueRef::Float(v),
            BaseValue::Vector3(v) => BaseValueRef::Vector3(v),
            BaseValue::Vector4(v) => BaseValueRef::Vector4(v),
            BaseValue::Quaternion(v) => BaseValueRef::Quaternion(v),
        }
    }
}
impl<'a> From<&'a f32> for BaseValueRef<'a> {
    fn from(v: &'a f32) -> Self {
        BaseValueRef::Float(v)
    }
}
impl<'a> From<&'a Vec3> for BaseValueRef<'a> {
    fn from(v: &'a Vec3) -> Self {
        BaseValueRef::Vector3(v)
    }
}
impl<'a> From<&'a Vec4> for BaseValueRef<'a> {
    fn from(v: &'a Vec4) -> Self {
        BaseValueRef::Vector4(v)
    }
}
impl<'a> From<&'a Quat> for BaseValueRef<'a> {
    fn from(v: &'a Quat) -> Self {
        BaseValueRef::Quaternion(v)
    }
}
