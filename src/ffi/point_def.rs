use crate::values::value::BaseValue;

use std::ffi::CStr;
use std::ffi::c_char;
use std::ffi::c_void;
use std::slice;

use crate::base_provider_context::BaseProviderContext;

use crate::values::base_ffi::BaseFFIProviderValues;

use crate::values::base_ffi::BaseFFIProvider;

mod base;
mod float;
mod quat;
mod vec3;
mod vec4;

#[repr(C)]
pub enum PointDefinitionType {
    Float = 0,
    Vector3 = 1,
    Vector4 = 2,
    Quaternion = 3,
}

#[unsafe(no_mangle)]
/// Create a `BaseFFIProviderValues` wrapper from a C function pointer and user value.
///
/// # Safety
/// - `func` must be a valid pointer to a `BaseFFIProvider` function table and not null.
/// - `user_value` is passed through as-is and its ownership remains with the caller.
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
/// Dispose of a `BaseFFIProviderValues` previously created.
///
/// # Safety
/// - `func` must be a pointer previously returned by `tracks_make_base_ffi_provider` and not already freed.
pub unsafe extern "C" fn tracks_dipose_base_ffi_provider(func: *mut BaseFFIProviderValues) {
    assert!(!func.is_null());

    // destroy
    unsafe {
        let _ = Box::from_raw(func);
    };
}

/// CONTEXT
#[unsafe(no_mangle)]
/// Create a `BaseProviderContext` and return a pointer to it.
///
/// # Safety
/// - The returned pointer is owned by the caller and must be freed with the matching disposal function if provided.
pub unsafe extern "C" fn tracks_make_base_provider_context() -> *mut BaseProviderContext {
    let context = Box::new(BaseProviderContext::new());

    (Box::leak(context)) as _
}

#[unsafe(no_mangle)]
/// Set a named base provider's values from a raw float buffer.
///
/// # Safety
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `base` must be a valid null-terminated C string.
/// - `values` must point to `count` contiguous `f32` values.
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
