//! FFI bindings for Track properties.

use std::ffi::{CStr, CString, c_char};
use std::os::raw::{c_float, c_int};
use std::ptr;

use crate::animation::property::{PathProperty, PathPropertyGlobal};
use crate::animation::property::{ValueProperty, ValuePropertyGlobal};
use crate::values::value::BaseValue;
use std::{cell::RefCell, rc::Rc};

/// Creates a new ValuePropertyGlobal with None value.
#[unsafe(no_mangle)]
pub extern "C" fn value_property_global_create() -> *mut ValuePropertyGlobal {
    Box::into_raw(Box::new(Rc::new(RefCell::new(None))))
}

/// Creates a new PathPropertyGlobal with default values.
#[unsafe(no_mangle)]
pub extern "C" fn path_property_global_create() -> *mut PathPropertyGlobal {
    Box::into_raw(Box::new(Rc::new(RefCell::new(PathProperty {
        time: 0.0,
        prev_point: None,
        point: None,
    }))))
}

/// Destroys a ValuePropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_destroy(property: *mut ValuePropertyGlobal) {
    if property.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(property));
    }
}

/// Destroys a PathPropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_global_destroy(property: *mut PathPropertyGlobal) {
    if property.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(property));
    }
}

/// Gets a value from a ValuePropertyGlobal into a WrapBaseValue.
/// Returns 1 if a value was retrieved, 0 if property was None.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_get_value(
    property: *const ValuePropertyGlobal,
    out_value: *mut crate::ffi::types::WrapBaseValue,
) -> c_int {
    if property.is_null() || out_value.is_null() {
        return 0;
    }
    
    let property_ref = unsafe { &*property };
    let value_opt = property_ref.borrow();
    
    if let Some(base_value) = &*value_opt {
        let out = unsafe { &mut *out_value };
        match base_value {
            BaseValue::Float(v) => {
                out.ty = crate::ffi::types::WrapBaseValueType::Float;
                out.value.float = *v;
            },
            BaseValue::Vector3(v) => {
                out.ty = crate::ffi::types::WrapBaseValueType::Vec3;
                out.value.vec3.x = v.x;
                out.value.vec3.y = v.y;
                out.value.vec3.z = v.z;
            },
            BaseValue::Quaternion(q) => {
                out.ty = crate::ffi::types::WrapBaseValueType::Quat;
                out.value.quat.x = q.x;
                out.value.quat.y = q.y;
                out.value.quat.z = q.z;
                out.value.quat.w = q.w;
            },
            BaseValue::Vector4(v) => {
                out.ty = crate::ffi::types::WrapBaseValueType::Vec4;
                out.value.vec4.x = v.x;
                out.value.vec4.y = v.y;
                out.value.vec4.z = v.z;
                out.value.vec4.w = v.w;
            },
        }
        1
    } else {
        0
    }
}

/// Sets a value in a ValuePropertyGlobal using WrapBaseValue.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_set_value(
    property: *mut ValuePropertyGlobal,
    value: crate::ffi::types::WrapBaseValue,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        *property_ref.borrow_mut() = match value.ty {
            crate::ffi::types::WrapBaseValueType::Float => {
                Some(BaseValue::Float(value.value.float))
            }
            crate::ffi::types::WrapBaseValueType::Vec3 => {
                let v = value.value.vec3;
                Some(BaseValue::Vector3(glam::Vec3::new(v.x, v.y, v.z)))
            }
            crate::ffi::types::WrapBaseValueType::Quat => {
                let q = value.value.quat;
                Some(BaseValue::Quaternion(glam::Quat::from_xyzw(
                    q.x, q.y, q.z, q.w,
                )))
            }
            crate::ffi::types::WrapBaseValueType::Vec4 => {
                let v = value.value.vec4;
                Some(BaseValue::Vector4(glam::Vec4::new(v.x, v.y, v.z, v.w)))
            }
        };
    }
}

/// Sets a float value in a ValuePropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_set_float(
    property: *mut ValuePropertyGlobal,
    value: c_float,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        *property_ref.borrow_mut() = Some(BaseValue::Float(value));
    }
}

/// Sets a Vec3 value in a ValuePropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_set_vec3(
    property: *mut ValuePropertyGlobal,
    x: c_float,
    y: c_float,
    z: c_float,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        *property_ref.borrow_mut() = Some(BaseValue::Vector3(glam::Vec3::new(x, y, z)));
    }
}

/// Sets a Quat value in a ValuePropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_set_quat(
    property: *mut ValuePropertyGlobal,
    x: c_float,
    y: c_float,
    z: c_float,
    w: c_float,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        *property_ref.borrow_mut() = Some(BaseValue::Quaternion(glam::Quat::from_xyzw(x, y, z, w)));
    }
}

/// Clears the value in a ValuePropertyGlobal.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn value_property_global_clear(property: *mut ValuePropertyGlobal) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        *property_ref.borrow_mut() = None;
    }
}

/// Initializes a PathProperty with a new point.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_global_init(
    property: *mut PathPropertyGlobal,
    point_data: *mut crate::point_definition::BasePointDefinitionGlobal,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        let point = if point_data.is_null() {
            None
        } else {
            Some(Rc::clone(&*point_data))
        };
        property_ref.borrow_mut().init(point);
    }
}

/// Finishes a PathProperty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_global_finish(property: *mut PathPropertyGlobal) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        property_ref.borrow_mut().finish();
    }
}

/// Sets the time of a PathProperty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_global_set_time(
    property: *mut PathPropertyGlobal,
    time: c_float,
) {
    if property.is_null() {
        return;
    }
    unsafe {
        let property_ref = &mut *property;
        property_ref.borrow_mut().time = time;
    }
}
