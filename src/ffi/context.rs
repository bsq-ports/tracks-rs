use crate::animation::coroutine_manager::CoroutineManager;
use crate::animation::tracks::Track;
use crate::base_provider_context::BaseProviderContext;
use crate::context::TracksContext;
use crate::point_definition::base_point_definition::{self};
use std::cell::RefCell;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut, Not};
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

use super::types::WrapBaseValueType;

#[unsafe(no_mangle)]
pub extern "C" fn tracks_context_create<'a>() -> *mut TracksContext<'a> {
    let context = TracksContext {
        tracks: Vec::new(),
        point_definitions: Default::default(),
        coroutine_manager: Default::default(),
        base_providers: Default::default(),
    };

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
pub unsafe extern "C" fn tracks_context_add_track<'a>(
    context: *mut TracksContext<'a>,
    track: *mut Track<'a>,
) -> *const Track<'a> {
    if context.is_null() || track.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_obj = Box::from_raw(track);
        let rc = Rc::new(RefCell::new(*track_obj));
        (*context).add_track(rc.clone());
        // Don't drop the Box here, ownership is transferred to the context

        rc.borrow().deref()
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
        let c_str = id
            .is_null()
            .not()
            .then(|| CStr::from_ptr(id).to_str().unwrap_or_default());

        let id_str = if c_str.is_some_and(|c| !c.is_empty()) {
            c_str.unwrap().to_string()
        } else {
            uuid::Uuid::new_v4().to_string()
        };

        let point_def_obj = Box::from_raw(point_def);
        let rc = Rc::new(*point_def_obj);
        (*context).add_point_definition(id_str.to_owned(), rc.clone());

        rc.as_ref()
        // Don't drop the Box here, ownership is transferred to the context
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
pub unsafe extern "C" fn tracks_context_get_track_by_name(
    context: *mut TracksContext<'_>,
    name: *const c_char,
) -> *mut Track<'_> {
    if context.is_null() || name.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let name_str = CStr::from_ptr(name).to_str().unwrap_or_default();
        match (*context).get_track_by_name(name_str) {
            Some(track) => track.borrow_mut().deref_mut() as *mut _,
            None => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_track(
    context: *mut TracksContext<'_>,
    index: usize,
) -> *mut Track<'_> {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        match (*context).get_track(index) {
            Some(track) => track.borrow_mut().deref_mut() as *mut _,
            None => ptr::null_mut(),
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_context_get_coroutine_manager(
    context: *mut TracksContext<'_>,
) -> *mut CoroutineManager<'_> {
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
    context: *mut TracksContext<'_>,
) -> *mut BaseProviderContext {
    if context.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        // Return a const pointer to the base provider context
        (*context).get_mut_base_provider_context() as *mut _
    }
}
