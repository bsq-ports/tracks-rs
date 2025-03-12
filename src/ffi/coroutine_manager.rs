use std::ptr;
use crate::animation::coroutine_manager::CoroutineManager;
use crate::animation::events::EventData;
use crate::values::base_provider_context::BaseProviderContext;

/// Create a new coroutine manager.
#[unsafe(no_mangle)]
pub extern "C" fn coroutine_manager_new() -> *mut CoroutineManager {
    Box::into_raw(Box::new(CoroutineManager::default()))
}

/// Start an event coroutine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn coroutine_manager_start_event(
    manager: *mut CoroutineManager,
    bpm: f32,
    song_time: f32,
    context: *const BaseProviderContext,
    event_data: *mut EventData,
) {
    if manager.is_null() || context.is_null() || event_data.is_null() {
        return;
    }

    let manager = unsafe { &mut *manager };
    let context = unsafe { &*context };
    let event_data = unsafe { Box::from_raw(event_data) };

    manager.start_event_coroutine(bpm, song_time, context, *event_data);
}

/// Poll events in a coroutine manager.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn coroutine_manager_poll_events(
    manager: *mut CoroutineManager,
    song_time: f32,
    context: *const BaseProviderContext,
) {
    if manager.is_null() || context.is_null() {
        return;
    }

    // Take ownership of the CoroutineManager to call poll_events
    let manager = unsafe { Box::from_raw(manager) };
    let context = unsafe { &*context };

    // poll_events consumes self, so we don't need to put it back
    manager.poll_events(song_time, context);
}

/// Free a coroutine manager.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn coroutine_manager_free(manager: *mut CoroutineManager) {
    if manager.is_null() {
        return;
    }
    
    unsafe {
        // Convert the raw pointer back to a Box and drop it
        let _ = Box::from_raw(manager);
    }
}