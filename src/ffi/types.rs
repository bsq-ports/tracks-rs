use std::ffi::c_char;

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
    pub(crate) float: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WrapBaseValue {
    pub(crate) ty: WrapBaseValueType,
    pub(crate) value: WrapBaseValueUnion,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct WrappedValues {
    pub values: *const f32,
    pub length: usize,
}
