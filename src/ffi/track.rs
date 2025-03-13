use std::os::raw::c_int;
use std::ptr::null;
use std::{cell::RefCell, ffi::c_char, rc::Rc, time::Instant};

use crate::{
    animation::{
        game_object::GameObject,
        property::{PathProperty, ValueProperty},
        tracks::{Track, TrackGlobal},
    },
    ffi::types::{RcCRefCell, WrapBaseValue},
    values::value::BaseValue,
};

/// Create a new empty track
#[unsafe(no_mangle)]
pub extern "C" fn track_create() -> *const Track {
    let track = Track::default();
    Box::leak(Box::new(track))
    // let rc = Rc::new(RefCell::new(track));
    // rc.into()
}

pub unsafe extern "C" fn track_free(track: *mut Track) {
    if track.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(track);
    }
}

pub unsafe extern "C" fn track_into_global(track: *mut Track) -> RcCRefCell<Track> {
    if track.is_null() {
        return RcCRefCell::null();
    }

    let track = unsafe { Box::from_raw(track) };
    Rc::new(RefCell::new(Box::into_inner(track))).into()
}

/// Free a track
#[unsafe(no_mangle)]
pub extern "C" fn track_global_dispose(track: RcCRefCell<Track>) {
    let _ = track.unleak();
    // Track will be dropped here
}

/// Register a value property
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_property(
    track: *mut Track,
    id: *const c_char,
    property: *mut ValueProperty,
) {
    if id.is_null() || property.is_null() || track.is_null() {
        return;
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id) }
        .to_string_lossy()
        .to_string();
    let property = unsafe { Box::from_raw(property) };

    let track = unsafe { &mut *track };
    track.register_property(id_str, *property);
}

/// Register a path property
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_path_property(
    track: *mut Track,
    id: *const c_char,
    property: *mut PathProperty,
) {
    if id.is_null() || property.is_null() || track.is_null() {
        return;
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id) }
        .to_string_lossy()
        .to_string();
    let property = unsafe { Box::from_raw(property) };

    let track = unsafe { &mut *track };
    track.register_path_property(id_str, *property);
}

/// Register a game object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_game_object(
    track: *mut Track,
    game_object: *mut GameObject,
) {
    if game_object.is_null() || track.is_null() {
        return;
    }

    let game_object = unsafe { Box::from_raw(game_object) };

    let track = unsafe { &mut *track };
    track.register_game_object(*game_object);
}

/// Remove a game object
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_remove_game_object(
    track: *mut Track,
    game_object: *const GameObject,
) {
    if game_object.is_null() || track.is_null() {
        return;
    }

    let game_object_ref = unsafe { &*game_object };

    let track = unsafe { &mut *track };
    track.remove_game_object(game_object_ref);
}

/// Mark the track as updated
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_mark_updated(track: *mut Track) {
    let track = unsafe { &mut *track };
    track.mark_updated();
}

/// Check if a property exists
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_has_property(track: *const Track, id: *const c_char) -> c_int {
    if id.is_null() || track.is_null() {
        return 0;
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id) }.to_string_lossy();

    let track = unsafe { &*track };
    let result = { track.get_property(&id_str).is_some() as c_int };

    result
}

/// Check if a path property exists
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_has_path_property(track: *const Track, id: *const c_char) -> c_int {
    if id.is_null() {
        return 0;
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id) }.to_string_lossy();

    let result = {
        let track = unsafe { &*track };
        track.get_path_property(&id_str).is_some() as c_int
    };

    result
}

/// Get a property
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property(
    track: *const Track,
    id: *const c_char,
) -> *const ValueProperty {
    if id.is_null() || track.is_null() {
        return std::ptr::null();
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id).to_string_lossy() };
    let track = unsafe { &*track };

    match track.get_property(&id_str) {
        Some(property) => property,
        None => std::ptr::null(),
    }
}

/// Get a path property
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_property(
    track: *const Track,
    id: *const c_char,
) -> *const PathProperty {
    if id.is_null() || track.is_null() {
        return std::ptr::null();
    }

    let id_str = unsafe { std::ffi::CStr::from_ptr(id).to_string_lossy() };
    let track = unsafe { &*track };

    match track.get_path_property(&id_str) {
        Some(property) => property,
        None => std::ptr::null(),
    }
}
