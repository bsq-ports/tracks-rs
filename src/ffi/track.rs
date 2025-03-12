use std::{
    cell::RefCell,
    ffi::{CStr, c_char},
    ptr,
    rc::Rc,
};

use crate::animation::{
    property::{PathProperty, PathPropertyGlobal, ValueProperty, ValuePropertyGlobal},
    tracks::TrackGlobal,
};

/// Creates a new empty track global and returns a pointer to it
#[unsafe(no_mangle)]
pub extern "C" fn track_global_create() -> *mut TrackGlobal {
    let track_global = TrackGlobal::new(RefCell::new(Default::default()));

    Box::into_raw(Box::new(track_global))
}

/// Frees a track global that was created with track_global_create
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_global_destroy(track_global: *mut TrackGlobal) {
    if track_global.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(track_global));
    }
}

/// Registers a property with the track global
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_global_register_property(
    track_global: *mut TrackGlobal,
    id: *const c_char,
    property: *mut ValuePropertyGlobal,
) -> bool {
    if track_global.is_null() || id.is_null() || property.is_null() {
        return false;
    }

    unsafe {
        let track_global = &mut *track_global;
        let id_str = CStr::from_ptr(id).to_string_lossy().into_owned();
        let property_global = Box::from_raw(property);

        track_global
            .borrow_mut()
            .register_property(id_str, *property_global);
        true
    }
}

/// Registers a path property with the track global
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_global_register_path_property(
    track_global: *mut TrackGlobal,
    id: *const c_char,
    property: *mut PathPropertyGlobal,
) -> bool {
    if track_global.is_null() || id.is_null() || property.is_null() {
        return false;
    }

    unsafe {
        let track_global = &mut *track_global;
        let id_str = CStr::from_ptr(id).to_string_lossy().into_owned();
        let property_global = Box::from_raw(property);

        track_global
            .borrow_mut()
            .register_path_property(id_str, *property_global);
        true
    }
}

/// Gets a property from track global by ID. Returns null if not found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_global_get_property(
    track_global: *const TrackGlobal,
    id: *const c_char,
) -> *const ValueProperty {
    if track_global.is_null() || id.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_global = &*track_global;
        let id_str = CStr::from_ptr(id).to_string_lossy();

        match track_global.borrow().get_property(&id_str) {
            Some(property) => Rc::as_ptr(property) as *const ValueProperty,
            None => ptr::null(),
        }
    }
}

/// Gets a path property from track global by ID. Returns null if not found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_global_get_path_property(
    track_global: *const TrackGlobal,
    id: *const c_char,
) -> *const PathProperty {
    if track_global.is_null() || id.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_global = &*track_global;
        let id_str = CStr::from_ptr(id).to_string_lossy();

        match track_global.borrow().get_path_property(&id_str) {
            Some(property) => Rc::as_ptr(property) as *const PathProperty,
            None => ptr::null(),
        }
    }
}
