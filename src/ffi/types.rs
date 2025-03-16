use glam::{Quat, Vec3, Vec4};

use crate::values::value::BaseValue;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapVec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapVec4 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapQuat {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum WrapBaseValueType {
    Vec3 = 0,
    Quat = 1,
    Vec4 = 2,
    Float = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union WrapBaseValueUnion {
    pub(crate) vec3: WrapVec3,
    pub(crate) quat: WrapQuat,
    pub(crate) vec4: WrapVec4,
    pub(crate) float_v: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapBaseValue {
    pub(crate) ty: WrapBaseValueType,
    pub(crate) value: WrapBaseValueUnion,
}



impl From<BaseValue> for WrapBaseValue {
    fn from(value: BaseValue) -> Self {
        match value {
            BaseValue::Vector3(v) => Self {
                ty: WrapBaseValueType::Vec3,
                value: WrapBaseValueUnion {
                    vec3: WrapVec3 {
                        x: v.x,
                        y: v.y,
                        z: v.z,
                    },
                },
            },
            BaseValue::Quaternion(v) => Self {
                ty: WrapBaseValueType::Quat,
                value: WrapBaseValueUnion {
                    quat: WrapQuat {
                        x: v.x,
                        y: v.y,
                        z: v.z,
                        w: v.w,
                    },
                },
            },
            BaseValue::Vector4(v) => Self {
                ty: WrapBaseValueType::Vec4,
                value: WrapBaseValueUnion {
                    vec4: WrapVec4 {
                        x: v.x,
                        y: v.y,
                        z: v.z,
                        w: v.w,
                    },
                },
            },
            BaseValue::Float(v) => Self {
                ty: WrapBaseValueType::Float,
                value: WrapBaseValueUnion { float_v: v },
            },
        }
    }
}

impl From<WrapBaseValue> for BaseValue {
    fn from(value: WrapBaseValue) -> Self {
        unsafe {
            match value.ty {
                WrapBaseValueType::Vec3 => BaseValue::Vector3(Vec3::new(
                    value.value.vec3.x,
                    value.value.vec3.y,
                    value.value.vec3.z,
                )),
                WrapBaseValueType::Quat => BaseValue::Quaternion(Quat::from_xyzw(
                    value.value.quat.x,
                    value.value.quat.y,
                    value.value.quat.z,
                    value.value.quat.w,
                )),
                WrapBaseValueType::Vec4 => BaseValue::Vector4(Vec4::new(
                    value.value.vec4.x,
                    value.value.vec4.y,
                    value.value.vec4.z,
                    value.value.vec4.w,
                )),
                WrapBaseValueType::Float => BaseValue::Float(value.value.float_v),
            }
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WrappedValues {
    pub values: *const f32,
    pub length: usize,
}
