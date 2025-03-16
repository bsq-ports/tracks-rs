use tracing::info;

use crate::point_definition::base_point_definition;
use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;

use crate::point_definition::vector4_point_definition::Vector4PointDefinition;

use crate::point_definition::vector3_point_definition::Vector3PointDefinition;

use crate::point_definition::float_point_definition::FloatPointDefinition;

use crate::point_definition::PointDefinition;
use crate::values::value::BaseValue;

use std::ffi::CStr;
use std::ffi::c_char;
use std::ffi::c_void;
use std::slice;

use crate::base_provider_context::BaseProviderContext;

use crate::values::base_ffi::BaseFFIProviderValues;

use crate::values::base_ffi::BaseFFIProvider;

use super::json;
use super::json::FFIJsonValue;
use super::types::WrapBaseValue;
use super::types::WrapBaseValueType;
use super::types::WrapQuat;
use super::types::WrapVec3;
use super::types::WrapVec4;

#[repr(C)]
pub enum PointDefinitionType {
    Float = 0,
    Vector3 = 1,
    Vector4 = 2,
    Quaternion = 3,
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_ffi_provider(
    func: *const BaseFFIProvider,
    user_value: *mut c_void,
) -> *mut BaseFFIProviderValues {
    assert!(!func.is_null());

    let context = Box::new(BaseFFIProviderValues::new(func, user_value));
    let context_ptr = Box::leak(context);
    context_ptr
}

/// Dispose the base provider. Consumes
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_dipose_base_ffi_provider(func: *mut BaseFFIProviderValues) {
    assert!(!func.is_null());

    // destroy
    unsafe {
        let _ = Box::from_raw(func);
    };
}

/// CONTEXT
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_provider_context() -> *mut BaseProviderContext {
    let context = Box::new(BaseProviderContext::new());
    let context_ptr = Box::leak(context);
    context_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_set_base_provider(
    context: *mut BaseProviderContext,
    base: *const c_char,
    values: *mut f32,
    count: usize,
    quat: bool,
) {
    let base_str = unsafe { CStr::from_ptr(base).to_str().unwrap() };
    let context = unsafe { &mut *context };
    context.set_values(base_str, unsafe {
        let v = slice::from_raw_parts(values, count);
        info!("v: {} {:?}", base_str, v);
        BaseValue::from_slice(v, quat)
    });
}

///FLOAT POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const FloatPointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(FloatPointDefinition::new(value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_float(
    point_definition: *const FloatPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> FloatInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    FloatInterpolationResult { value, is_last }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_count(
    point_definition: *const FloatPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_has_base_provider(
    point_definition: *const FloatPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///BASE POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_point_definition(
    json: *const FFIJsonValue,
    ty: WrapBaseValueType,
    context: *mut BaseProviderContext,
) -> *mut base_point_definition::BasePointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let context = unsafe { &*context };

    let point_definition: base_point_definition::BasePointDefinition = match ty {
        WrapBaseValueType::Vec3 => Vector3PointDefinition::new(value, context).into(),
        WrapBaseValueType::Quat => QuaternionPointDefinition::new(value, context).into(),
        WrapBaseValueType::Vec4 => Vector4PointDefinition::new(value, context).into(),
        WrapBaseValueType::Float => FloatPointDefinition::new(value, context).into(),
    };

    let point_definition = Box::new(point_definition);
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_base_point_definition(
    point_definition: *const base_point_definition::BasePointDefinition,
    time: f32,
    is_last_out: *mut bool,
    context: *mut BaseProviderContext,
) -> WrapBaseValue {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    unsafe { *is_last_out = is_last };

    WrapBaseValue::from(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_count(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_has_base_provider(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///VECTOR3 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector3_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const Vector3PointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector3PointDefinition::new(value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector3(
    point_definition: *const Vector3PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector3InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector3InterpolationResult {
        value: WrapVec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_count(
    point_definition: *const Vector3PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_has_base_provider(
    point_definition: *const Vector3PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///VECTOR4 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector4_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const Vector4PointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector4PointDefinition::new(value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector4(
    point_definition: *const Vector4PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector4InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector4InterpolationResult {
        value: WrapVec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_count(
    point_definition: *const Vector4PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_has_base_provider(
    point_definition: *const Vector4PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

///QUATERNION POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_quat_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const QuaternionPointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(QuaternionPointDefinition::new(value, unsafe { &*context }));
    let point_definition_ptr = Box::leak(point_definition);
    point_definition_ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_quat(
    point_definition: *const QuaternionPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> QuaternionInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    QuaternionInterpolationResult {
        value: WrapQuat {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_count(
    point_definition: *const QuaternionPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_has_base_provider(
    point_definition: *const QuaternionPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
