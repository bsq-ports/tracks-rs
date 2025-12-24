use glam::{Quat, Vec3, Vec4};

use crate::values::value::BaseValue;

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct WrapVec3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct WrapVec4 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct WrapQuat {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
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
                WrapBaseValueType::Unknown => panic!("Unknown WrapBaseValueType encountered"),
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

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vec3Option {
    pub value: WrapVec3,
    pub has_value: bool,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct QuatOption {
    pub value: WrapQuat,
    pub has_value: bool,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct Vec4Option {
    pub value: WrapVec4,
    pub has_value: bool,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct FloatOption {
    pub value: f32,
    pub has_value: bool,
}

impl From<Option<Vec3>> for Vec3Option {
    fn from(option: Option<Vec3>) -> Self {
        match option {
            Some(v) => Vec3Option {
                value: WrapVec3 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                },
                has_value: true,
            },
            None => Vec3Option {
                value: WrapVec3 { x: 0.0, y: 0.0, z: 0.0 },
                has_value: false,
            },
        }
    }
}

impl From<Option<Quat>> for QuatOption {
    fn from(option: Option<Quat>) -> Self {
        match option {
            Some(v) => QuatOption {
                value: WrapQuat {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                },
                has_value: true,
            },
            None => QuatOption {
                value: WrapQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                has_value: false,
            },
        }
    }
}

impl From<Option<Vec4>> for Vec4Option {
    fn from(option: Option<Vec4>) -> Self {
        match option {
            Some(v) => Vec4Option {
                value: WrapVec4 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                },
                has_value: true,
            },
            None => Vec4Option {
                value: WrapVec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                has_value: false,
            },
        }
    }
}

impl From<Option<f32>> for FloatOption {
    fn from(option: Option<f32>) -> Self {
        match option {
            Some(v) => FloatOption {
                value: v,
                has_value: true,
            },
            None => FloatOption {
                value: 0.0,
                has_value: false,
            },
        }
    }
}

impl From<Option<BaseValue>> for Vec3Option {
    fn from(option: Option<BaseValue>) -> Self {
        match option {
            Some(BaseValue::Vector3(v)) => Vec3Option {
                value: WrapVec3 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                },
                has_value: true,
            },
            _ => Vec3Option {
                value: WrapVec3 { x: 0.0, y: 0.0, z: 0.0 },
                has_value: false,
            },
        }
    }
}
impl From<Option<BaseValue>> for FloatOption {
    fn from(option: Option<BaseValue>) -> Self {
        match option {
            Some(BaseValue::Float(v)) => FloatOption {
                value: v,
                has_value: true,
            },
            _ => FloatOption {
                value: 0.0,
                has_value: false,
            },
        }
    }
}
impl From<Option<BaseValue>> for QuatOption {
    fn from(option: Option<BaseValue>) -> Self {
        match option {
            Some(BaseValue::Quaternion(v)) => QuatOption {
                value: WrapQuat {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                },
                has_value: true,
            },
            _ => QuatOption {
                value: WrapQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
                has_value: false,
            },
        }
    }
}
impl From<Option<BaseValue>> for Vec4Option {
    fn from(option: Option<BaseValue>) -> Self {
        match option {
            Some(BaseValue::Vector4(v)) => Vec4Option {
                value: WrapVec4 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                    w: v.w,
                },
                has_value: true,
            },
            _ => Vec4Option {
                value: WrapVec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 },
                has_value: false,
            },
        }
    }
}