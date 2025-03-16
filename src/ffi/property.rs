
use crate::animation::property::{PathProperty, ValueProperty};
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
                ..Default::default()
            },
            None => CValueProperty::default(),
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn path_property_create() -> *mut PathProperty<'static> {
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
pub unsafe extern "C" fn path_property_init(
    ptr: *mut PathProperty,
    // nullable
    new_point_data: *const base_point_definition::BasePointDefinition,
) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let inner = &mut *ptr;
        inner.init(new_point_data.as_ref());
    }
}

/// Consumes the path property and frees its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_free(ptr: *mut PathProperty) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_time(ptr: *const PathProperty) -> f32 {
    if ptr.is_null() {
        return 0.0;
    }
    unsafe {
        let inner = &*ptr;
        inner.time
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_set_time(ptr: *mut PathProperty, time: f32) {
    if !ptr.is_null() {
        unsafe {
            let inner = &mut *ptr;
            inner.time = time;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_interpolate(
    ptr: *mut PathProperty,
    time: f32,
    context: *mut BaseProviderContext,
) -> CValueNullable {
    if ptr.is_null() || context.is_null() {
        return CValueNullable::default();
    }
    unsafe {
        let context = &mut *context;
        let inner = &mut *ptr;
        inner.interpolate(time, context).into()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_type(ptr: *const PathProperty) -> WrapBaseValueType {
    unsafe {
        if ptr.is_null() {
            return WrapBaseValueType::Float; // Default type if pointer is null
        }

        let inner = &*ptr;
        match inner.point.or(inner.prev_point) {
            Some(value_type) => match value_type {
                base_point_definition::BasePointDefinition::Float(_) => WrapBaseValueType::Float,
                base_point_definition::BasePointDefinition::Vector3(_) => WrapBaseValueType::Vec3,
                base_point_definition::BasePointDefinition::Quaternion(_) => {
                    WrapBaseValueType::Quat
                }
                base_point_definition::BasePointDefinition::Vector4(_) => WrapBaseValueType::Vec4,
            },
            None => WrapBaseValueType::Float, // Default to Float if type is not set
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_type(ptr: *const ValueProperty) -> WrapBaseValueType {
    if ptr.is_null() {
        return WrapBaseValueType::Float; // Default type if pointer is null
    }

    let inner = unsafe { &*ptr };


    match inner.get_value() {
        Some(value_type) => match value_type {
            BaseValue::Float(_) => WrapBaseValueType::Float,
            BaseValue::Vector3(_) => WrapBaseValueType::Vec3,
            BaseValue::Quaternion(_) => WrapBaseValueType::Quat,
            BaseValue::Vector4(_) => WrapBaseValueType::Vec4,
        },
        None => WrapBaseValueType::Float, // Default to Float if type is not set
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_value(ptr: *const ValueProperty) -> CValueProperty {
    if ptr.is_null() {
        return CValueProperty::default(); // Default type if pointer is null
    }

    let inner = *unsafe { &*ptr };
    inner.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn property_get_last_updated(ptr: *const ValueProperty) -> CTimeUnit {
    if ptr.is_null() {
        return CTimeUnit::default(); 
        // Default type if pointer is null
    }

    let inner = *unsafe { &*ptr };
    inner.last_updated.into()
}

