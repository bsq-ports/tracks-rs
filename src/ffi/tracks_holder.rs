use crate::animation::track::Track;
use crate::animation::tracks_holder::TrackKey;
use crate::animation::tracks_holder::TracksHolder;
use crate::ffi::track::TrackKeyFFI;
use std::ffi::CStr;
use std::ptr;

/// Create a new `TracksHolder` and return a raw pointer to it.
#[unsafe(no_mangle)]
pub extern "C" fn tracks_holder_create() -> *mut TracksHolder {
    let h = TracksHolder::new();
    Box::into_raw(Box::new(h))
}

/// Destroy a `TracksHolder` previously returned by `tracks_holder_create`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_destroy(holder: *mut TracksHolder) {
    if holder.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(holder));
    }
}

/// Add a `Track` to the holder. Takes ownership of the `Track` pointer passed in.
/// Returns a `TrackKeyFFI` identifying the inserted track, or null-equivalent on error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_add_track(
    holder: *mut TracksHolder,
    track: *mut Track,
) -> TrackKeyFFI {
    if holder.is_null() || track.is_null() {
        return TrackKeyFFI::null();
    }

    let holder_ref = unsafe { &mut *holder };

    // take ownership of the track pointer
    let boxed = unsafe { Box::from_raw(track) };
    let track_val = *boxed; // move out of box

    let key: TrackKey = holder_ref.add_track(track_val);

    TrackKeyFFI::from(key)
}

/// Get an immutable pointer to a `Track` by `TrackKeyFFI`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_get_track(
    holder: *const TracksHolder,
    key: TrackKeyFFI,
) -> *const Track {
    if holder.is_null() {
        return ptr::null();
    }

    let holder_ref = unsafe { &*holder };
    let tk: TrackKey = key.into();

    match holder_ref.get_track(tk) {
        Some(t) => t as *const Track,
        None => ptr::null(),
    }
}

/// Get a mutable pointer to a `Track` by `TrackKeyFFI`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_get_track_mut(
    holder: *mut TracksHolder,
    key: TrackKeyFFI,
) -> *mut Track {
    if holder.is_null() {
        return ptr::null_mut();
    }

    let holder_ref = unsafe { &mut *holder };
    let tk: TrackKey = key.into();

    match holder_ref.get_track_mut(tk) {
        Some(t) => t as *mut Track,
        None => ptr::null_mut(),
    }
}

/// Look up a track by name and return a pointer to it (const).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_get_track_by_name(
    holder: *const TracksHolder,
    name: *const std::os::raw::c_char,
) -> *const Track {
    if holder.is_null() || name.is_null() {
        return ptr::null();
    }

    let holder_ref = unsafe { &*holder };
    let cstr = unsafe { CStr::from_ptr(name) };
    if let Ok(s) = cstr.to_str() {
        match holder_ref.get_track_by_name(s) {
            Some(t) => t as *const Track,
            None => ptr::null(),
        }
    } else {
        ptr::null()
    }
}

/// Get the `TrackKeyFFI` for a track with the given name, or null-equivalent if not found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_get_track_key(
    holder: *mut TracksHolder,
    name: *const std::os::raw::c_char,
) -> TrackKeyFFI {
    if holder.is_null() || name.is_null() {
        return TrackKeyFFI::null();
    }

    let holder_ref = unsafe { &mut *holder };
    let cstr = unsafe { CStr::from_ptr(name) };
    if let Ok(s) = cstr.to_str() {
        match holder_ref.get_track_key(s) {
            Some(k) => TrackKeyFFI::from(k),
            None => TrackKeyFFI::null(),
        }
    } else {
        TrackKeyFFI::null()
    }
}

/// Return number of tracks in the holder.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_holder_count(holder: *const TracksHolder) -> usize {
    if holder.is_null() {
        return 0;
    }

    let holder_ref = unsafe { &*holder };
    // SlotMap::len is available on the field; we access it via its internal field
    holder_ref.len()
}
