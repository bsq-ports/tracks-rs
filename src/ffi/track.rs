use std::{ffi::{c_char, CStr, CString}, ptr};
use crate::animation::{tracks::Track, game_object::GameObject, property::{ValueProperty, PathProperty}};

#[unsafe(no_mangle)]
pub extern "C" fn track_create() -> *mut Track<'static> {
    let track = Track::default();
    Box::into_raw(Box::new(track))
}

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
pub unsafe  extern "C" fn track_register_game_object(track: *mut Track, game_object: *mut GameObject) {
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
    property: *mut ValueProperty
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
pub unsafe extern "C" fn track_mark_updated(track: *mut Track) {
    if !track.is_null() {
        unsafe {
            (*track).mark_updated();
        }
    }
}