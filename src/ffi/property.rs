use crate::{
    animation::property::{PathProperty, ValueProperty},
    ffi::types::WrapBaseValue,
    values::value::BaseValue,
};

use super::types::RcCRefCell;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_create() -> *mut ValueProperty {
    let property: ValueProperty = None;
    Box::into_raw(Box::new(property))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_destroy(property: *mut ValueProperty) {
    if !property.is_null() {
        let _ = Box::from_raw(property); // Convert back to Box and drop it
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_to_global(
    property: *mut ValueProperty,
) -> RcCRefCell<ValueProperty> {
    if property.is_null() {
        return RcCRefCell::null();
    }

    // Take ownership of the box and create a new owned copy
    let owned_property = Box::from_raw(property);
    owned_property.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_dispose(property: RcCRefCell<ValueProperty>) {
    property.unleak();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_get(
    property: *const ValueProperty,
    out_value: *mut WrapBaseValue,
) -> bool {
    if property.is_null() || out_value.is_null() {
        return false;
    }

    unsafe {
        match &*property {
            Some(value) => {
                *out_value = (*value).into();
                true
            }
            _ => false,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_set(
    property: *mut ValueProperty,
    value: *const WrapBaseValue,
) -> bool {
    if property.is_null() || value.is_null() {
        return false;
    }

    unsafe {
        let base_value = BaseValue::from(*value);
        *property = Some(base_value);
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_create() -> *mut PathProperty {
    let property: PathProperty = PathProperty::default();
    Box::into_raw(Box::new(property))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_destroy(property: *mut PathProperty) {
    if !property.is_null() {
        let _ = unsafe { Box::from_raw(property) }; // Convert back to Box and drop it
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_to_global(
    property: *mut PathProperty,
) -> RcCRefCell<PathProperty> {
    if property.is_null() {
        return RcCRefCell::null();
    }

    // Take ownership of the box and create a new owned copy
    let owned_property = unsafe { Box::from_raw(property) };
    owned_property.into()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_global_dispose(property: RcCRefCell<PathProperty>) {
    property.unleak();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_finish(property: *mut PathProperty) -> bool {
    if property.is_null() {
        return false;
    }
    unsafe {
        (*property).finish();
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_init(
    property: *mut PathProperty,
    new_point_data: RcCRefCell<crate::point_definition::BasePointDefinition>,
) -> bool {
    if property.is_null() {
        return false;
    }

    let point_data = if new_point_data.is_null() {
        None
    } else {
        Some(new_point_data.unleak())
    };

    unsafe {
        (*property).init(point_data);
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_interpolate(
    property: *const PathProperty,
    time: f32,
    context: *const crate::values::base_provider_context::BaseProviderContext,
    out_value: *mut WrapBaseValue,
) -> bool {
    if property.is_null() || context.is_null() || out_value.is_null() {
        return false;
    }

    unsafe {
        match (*property).interpolate(time, &*context) {
            Some(value) => {
                *out_value = value.into();
                true
            }
            None => false,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_set_time(property: *mut PathProperty, time: f32) -> bool {
    if property.is_null() {
        return false;
    }

    unsafe {
        (*property).time = time;
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_time(property: *const PathProperty) -> f32 {
    if property.is_null() {
        return 0.0;
    }

    unsafe { (*property).time }
}
