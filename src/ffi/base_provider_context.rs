use crate::base_provider_context::BaseProviderContext;
use crate::base_value::{BaseValue, WrapBaseValueType};
use crate::ffi::types::{WrapBaseValue, WrappedValues};
use crate::providers::base_ffi::{BaseFFIProvider, BaseFFIProviderValues};
use std::ffi::{CStr, c_void};
use std::ptr;

#[repr(C)]
pub enum PointDefinitionType {
    Float = 0,
    Vector3 = 1,
    Vector4 = 2,
    Quaternion = 3,
}

/// Create a new `BaseProviderContext` and return a raw pointer to it.
#[unsafe(no_mangle)]
pub extern "C" fn base_provider_context_create() -> *mut BaseProviderContext {
    Box::into_raw(Box::new(BaseProviderContext::new()))
}

/// Destroy a `BaseProviderContext` previously returned by `base_provider_context_create`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_destroy(ctx: *mut BaseProviderContext) {
    if ctx.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(ctx));
    }
}

/// Create a `BaseFFIProviderValues` wrapper from a C function pointer and user value.
///
/// # Safety
/// - `func` must be a valid pointer to a `BaseFFIProvider` function table and not null.
/// - `user_value` is passed through as-is and its ownership remains with the caller.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_ffi_provider(
    func: *const BaseFFIProvider,
    user_value: *mut c_void,
) -> *mut BaseFFIProviderValues {
    assert!(!func.is_null());

    let context = Box::new(BaseFFIProviderValues::new(func, user_value));

    Box::into_raw(context)
}

/// Set a base provider value by name. `value` is a `WrapBaseValue` (C layout) converted into `BaseValue`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_set_value(
    ctx: *mut BaseProviderContext,
    base: *const std::os::raw::c_char,
    value: WrapBaseValue,
) {
    if ctx.is_null() || base.is_null() {
        return;
    }

    let ctx_ref = unsafe { &mut *ctx };
    let cstr = unsafe { CStr::from_ptr(base) };
    if let Ok(name) = cstr.to_str() {
        let bv: BaseValue = value.into();
        ctx_ref.set_values(name, bv);
    }
}

/// Get a base provider value by name as a `WrapBaseValue`.
/// The returned `WrapBaseValue` points into data owned by `ctx` (via slice pointer), callers must not free it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_get_value(
    ctx: *const BaseProviderContext,
    base: *const std::os::raw::c_char,
) -> WrapBaseValue {
    if ctx.is_null() || base.is_null() {
        return unsafe { std::mem::zeroed() };
    }

    let ctx_ref = unsafe { &*ctx };
    let cstr = unsafe { CStr::from_ptr(base) };
    if let Ok(name) = cstr.to_str() {
        let bv = ctx_ref.get_values(name);

        bv.into()
    } else {
        unsafe { std::mem::zeroed() }
    }
}

/// Get base provider values as a pointer+length pair. The returned `WrappedValues` borrows data from `ctx`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_get_values_array(
    ctx: *const BaseProviderContext,
    base: *const std::os::raw::c_char,
) -> WrappedValues {
    if ctx.is_null() || base.is_null() {
        return WrappedValues {
            values: ptr::null(),
            length: 0,
        };
    }

    let ctx_ref = unsafe { &*ctx };
    let cstr = unsafe { CStr::from_ptr(base) };
    if let Ok(name) = cstr.to_str() {
        let bvref = ctx_ref.get_values(name);
        let slice = bvref.as_slice();
        WrappedValues {
            values: slice.as_ptr(),
            length: slice.len(),
        }
    } else {
        WrappedValues {
            values: ptr::null(),
            length: 0,
        }
    }
}

/// Get the type of the base provider value for `base` (Vec3/Quat/Vec4/Float)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_get_type(
    ctx: *const BaseProviderContext,
    base: *const std::os::raw::c_char,
) -> WrapBaseValueType {
    if ctx.is_null() || base.is_null() {
        return WrapBaseValueType::Unknown;
    }

    let ctx_ref = unsafe { &*ctx };
    let cstr = unsafe { CStr::from_ptr(base) };
    if let Ok(name) = cstr.to_str() {
        ctx_ref.get_values(name).get_type()
    } else {
        WrapBaseValueType::Unknown
    }
}

/// Call `update_providers` on the `BaseProviderContext` with a delta time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_provider_context_update(ctx: *mut BaseProviderContext, delta: f32) {
    if ctx.is_null() {
        return;
    }

    let ctx_ref = unsafe { &*ctx };
    ctx_ref.update_providers(delta);
}
