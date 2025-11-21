use slotmap::Key;

use crate::animation::coroutine_manager::CoroutineManager;
use crate::animation::track::Track;
use crate::animation::tracks_holder::TracksHolder;
use crate::base_provider_context::BaseProviderContext;
use crate::context::TracksContext;
use crate::ffi::track::TrackKeyFFI;
use crate::point_definition::base_point_definition::{self};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

use super::types::WrapBaseValueType;

#[unsafe(no_mangle)]
pub extern "C" fn tracks_context_create() -> *mut TracksContext {
    let context = TracksContext::default();

    Box::into_raw(Box::new(context))
}

/// Consumes the context and frees its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_destroy(context: *mut TracksContext) {
    if !context.is_null() {
        unsafe {
            drop(Box::from_raw(context));
        }
    }
}

/// Consumes the track and moves
/// it into the context. Returns a const pointer to the track.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_add_track(
    context: *mut TracksContext,
    track: *mut Track,
) -> TrackKeyFFI {
    if context.is_null() || track.is_null() {
        return TrackKeyFFI::null();
    }

    unsafe {
        let track_obj = Box::from_raw(track);
        (*context).tracks.add_track(*track_obj).into()
    }
}

/// Consumes the point definition and moves it into the context.
/// Returns a const pointer to the point definition.
///
/// If id is null/empty, generates a uuid for the point definition.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_add_point_definition(
    context: *mut TracksContext,
    id: *const c_char,
    point_def: *mut base_point_definition::BasePointDefinition,
) -> *const base_point_definition::BasePointDefinition {
    if context.is_null() || point_def.is_null() {
        return ptr::null();
    }

    unsafe {
        let c_str = if id.is_null() {
            None
        } else {
            CStr::from_ptr(id).to_str().ok()
        };

        let id_str = if let Some(c_str) = c_str
            && !c_str.is_empty()
        {
            c_str.to_string()
        } else {
            uuid::Uuid::new_v4().to_string()
        };

        let point_def_obj = Box::from_raw(point_def);
        let rc = Rc::new(*point_def_obj);
        (*context).add_point_definition(id_str.to_owned(), rc.clone());

        rc.as_ref()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_point_definition(
    context: *mut TracksContext,
    name: *const c_char,
    ty: WrapBaseValueType,
) -> *const base_point_definition::BasePointDefinition {
    if context.is_null() || name.is_null() {
        return ptr::null();
    }

    unsafe {
        let name_str = CStr::from_ptr(name).to_str().unwrap_or_default();
        match (*context).get_point_definition(name_str, ty) {
            Some(point_def) => point_def.as_ref(),
            None => ptr::null(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_track_key(
    context: *mut TracksContext,
    name: *const c_char,
) -> TrackKeyFFI {
    if context.is_null() || name.is_null() {
        return TrackKeyFFI::null();
    }

    unsafe {
        let name_str = CStr::from_ptr(name).to_str().unwrap_or_default();
        match (*context).tracks.get_track_key(name_str) {
            Some(key) => key.into(),
            None => TrackKeyFFI::null(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_track(
    context: *mut TracksContext,
    index: TrackKeyFFI,
) -> *mut Track {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        match (*context).tracks.get_track_mut(index.into()) {
            Some(track) => track as *mut _,
            None => ptr::null_mut(),
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_coroutine_manager(
    context: *mut TracksContext,
) -> *mut CoroutineManager {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Return a mutable pointer to the coroutine manager
        &mut (*context).coroutine_manager as *mut _
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_base_provider_context(
    context: *mut TracksContext,
) -> *mut BaseProviderContext {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Return a const pointer to the base provider context
        (*context).get_mut_base_provider_context() as *mut _
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_tracks_holder(
    context: *mut TracksContext,
) -> *mut TracksHolder {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Return a mutable pointer to the tracks holder
        &mut (*context).tracks as *mut _
    }
}
