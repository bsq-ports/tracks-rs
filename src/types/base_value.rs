use glam::FloatExt;
use smallvec::SmallVec;

use std::ops::Add;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Div;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;
use std::ops::Sub;

use glam::Quat;
use glam::Vec2;
use glam::Vec3;

use glam::Vec4;

use super::quaternion_utils::QuaternionUtilsExt;

///
/// Time based number
///
#[repr(transparent)]
#[derive(Clone, Debug, Copy)]
pub struct TimeValue(f32);

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum BaseValue {
    Float(f32),
    Vector2(Vec2),
    Vector3(Vec3),
    Vector4(Vec4),
    // Store quaternion as euler angles in degrees for easier interpolation and editing, convert to quaternion when needed
    // most of the math assumes euler angles, and it's easier to work with them directly for things like partial providers and value providers that operate on components
    // slerp/lerp is done on the quaternion representation
    Quaternion(EulerVec3),
}

#[repr(transparent)]
#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct EulerVec3(pub Vec3);

impl Deref for EulerVec3 {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EulerVec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for BaseValue {
    fn default() -> Self {
        BaseValue::Float(0.0)
    }
}
impl From<Quat> for EulerVec3 {
    fn from(q: Quat) -> Self {
        EulerVec3(q.to_unity_euler_degrees())
    }
}
impl EulerVec3 {
    pub const IDENTITY: Self = Self(Vec3::ZERO);

    pub fn to_quat(&self) -> Quat {
        Quat::from_unity_euler_degrees(&self.0)
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl From<BaseValue> for f32 {
    fn from(v: BaseValue) -> Self {
        let slice = v.as_slice();
        slice.first().copied().unwrap_or(0.0)
    }
}

impl From<BaseValue> for Vec3 {
    fn from(v: BaseValue) -> Self {
        let slice = v.as_slice();
        let x = *slice.first().unwrap_or(&0.0);
        let y = *slice.get(1).unwrap_or(&0.0);
        let z = *slice.get(2).unwrap_or(&0.0);
        Vec3::new(x, y, z)
    }
}

impl From<BaseValue> for Vec4 {
    fn from(v: BaseValue) -> Self {
        let slice = v.as_slice();
        let x = *slice.first().unwrap_or(&0.0);
        let y = *slice.get(1).unwrap_or(&0.0);
        let z = *slice.get(2).unwrap_or(&0.0);
        let w = *slice.get(3).unwrap_or(&0.0);
        Vec4::new(x, y, z, w)
    }
}

impl From<BaseValue> for Vec2 {
    fn from(v: BaseValue) -> Self {
        let slice = v.as_slice();
        let x = *slice.first().unwrap_or(&0.0);
        let y = *slice.get(1).unwrap_or(&0.0);
        Vec2::new(x, y)
    }
}

impl From<EulerVec3> for BaseValue {
    fn from(v: EulerVec3) -> Self {
        BaseValue::Quaternion(v)
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
    Vec2 = 4,
}

impl BaseValue {
    #[inline(always)]
    pub fn from_vec(value: Vec<f32>, quat: bool) -> BaseValue {
        Self::from_slice(value.as_slice(), quat)
    }

    pub fn from_slice(value: &[f32], quat: bool) -> BaseValue {
        match value.len() {
            1 => BaseValue::Float(value[0]),
            2 => BaseValue::Vector2(Vec2::new(value[0], value[1])),
            3 => BaseValue::Vector3(Vec3::new(value[0], value[1], value[2])),
            3.. if quat => {
                BaseValue::Quaternion(EulerVec3(Vec3::new(value[0], value[1], value[2])))
            }
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

    pub fn as_vec2(&self) -> Option<Vec2> {
        match self {
            BaseValue::Vector2(v) => Some(*v),
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
            BaseValue::Quaternion(v) => Some(Quat::from_unity_euler_degrees(&v.0)),
            _ => None,
        }
    }

    pub fn as_euler_vec3(&self) -> Option<EulerVec3> {
        match self {
            BaseValue::Quaternion(v) => Some(*v),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            BaseValue::Float(_) => 1,
            BaseValue::Vector2(_) => 2,
            BaseValue::Vector3(_) => 3,
            BaseValue::Vector4(_) => 4,
            // Quaternion is stored as EulerVec3 internally (3 components)
            BaseValue::Quaternion(_) => 3,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn as_slice(&self) -> &[f32] {
        match self {
            BaseValue::Float(v) => std::slice::from_ref(v),
            BaseValue::Vector2(v) => v.as_ref(),
            BaseValue::Vector3(v) => v.as_ref(),
            BaseValue::Vector4(v) => v.as_ref(),
            BaseValue::Quaternion(v) => v.as_ref(),
        }
    }

    pub fn into_small_vec(self) -> SmallVec<[f32; 4]> {
        match self {
            BaseValue::Float(v) => smallvec::smallvec![v],
            BaseValue::Vector2(v) => smallvec::smallvec![v.x, v.y],
            BaseValue::Vector3(v) => smallvec::smallvec![v.x, v.y, v.z],
            BaseValue::Vector4(v) => smallvec::smallvec![v.x, v.y, v.z, v.w],
            BaseValue::Quaternion(v) => smallvec::smallvec![v.x, v.y, v.z],
        }
    }

    pub fn lerp(a: BaseValue, b: BaseValue, t: f32) -> BaseValue {
        match (a, b) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => f32::lerp(v1, v2, t).into(),
            (BaseValue::Vector2(v1), BaseValue::Vector2(v2)) => Vec2::lerp(v1, v2, t).into(),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => Vec3::lerp(v1, v2, t).into(),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => Vec4::lerp(v1, v2, t).into(),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // lerp or slerp?

                Quat::slerp(v1.to_quat(), v2.to_quat(), t).into()
            }
            _ => panic!("Invalid interpolation"),
        }
    }

    pub fn get_type(&self) -> WrapBaseValueType {
        match self {
            BaseValue::Float(_) => WrapBaseValueType::Float,
            BaseValue::Vector2(_) => WrapBaseValueType::Vec2,
            BaseValue::Vector3(_) => WrapBaseValueType::Vec3,
            BaseValue::Vector4(_) => WrapBaseValueType::Vec4,
            BaseValue::Quaternion(_) => WrapBaseValueType::Quat,
        }
    }
}

impl From<BaseValue> for SmallVec<[f32; 4]> {
    fn from(v: BaseValue) -> Self {
        v.into_small_vec()
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

impl From<Vec2> for BaseValue {
    fn from(v: Vec2) -> Self {
        BaseValue::Vector2(v)
    }
}

impl From<Quat> for BaseValue {
    fn from(v: Quat) -> Self {
        BaseValue::Quaternion(EulerVec3(v.to_unity_euler_degrees()))
    }
}

impl Add<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn add(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 + v2),
            (BaseValue::Vector2(v1), BaseValue::Vector2(v2)) => BaseValue::Vector2(v1 + v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 + v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 + v2),
            (BaseValue::Quaternion(v1), BaseValue::Quaternion(v2)) => {
                // Add or multiply quaternions?
                let result = v1.to_quat() * v2.to_quat();

                BaseValue::Quaternion(EulerVec3(result.to_unity_euler_degrees()))
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
            (BaseValue::Vector2(v1), BaseValue::Vector2(v2)) => BaseValue::Vector2(v1 - v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 - v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 - v2),
            _ => panic!("Invalid subtraction"),
        }
    }
}

impl Mul<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn mul(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 * v2),
            (BaseValue::Vector2(v1), BaseValue::Vector2(v2)) => BaseValue::Vector2(v1 * v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 * v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 * v2),
            _ => panic!("Invalid multiplication"),
        }
    }
}

impl Div<BaseValue> for BaseValue {
    type Output = BaseValue;

    fn div(self, rhs: BaseValue) -> Self::Output {
        match (self, rhs) {
            (BaseValue::Float(v1), BaseValue::Float(v2)) => BaseValue::Float(v1 / v2),
            (BaseValue::Vector2(v1), BaseValue::Vector2(v2)) => BaseValue::Vector2(v1 / v2),
            (BaseValue::Vector3(v1), BaseValue::Vector3(v2)) => BaseValue::Vector3(v1 / v2),
            (BaseValue::Vector4(v1), BaseValue::Vector4(v2)) => BaseValue::Vector4(v1 / v2),
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
            BaseValue::Vector2(v) => BaseValue::Vector2(v * rhs),
            BaseValue::Vector3(v) => BaseValue::Vector3(v * rhs),
            BaseValue::Vector4(v) => BaseValue::Vector4(v * rhs),
            BaseValue::Quaternion(v) => BaseValue::Quaternion(EulerVec3(v.0 * rhs)),
        }
    }
}

impl Div<f32> for BaseValue {
    type Output = BaseValue;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            BaseValue::Float(v) => BaseValue::Float(v / rhs),
            BaseValue::Vector2(v) => BaseValue::Vector2(v / rhs),
            BaseValue::Vector3(v) => BaseValue::Vector3(v / rhs),
            BaseValue::Vector4(v) => BaseValue::Vector4(v / rhs),
            BaseValue::Quaternion(v) => BaseValue::Quaternion(EulerVec3(v.0 / rhs)),
        }
    }
}

impl Index<usize> for BaseValue {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            BaseValue::Float(f) => f,
            BaseValue::Vector2(v) => &v[index],
            BaseValue::Vector3(v) => &v[index],
            BaseValue::Vector4(v) => &v[index],
            BaseValue::Quaternion(v) => match index {
                0 => &v.x,
                1 => &v.y,
                2 => &v.z,
                _ => panic!("Invalid index for EulerVec3"),
            },
        }
    }
}
impl IndexMut<usize> for BaseValue {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            BaseValue::Float(f) => f,
            BaseValue::Vector2(v) => &mut v[index],
            BaseValue::Vector3(v) => &mut v[index],
            BaseValue::Vector4(v) => &mut v[index],
            BaseValue::Quaternion(v) => match index {
                0 => &mut v.x,
                1 => &mut v.y,
                2 => &mut v.z,
                _ => panic!("Invalid index for EulerVec3"),
            },
        }
    }
}

// Stack-backed iterator for BaseValue that preserves length without heap allocs
pub struct BaseValueIntoIter {
    base_value: BaseValue,
    idx: usize,
}

impl Iterator for BaseValueIntoIter {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.base_value.len() {
            return None;
        }
        let v = self.base_value[self.idx];
        self.idx += 1;
        Some(v)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.base_value.len().saturating_sub(self.idx);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for BaseValueIntoIter {}

impl IntoIterator for BaseValue {
    type Item = f32;
    type IntoIter = BaseValueIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        BaseValueIntoIter {
            base_value: self,
            idx: 0,
        }
    }
}
