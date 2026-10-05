use crate::animation::events::{EventData, EventType};
use crate::animation::replay_based_coroutine_manager::ReplayBasedCoroutineManager;
use crate::animation::tracks_holder::TracksHolder;
use crate::base_provider_context::BaseProviderContext;
use crate::ffi::event_data::{CEventType, c_event_type_to_rust};
use crate::ffi::property::CValueNullable;
use crate::ffi::track::TrackKeyFFI;

/// Creates a new ReplayBasedCoroutineManager instance and returns a raw pointer to it.
/// The caller is responsible for freeing the memory using destroy_replay_coroutine_manager.
#[unsafe(no_mangle)]
pub extern "C" fn create_replay_coroutine_manager() -> *mut ReplayBasedCoroutineManager {
    Box::into_raw(Box::new(ReplayBasedCoroutineManager::new()))
}

/// Destroys a `ReplayBasedCoroutineManager` instance, freeing its memory.
///
/// # Safety
/// - `manager` must be a pointer previously returned by `create_replay_coroutine_manager` and not already freed.
/// - Passing a null pointer is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_replay_coroutine_manager(
    manager: *mut ReplayBasedCoroutineManager,
) {
    unsafe {
        if manager.is_null() {
            return;
        }
        let _ = Box::from_raw(manager);
    }
}

/// Adds an event to the manager's timeline.
///
/// # Safety
/// - `manager` must be a valid pointer to a `ReplayBasedCoroutineManager`.
/// - `event_data` must be a pointer returned by `event_data_to_rust`. The data is cloned, so the caller retains ownership.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replay_coroutine_manager_add_event(
    manager: *mut ReplayBasedCoroutineManager,
    bpm: f32,
    event_data: *const EventData,
) {
    if manager.is_null() || event_data.is_null() {
        return;
    }

    unsafe {
        (*manager).add_event(bpm, (*event_data).clone());
    }
}

/// Writes the state of every animated property and path property at `song_time` into the tracks.
/// `song_time` may move in any direction.
///
/// # Safety
/// - `manager` must be a valid pointer to a `ReplayBasedCoroutineManager`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `tracks_holder` must be a valid pointer to a `TracksHolder`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replay_coroutine_manager_apply(
    manager: *mut ReplayBasedCoroutineManager,
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

/// Returns the value of an `AnimateTrack` property at `song_time` without modifying any track.
/// Has no value if no event is active, or if `property` is not an `AnimateTrack` event type.
///
/// # Safety
/// - `manager` must be a valid pointer to a `ReplayBasedCoroutineManager`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `property` must be a valid pointer to a `CEventType`; C strings inside it must be null-terminated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replay_coroutine_manager_value_at(
    manager: *const ReplayBasedCoroutineManager,
    song_time: f32,
    context: *const BaseProviderContext,
    track_key: TrackKeyFFI,
    property: *const CEventType,
) -> CValueNullable {
    if manager.is_null() || context.is_null() || property.is_null() {
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

/// Samples an `AssignPathAnimation` path property at `path_time` (an object's lifetime) as it is at `song_time`,
/// without modifying any track.
/// Has no value if the path has no points, or if `property` is not an `AssignPathAnimation` event type.
///
/// # Safety
/// - `manager` must be a valid pointer to a `ReplayBasedCoroutineManager`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `property` must be a valid pointer to a `CEventType`; C strings inside it must be null-terminated.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn replay_coroutine_manager_path_value_at(
    manager: *const ReplayBasedCoroutineManager,
    song_time: f32,
    path_time: f32,
    context: *const BaseProviderContext,
    track_key: TrackKeyFFI,
    property: *const CEventType,
) -> CValueNullable {
    if manager.is_null() || context.is_null() || property.is_null() {
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
