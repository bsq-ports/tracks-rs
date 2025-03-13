use crate::animation::coroutine_manager::CoroutineManager;
use crate::animation::tracks::Track;
use crate::context::TracksContext;
use crate::point_definition::BasePointDefinition;
use crate::values::base_provider_context::BaseProviderContext;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn tracks_context_create<'a>() -> *mut TracksContext<'a> {
    let context = TracksContext {
        tracks: Vec::new(),
        point_definitions: Vec::new(),
        coroutine_manager: Default::default(),
        base_providers: Default::default(),
    };

    Box::into_raw(Box::new(context))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_destroy(context: *mut TracksContext) {
    if !context.is_null() {
        unsafe {
            drop(Box::from_raw(context));
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_add_track<'a>(
    context: *mut TracksContext<'a>,
    track: *mut Track<'a>,
) {
    if context.is_null() || track.is_null() {
        return;
    }

    unsafe {
        let track_obj = Box::from_raw(track);
        (*context).add_track(*track_obj);
        // Don't drop the Box here, ownership is transferred to the context
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_add_point_definition(
    context: *mut TracksContext,
    point_def: *mut BasePointDefinition,
) {
    if context.is_null() || point_def.is_null() {
        return;
    }

    unsafe {
        let point_def_obj = Box::from_raw(point_def);
        (*context).add_point_definition(*point_def_obj);
        // Don't drop the Box here, ownership is transferred to the context
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_track_by_name<'a>(
    context: *mut TracksContext<'a>,
    name: *const c_char,
) -> *mut Track<'a> {
    if context.is_null() || name.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let name_str = CStr::from_ptr(name).to_str().unwrap_or_default();
        match (*context).get_track_by_name(name_str) {
            Some(track) => track as *mut _,
            None => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_track<'a>(
    context: *mut TracksContext<'a>,
    index: usize,
) -> *mut Track<'a> {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        match (*context).get_track(index) {
            Some(track) => track as *mut _,
            None => ptr::null_mut(),
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_coroutine_manager<'a>(
    context: *const TracksContext<'a>,
) -> *const CoroutineManager<'a> {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Return a mutable pointer to the coroutine manager
        &(*context).coroutine_manager as *const _
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_base_provider_context<'a>(
    context: *const TracksContext<'a>,
) -> *const BaseProviderContext {
    if context.is_null() {
        return ptr::null();
    }

    unsafe {
        // Return a const pointer to the base provider context
        (*context).get_base_provider_context() as *const _
    }
}