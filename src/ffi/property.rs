use std::ffi::c_char;
use std::str::FromStr;

use crate::animation::property::{PathProperty, ValueProperty};
use crate::animation::track::PropertyNames;
use crate::base_provider_context::BaseProviderContext;
use crate::ffi::types::{WrapBaseValue, WrapBaseValueType};
use crate::point_definition::base_point_definition::{self};
use crate::values::value::BaseValue;

use super::time::CTimeUnit;

#[repr(C)]
pub struct CValueNullable {
    has_value: bool,
    value: WrapBaseValue,
}

#[repr(C)]
#[derive(Default)]
pub struct CValueProperty {
    value: CValueNullable,
    last_updated: CTimeUnit,
}

impl Default for CValueNullable {
    fn default() -> Self {
        CValueNullable {
            has_value: false,
            value: unsafe { std::mem::zeroed() },
        }
    }
}

impl From<Option<BaseValue>> for CValueNullable {
    fn from(value: Option<BaseValue>) -> Self {
        match value {
            Some(base_value) => CValueNullable {
                has_value: true,
                value: base_value.into(),
            },
            None => CValueNullable::default(),
        }
    }
}

impl From<ValueProperty> for CValueProperty {
    fn from(prop: ValueProperty) -> Self {
        match prop.get_value() {
            Some(base_value) => CValueProperty {
                value: CValueNullable {
                    has_value: true,
                    value: base_value.into(),
                },
                last_updated: prop.last_updated.into(),
            },
            None => CValueProperty::default(),
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn path_property_create() -> *mut PathProperty {
    Box::into_raw(Box::new(PathProperty::default()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_finish(ptr: *mut PathProperty) {
    if !ptr.is_null() {
        unsafe {
            let inner = &mut *ptr;
            inner.finish();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_to_property_name(ptr: *const c_char) -> PropertyNames {
    if ptr.is_null() {
        return PropertyNames::UnknownPropertyName;
    }
    unsafe {
        let c_str = std::ffi::CStr::from_ptr(ptr);
        match c_str.to_str() {
            Ok(str_slice) => {
                PropertyNames::from_str(str_slice).unwrap_or(PropertyNames::UnknownPropertyName)
            }
            Err(_) => PropertyNames::UnknownPropertyName,
        }
    }
}

/// # Safety
/// - `ptr` must be a valid pointer to a `PathProperty` created by `path_property_create`.
/// - After calling this function the `PathProperty` remains owned by the caller; this function only performs finalization.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_init(
    ptr: *mut PathProperty,
    // nullable
    new_point_data: *mut base_point_definition::BasePointDefinition,
) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let inner = &mut *ptr;
        let point_data = new_point_data.as_ref().cloned();

        inner.init(point_data);
    }
}
/// # Safety
/// - `ptr` must be a valid pointer to a `PathProperty`.
/// - `new_point_data`, if non-null, must point to a valid `BasePointDefinition` and ownership of its contents may be moved.
/// 
/// Consumes the path property and frees its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_free(ptr: *mut PathProperty) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}
/// # Safety
/// - `ptr` must be a pointer previously returned by `path_property_create` and not already freed.
/// - Passing null is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_time(ptr: *const PathProperty) -> f32 {
    if ptr.is_null() {
        return 0.0;
    }
    unsafe {
        let inner: &crate::point_definition::point_definition_interpolation::PointDefinitionInterpolation = &*ptr;
        inner.interpolate_time
    }
}
/// # Safety
/// - `ptr` must be a valid pointer to a `PathProperty`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_set_time(ptr: *mut PathProperty, time: f32) {
    if !ptr.is_null() {
        unsafe {
            let inner = &mut *ptr;
            inner.interpolate_time = time;
        }
    }
}
/// # Safety
/// - `ptr` must be a valid, non-null pointer to a `PathProperty`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_interpolate(
    ptr: *mut PathProperty,
    time: f32,
    context: *const BaseProviderContext,
) -> CValueNullable {
    if ptr.is_null() || context.is_null() {
        return CValueNullable::default();
    }
    unsafe {
        let context = &*context;
        let inner = &mut *ptr;
        inner.interpolate(time, context).into()
    }
}
/// # Safety
/// - `ptr` must be a valid pointer to a `PathProperty`.
/// - `context` must be a valid pointer to a `BaseProviderContext` for the duration of the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_type(ptr: *const PathProperty) -> WrapBaseValueType {
    unsafe {
        if ptr.is_null() {
            return WrapBaseValueType::Unknown; // Default type if pointer is null
        }

        let inner = &*ptr;

        inner.get_type()
    }
}
/// # Safety
/// - `ptr` may be null; if non-null it must point to a valid `PathProperty`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_type(ptr: *const ValueProperty) -> WrapBaseValueType {
    if ptr.is_null() {
        return WrapBaseValueType::Unknown; // Default type if pointer is null
    }

    let inner: &ValueProperty = unsafe { &*ptr };

    inner.get_type()
}
/// # Safety
/// - `ptr` may be null; if non-null it must point to a valid `ValueProperty`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_value(ptr: *const ValueProperty) -> CValueProperty {
    if ptr.is_null() {
        return CValueProperty::default(); // Default type if pointer is null
    }

    let inner = unsafe { &*ptr };
    inner.clone().into()
}
/// # Safety
/// - `ptr` may be null; if non-null it must point to a valid `ValueProperty`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_last_updated(ptr: *const ValueProperty) -> CTimeUnit {
    if ptr.is_null() {
        return CTimeUnit::default();
        // Default type if pointer is null
    }

    let inner = unsafe { &*ptr };
    inner.last_updated.into()
}
