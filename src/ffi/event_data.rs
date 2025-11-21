use std::ptr;

use crate::animation::events::{EventData, EventType};
use crate::animation::property::{PathProperty, ValueProperty};
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
pub union CEventTypeData {
    /// AnimateTrack(ValueProperty)
    pub property: *mut ValueProperty,
    /// AssignPathAnimation(PathProperty)
    pub path_property: *mut PathProperty,
}
#[repr(C)]
pub struct CEventType {
    pub ty: CEventTypeEnum,
    pub data: CEventTypeData,
}

/// Converts a CEventData into a Rust EventData
/// Does not consume the CEventData
/// Returns a raw pointer to the Rust EventData
#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_data_to_rust<'a>(
    c_event_data: *const CEventData,
) -> *mut EventData<'a> {
    if c_event_data.is_null() {
        return ptr::null_mut();
    }
    unsafe {
        let c_event_data = &*c_event_data;

        let event_type = match c_event_data.event_type.ty {
            CEventTypeEnum::AnimateTrack => {
                let value_property = &mut *(c_event_data.event_type.data.property);
                EventType::AnimateTrack(value_property)
            }
            CEventTypeEnum::AssignPathAnimation => {
                let path_property = &mut *(c_event_data.event_type.data.path_property);
                EventType::AssignPathAnimation(path_property)
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn event_data_dispose(event_data: *mut EventData<'_>) {
    if event_data.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(event_data);
    }
}
