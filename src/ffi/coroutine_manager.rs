use crate::animation::coroutine_manager::CoroutineManager;
use crate::animation::events::EventData;
use crate::animation::tracks_holder::TracksHolder;
use crate::base_provider_context::BaseProviderContext;

// filepath: /Users/fern/Developer/tracks-rs/src/ffi/coroutine_manager.rs

/// Creates a new CoroutineManager instance and returns a raw pointer to it.
/// The caller is responsible for freeing the memory using destroy_coroutine_manager.
#[unsafe(no_mangle)]
pub extern "C" fn create_coroutine_manager() -> *mut CoroutineManager {
    let manager = Box::new(CoroutineManager::default());
    Box::into_raw(manager)
}

/// Destroys a CoroutineManager instance, freeing its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_coroutine_manager(manager: *mut CoroutineManager) {
    unsafe {
        if manager.is_null() {
            return;
        }
        let _ = Box::from_raw(manager);
    }
}

/// Starts an event coroutine in the manager. Consumes event_data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_event_coroutine(
    manager: *mut CoroutineManager,
    bpm: f32,
    song_time: f32,
    context: *const BaseProviderContext,
    tracks_holder: *mut TracksHolder,
    event_data: *mut EventData,
) {
    if manager.is_null() || context.is_null() || event_data.is_null() || tracks_holder.is_null() {
        return;
    }

    unsafe {
        let manager = &mut *manager;
        let context = &*context;
        let event_data = Box::from_raw(event_data);

        manager.start_event_coroutine(bpm, song_time, context, &mut *tracks_holder, *event_data);
    }
}

/// Polls all events in the manager, updating their state based on the current song time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn poll_events(
    manager: *mut CoroutineManager,
    song_time: f32,
    context: *const BaseProviderContext,
    tracks_holder: *mut TracksHolder,
) {
    if manager.is_null() || context.is_null() || tracks_holder.is_null() {
        return;
    }

    unsafe {
        let manager = &mut *manager;
        let context = &*context;

        manager.poll_events(song_time, context, &mut *tracks_holder);
    }
}
