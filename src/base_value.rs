use glam::FloatExt;
use smallvec::SmallVec;

use std::ops::Add;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::Sub;

use glam::Quat;
use glam::Vec3;

use glam::Vec4;

use crate::quaternion_utils::QuaternionUtilsExt;

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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Hash, Ord, Debug, Default)]
pub enum WrapBaseValueType {
    #[default]
    Unknown = -1,
    Vec3 = 0,
    Quat = 1,
    Vec4 = 2,
    Float = 3,
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
            4.. if quat => BaseValue::Quaternion(Quat::from_slice(value)),
            4.. => BaseValue::Vector4(Vec4::new(value[0], value[1], value[2], value[3])),
            _ => panic!("Invalid value length {}, expected 1 to 4", value.len()),
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

    pub fn len_raw(&self) -> usize {
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

    pub fn as_slice_raw(&self) -> &[f32] {
        match self {
            BaseValue::Float(v) => std::slice::from_ref(v),
            BaseValue::Vector3(v) => v.as_ref(),
            BaseValue::Vector4(v) => v.as_ref(),
            BaseValue::Quaternion(v) => v.as_ref(),
        }
    }

    pub fn into_small_vec_raw(self) -> SmallVec<[f32; 4]> {
        match self {
            BaseValue::Float(v) => smallvec::smallvec![v],
            BaseValue::Vector3(v) => smallvec::smallvec![v.x, v.y, v.z],
            BaseValue::Vector4(v) => smallvec::smallvec![v.x, v.y, v.z, v.w],
            BaseValue::Quaternion(v) => smallvec::smallvec![v.x, v.y, v.z, v.w],
        }
    }

    pub fn into_small_vec_euler(self) -> SmallVec<[f32; 4]> {
        match self {
            BaseValue::Quaternion(v) => {
                let euler = v.to_unity_euler_degrees();
                smallvec::smallvec![euler.x, euler.y, euler.z]
            }
            _ => self.into_small_vec_raw(),
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

impl Add<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn add(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 + v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 + v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 + v2),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // Add or multiply quaternions?

                BaseValue::Quaternion((v1 * v2).normalize())
            }
            _ => panic!("Invalid addition"),
        }
    }
}

impl Sub<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn sub(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 - v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 - v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 - v2),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // Subtract or divide quaternions?

                BaseValue::Quaternion((v1 * v2.inverse()).normalize())
            }
            _ => panic!("Invalid subtraction"),
        }
    }
}

impl Mul<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn mul(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 * v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 * v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 * v2),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // Multiply or slerp quaternions?

                BaseValue::Quaternion((v1 * v2).normalize())
            }
            _ => panic!("Invalid multiplication"),
        }
    }
}

impl Div<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn div(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 / v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 / v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 / v2),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // Divide or slerp quaternions?
                // TODO: Quaternion division is not well defined, we can either do v1 * v2.inverse() or slerp between identity and v1 * v2.inverse() based on the length of v2
                BaseValue::Quaternion((v1 * v2.inverse()).normalize())
            }
            _ => panic!("Invalid division"),
        }
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
