use crate::animation::events::{EventData, EventType};
use crate::animation::timeline_coroutine_manager::{EventId, TimelineCoroutineManager};
use crate::animation::tracks_holder::TracksHolder;
use crate::base_provider_context::BaseProviderContext;
use crate::ffi::event_data::{CEventType, c_event_type_to_rust};
use crate::ffi::property::CValueNullable;
use crate::ffi::track::TrackKeyFFI;

/// Identifies an event added with `timeline_add_event`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CEventId {
    pub track_key: TrackKeyFFI,
    pub id: u64,
}

impl CEventId {
    /// Returned when an event could not be added. Never a valid id.
    fn invalid() -> Self {
        Self {
            track_key: TrackKeyFFI::null(),
            id: u64::MAX,
        }
    }
}

impl From<EventId> for CEventId {
    fn from(id: EventId) -> Self {
        Self {
            track_key: id.0.into(),
            id: id.1,
        }
    }
}

impl From<CEventId> for EventId {
    fn from(id: CEventId) -> Self {
        EventId(id.track_key.into(), id.id)
    }
}

/// Creates a new, empty `TimelineCoroutineManager` and returns a raw pointer to it.
/// The caller is responsible for freeing the memory using `destroy_timeline_coroutine_manager`.
#[unsafe(no_mangle)]
pub extern "C" fn create_timeline_coroutine_manager() -> *mut TimelineCoroutineManager {
    Box::into_raw(Box::new(TimelineCoroutineManager::new()))
}

/// Destroys a `TimelineCoroutineManager` instance, freeing its memory.
///
/// # Safety
/// - `manager` must be a pointer previously returned by `create_timeline_coroutine_manager` and not already freed.
/// - Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_timeline_coroutine_manager(
    manager: *mut TimelineCoroutineManager,
) {
    if manager.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(manager);
    }
}

/// Adds an event to the timeline and returns its id, to remove it with `timeline_remove_event`.
/// Events can be added in any order, at any time.
/// When two events on the same property start at the same time, the one added later wins.
/// Returns an invalid id (null track key, `id == UINT64_MAX`) if a pointer is null.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
/// - `event_data` must be a pointer returned by `event_data_to_rust`. The data is cloned, so the caller retains ownership.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_add_event(
    manager: *mut TimelineCoroutineManager,
    bpm: f32,
    event_data: *const EventData,
) -> CEventId {
    if manager.is_null() || event_data.is_null() {
        return CEventId::invalid();
    }

    unsafe { (*manager).add_event(bpm, (*event_data).clone()).into() }
}

/// Removes the event with `id`, as returned by `timeline_add_event`.
/// Returns false if it was already removed or never existed.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_remove_event(
    manager: *mut TimelineCoroutineManager,
    id: CEventId,
) -> bool {
    if manager.is_null() {
        return false;
    }

    unsafe { (*manager).remove_event(id.into()) }
}

/// Removes every event on every property of `track_key` that starts in `start_song_time..=end_song_time`.
/// Returns how many events were removed.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_remove_events_between(
    manager: *mut TimelineCoroutineManager,
    track_key: TrackKeyFFI,
    start_song_time: f32,
    end_song_time: f32,
) -> u32 {
    if manager.is_null() {
        return 0;
    }

    unsafe {
        (*manager).remove_events_between(
            track_key.into(),
            start_song_time.into(),
            end_song_time.into(),
        ) as u32
    }
}

/// Removes every event. Properties stay as they are until something else writes them.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_clear(manager: *mut TimelineCoroutineManager) {
    if manager.is_null() {
        return;
    }

    unsafe { (*manager).clear() }
}

/// Writes the state at `song_time` into every property that has events.
/// Can be called with any time, in any order, so it can be used to seek backwards.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `tracks_holder` must be a valid pointer to a `TracksHolder` containing every track used by the events.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_apply(
    manager: *const TimelineCoroutineManager,
    song_time: f32,
    context: *const BaseProviderContext,
    tracks_holder: *mut TracksHolder,
) {
    if manager.is_null() || context.is_null() || tracks_holder.is_null() {
        return;
    }

    unsafe {
        (*manager).apply(song_time.into(), &*context, &mut *tracks_holder);
    }
}

/// The blend state of a path property at a song time. See `timeline_path_blend`.
#[repr(C)]
#[derive(Default)]
pub struct CPathBlend {
    /// Whether any event ever targets this path property. The other fields are only meaningful if true.
    pub exists: bool,
    /// Whether a previous path is still being blended from.
    pub has_prev_point: bool,
    /// Whether there is an active path. False before the first event or after a null event.
    pub has_point: bool,
    /// Eased blend from the previous path to the active one.
    pub interpolate_time: f32,
}

/// Value of an animated property at `song_time`, without writing to any track.
/// Returns a value with `has_value == false` if no event is active, the active event has no
/// point data, `property` is not an `AnimateTrack` event type, or a pointer is null.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
/// - `property` must be a valid pointer to a `CEventType` with a valid property id (see `c_event_type_to_rust`).
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_value_at(
    manager: *const TimelineCoroutineManager,
    song_time: f32,
    track_key: TrackKeyFFI,
    property: *const CEventType,
    context: *const BaseProviderContext,
) -> CValueNullable {
    if manager.is_null() || property.is_null() || context.is_null() {
        return CValueNullable::default();
    }

    unsafe {
        let EventType::AnimateTrack(handle) = c_event_type_to_rust(&*property) else {
            return CValueNullable::default();
        };

        (*manager)
            .value_at(song_time.into(), track_key.into(), &handle, &*context)
            .into()
    }
}

/// Samples a path property at `song_time` (the state of the blend) at the object's lifetime `path_time`.
/// Works the same as `path_property_interpolate`. Returns a value with `has_value == false` if there is
/// no active path, `property` is not an `AssignPathAnimation` event type, or a pointer is null.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
/// - `property` must be a valid pointer to a `CEventType` with a valid property id (see `c_event_type_to_rust`).
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_path_interpolate(
    manager: *const TimelineCoroutineManager,
    song_time: f32,
    track_key: TrackKeyFFI,
    property: *const CEventType,
    path_time: f32,
    context: *const BaseProviderContext,
) -> CValueNullable {
    if manager.is_null() || property.is_null() || context.is_null() {
        return CValueNullable::default();
    }

    unsafe {
        let EventType::AssignPathAnimation(handle) = c_event_type_to_rust(&*property) else {
            return CValueNullable::default();
        };

        (*manager)
            .path_at(song_time.into(), track_key.into(), &handle)
            .and_then(|snapshot| snapshot.interpolate(path_time, &*context))
            .into()
    }
}

/// Blend state of a path property at `song_time`. `exists` is false if no event ever targets
/// the property, `property` is not an `AssignPathAnimation` event type, or a pointer is null.
///
/// # Safety
/// - `manager` must be a valid pointer to a `TimelineCoroutineManager`.
/// - `property` must be a valid pointer to a `CEventType` with a valid property id (see `c_event_type_to_rust`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timeline_path_blend(
    manager: *const TimelineCoroutineManager,
    song_time: f32,
    track_key: TrackKeyFFI,
    property: *const CEventType,
) -> CPathBlend {
    if manager.is_null() || property.is_null() {
        return CPathBlend::default();
    }

    unsafe {
        let EventType::AssignPathAnimation(handle) = c_event_type_to_rust(&*property) else {
            return CPathBlend::default();
        };

        match (*manager).path_at(song_time.into(), track_key.into(), &handle) {
            Some(snapshot) => CPathBlend {
                exists: true,
                has_prev_point: snapshot.prev_point.is_some(),
                has_point: snapshot.point.is_some(),
                interpolate_time: snapshot.interpolate_time,
            },
            None => CPathBlend::default(),
        }
    }
}
