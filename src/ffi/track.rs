use crate::animation::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
    tracks::Track,
};
use std::{
    ffi::{CStr, CString, c_char},
    ptr,
};

#[unsafe(no_mangle)]
pub extern "C" fn track_create() -> *mut Track<'static> {
    let track = Track::default();
    Box::into_raw(Box::new(track))
}

/// Consumes the track and frees its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_destroy(track: *mut Track) {
    if !track.is_null() {
        unsafe {
            drop(Box::from_raw(track));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_set_name(track: *mut Track, name: *const c_char) {
    if track.is_null() || name.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(name);
        if let Ok(str_name) = c_str.to_str() {
            (*track).name = str_name.to_string();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_name(track: *const Track) -> *const c_char {
    if track.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_ref = &*track;
        match CString::new(track_ref.name.clone()) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_game_object(
    track: *mut Track,
    game_object: *mut GameObject,
) {
    if track.is_null() || game_object.is_null() {
        return;
    }

    unsafe {
        let game_object_clone = (*game_object).clone();
        (*track).register_game_object(game_object_clone);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_property(
    track: *mut Track,
    id: *const c_char,
    property: *mut ValueProperty,
) {
    if track.is_null() || id.is_null() || property.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            let property_clone = (*property).clone();
            (*track).register_property(str_id.to_string(), property_clone);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property(
    track: *const Track,
    id: *const c_char,
) -> *const ValueProperty {
    if track.is_null() || id.is_null() {
        return ptr::null();
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            match (*track).get_property(str_id) {
                Some(property) => property,
                None => ptr::null(),
            }
        } else {
            ptr::null()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_property(
    track: *mut Track,
    id: *const c_char,
) -> *mut PathProperty {
    if track.is_null() || id.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        let Ok(str_id) = c_str.to_str() else {
            return ptr::null_mut();
        };
        match (*track).get_mut_path_property(str_id) {
            Some(property) => property,
            None => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_mark_updated(track: *mut Track) {
    if !track.is_null() {
        unsafe {
            (*track).mark_updated();
        }
    }
}
