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

#[repr(C)]
pub struct FloatInterpolationResult {
    pub value: f32,
    pub is_last: bool,
}

#[repr(C)]
pub struct Vector3InterpolationResult {
    pub value: WrapVec3,
    pub is_last: bool,
}

#[repr(C)]
pub struct Vector4InterpolationResult {
    pub value: WrapVec4,
    pub is_last: bool,
}

#[repr(C)]
pub struct QuaternionInterpolationResult {
    pub value: WrapQuat,
    pub is_last: bool,
}

/// JSON FFI
#[repr(C)]
#[derive(Debug)]
pub enum JsonValueType {
    Number,
    Null,
    String,
    Array,
}

#[repr(C)]
pub struct FFIJsonValue {
    pub value_type: JsonValueType,
    pub data: JsonValueData,
}

#[repr(C)]
pub union JsonValueData {
    pub number_value: f64,
    pub string_value: *const c_char,
    pub array: *const JsonArray,
}

#[repr(C)]
pub struct JsonArray {
    pub elements: *const FFIJsonValue,
    pub length: usize,
}
