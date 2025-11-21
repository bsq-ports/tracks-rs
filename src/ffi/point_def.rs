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

mod base;
mod float;
mod vec3;
mod vec4;
mod quat;

#[repr(C)]
pub enum PointDefinitionType {
    Float = 0,
    Vector3 = 1,
    Vector4 = 2,
    Quaternion = 3,
}




#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_ffi_provider(
    func: *const BaseFFIProvider,
    user_value: *mut c_void,
) -> *mut BaseFFIProviderValues {
    assert!(!func.is_null());

    let context = Box::new(BaseFFIProviderValues::new(func, user_value));

    (Box::leak(context)) as _
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

    (Box::leak(context)) as _
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
        BaseValue::from_slice(v, quat)
    });
}
