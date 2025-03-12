use tracing::info;

use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;

use crate::point_definition::vector4_point_definition::Vector4PointDefinition;

use crate::point_definition::vector3_point_definition::Vector3PointDefinition;

use crate::point_definition::float_point_definition::FloatPointDefinition;

use crate::point_definition::PointDefinition;
use crate::values::value::BaseValue;

use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::slice;

use crate::values::base_provider_context::BaseProviderContext;

use crate::values::base_ffi::BaseFFIProviderValues;

use crate::values::base_ffi::BaseFFIProvider;

use super::json;
use super::types;

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
    json: *const types::FFIJsonValue,
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
) -> types::FloatInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    types::FloatInterpolationResult { value, is_last }
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

///VECTOR3 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector3_point_definition(
    json: *const types::FFIJsonValue,
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
) -> types::Vector3InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    types::Vector3InterpolationResult {
        value: types::WrapVec3 {
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
    json: *const types::FFIJsonValue,
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
) -> types::Vector4InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    types::Vector4InterpolationResult {
        value: types::WrapVec4 {
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
    json: *const types::FFIJsonValue,
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
) -> types::QuaternionInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    types::QuaternionInterpolationResult {
        value: types::WrapQuat {
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
