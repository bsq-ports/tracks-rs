use std::ffi::{CStr, c_char};
use std::ptr;

use crate::animation::events::{EventData, EventType};
use crate::animation::track::{PathPropertyHandle, ValuePropertyHandle};
use crate::easings::functions::Functions;
use crate::ffi::track::TrackKeyFFI;
use crate::point_definition::base_point_definition::BasePointDefinition;

// Type-safe enum for event types
#[repr(C)]
pub struct CEventData {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub event_type: CEventType,
    pub track_key: TrackKeyFFI,
    pub point_data_ptr: *mut BasePointDefinition,
}

#[repr(u32)]
pub enum CEventTypeEnum {
    AnimateTrack = 0,
    AssignPathAnimation = 1,
}

#[repr(C)]
pub struct CEventType {
    pub ty: CEventTypeEnum,
    pub property: *const c_char,
}

/// Converts a `CEventData` into a Rust `EventData`.
/// Does not consume the input struct; returns an owned pointer to a newly allocated `EventData`.
///
/// # Safety
/// - `c_event_data` must be a valid, non-null pointer to a `CEventData`.
/// - Any C strings referenced inside `c_event_data` must be valid null-terminated pointers.
/// - The returned pointer is owned by the caller and must be freed by calling `event_data_dispose`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_data_to_rust(c_event_data: *const CEventData) -> *mut EventData {
    if c_event_data.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        let c_event_data = &*c_event_data;

        let event_type = match c_event_data.event_type.ty {
            CEventTypeEnum::AnimateTrack => {
                let value_property = CStr::from_ptr(c_event_data.event_type.property);
                let value_property_handle =
                    ValuePropertyHandle::new(value_property.to_str().unwrap());

                EventType::AnimateTrack(value_property_handle)
            }
            CEventTypeEnum::AssignPathAnimation => {
                let path_property = CStr::from_ptr(c_event_data.event_type.property);
                let path_property_handle = PathPropertyHandle::new(path_property.to_str().unwrap());

                EventType::AssignPathAnimation(path_property_handle)
            }
        };
        let track_key = c_event_data.track_key;
        let point_data = if c_event_data.point_data_ptr.is_null() {
            None
        } else {
            Some(std::mem::take(&mut *c_event_data.point_data_ptr))
        };

        let event_data = EventData {
            raw_duration: c_event_data.raw_duration,
            easing: c_event_data.easing,
            repeat: c_event_data.repeat,
            start_time: c_event_data.start_time,
            track_key: track_key.into(),
            point_data,
            property: event_type,
        };
        Box::into_raw(Box::new(event_data))
    }
}

/// Dispose of an `EventData` previously returned by `event_data_to_rust`.
///
/// # Safety
/// - `event_data` must be a pointer previously returned by `event_data_to_rust` and not already freed.
/// - Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_data_dispose(event_data: *mut EventData) {
    if event_data.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(event_data);
    }
}
