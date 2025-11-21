use slotmap::{Key, KeyData};

use crate::animation::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
    track::{PropertyNames, Track},
    tracks_holder::TrackKey,
};
use std::{
    ffi::{CStr, CString, c_char},
    ptr,
    rc::Rc,
};

// C-compatible callback function type for game object modifications
// Parameters: game_object, was_added (true for added, false for removed), user_data
pub type CGameObjectCallback = extern "C" fn(GameObject, bool, *mut std::ffi::c_void);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrackKeyFFI(u64);

impl TrackKeyFFI {
    pub fn null() -> Self {
        TrackKey::null().into()
    }
}

impl From<TrackKeyFFI> for TrackKey {
    fn from(ffi_key: TrackKeyFFI) -> Self {
        TrackKey::from(KeyData::from_ffi(ffi_key.0))
    }
}

impl From<TrackKey> for TrackKeyFFI {
    fn from(key: TrackKey) -> Self {
        TrackKeyFFI(key.data().as_ffi())
    }
}

#[repr(C)]
pub struct CPropertiesMap {
    // using the pointers reduces the size of the struct
    // to 112 bytes from 312 bytes

    // Noodle
    pub position: *mut ValueProperty,
    pub rotation: *mut ValueProperty,
    pub scale: *mut ValueProperty,
    pub local_rotation: *mut ValueProperty,
    pub local_position: *mut ValueProperty,
    pub dissolve: *mut ValueProperty,
    pub dissolve_arrow: *mut ValueProperty,
    pub time: *mut ValueProperty,
    pub cuttable: *mut ValueProperty,

    // Chroma
    pub color: *mut ValueProperty,
    pub attentuation: *mut ValueProperty, // PropertyType::linear
    pub fog_offset: *mut ValueProperty,   // PropertyType::linear
    pub height_fog_start_y: *mut ValueProperty, // PropertyType::linear
    pub height_fog_height: *mut ValueProperty, // PropertyType::linear
}

#[repr(C)]
pub struct CPathPropertiesMap {
    pub position: *mut PathProperty,
    pub rotation: *mut PathProperty,
    pub scale: *mut PathProperty,
    pub local_rotation: *mut PathProperty,
    pub local_position: *mut PathProperty,
    pub definite_position: *mut PathProperty,
    pub dissolve: *mut PathProperty,
    pub dissolve_arrow: *mut PathProperty,
    pub cuttable: *mut PathProperty,
    pub color: *mut PathProperty,
}

impl Default for CPropertiesMap {
    fn default() -> Self {
        CPropertiesMap {
            position: ptr::null_mut(),
            rotation: ptr::null_mut(),
            scale: ptr::null_mut(),
            local_rotation: ptr::null_mut(),
            local_position: ptr::null_mut(),
            dissolve: ptr::null_mut(),
            dissolve_arrow: ptr::null_mut(),
            time: ptr::null_mut(),
            cuttable: ptr::null_mut(),
            color: ptr::null_mut(),
            attentuation: ptr::null_mut(),
            fog_offset: ptr::null_mut(),
            height_fog_start_y: ptr::null_mut(),
            height_fog_height: ptr::null_mut(),
        }
    }
}

impl Default for CPathPropertiesMap {
    fn default() -> Self {
        CPathPropertiesMap {
            position: ptr::null_mut(),
            rotation: ptr::null_mut(),
            scale: ptr::null_mut(),
            local_rotation: ptr::null_mut(),
            local_position: ptr::null_mut(),
            definite_position: ptr::null_mut(),
            dissolve: ptr::null_mut(),
            dissolve_arrow: ptr::null_mut(),
            cuttable: ptr::null_mut(),
            color: ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn track_create() -> *mut Track {
    let track = Track::default();
    Box::into_raw(Box::new(track))
}

/// Consumes the track and frees its memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_destroy(track: *mut Track) {
    if !track.is_null() {
        unsafe {
            drop(Box::from_raw(track));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_reset(track: *mut Track) {
    if track.is_null() {
        return;
    }

    unsafe {
        let track_ref = &mut *track;
        track_ref.reset();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_set_name(track: *mut Track, name: *const c_char) {
    if track.is_null() || name.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(name);
        if let Ok(str_name) = c_str.to_str() {
            (*track).name = str_name.to_string();
        }
    }
}

/// Returns the name of the track as a C string.
/// This leaks memory
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_name(track: *const Track) -> *const c_char {
    if track.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_ref = &*track;
        match CString::new(track_ref.name.clone()) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_game_object(track: *mut Track, game_object: GameObject) {
    if track.is_null() {
        return;
    }

    unsafe {
        (*track).register_game_object(game_object);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_unregister_game_object(track: *mut Track, game_object: GameObject) {
    if track.is_null() {
        return;
    }

    unsafe {
        (*track).remove_game_object(&game_object);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_game_objects(
    track: *const Track,
    size: *mut usize,
) -> *const GameObject {
    if track.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_ref = &*track;

        *size = track_ref.game_objects.len();

        track_ref.game_objects.as_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_property(
    track: *mut Track,
    id: *const c_char,
    property: *mut ValueProperty,
) {
    if track.is_null() || id.is_null() || property.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            let property_clone = (*property).clone();
            (*track).register_property(str_id.to_string(), property_clone);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property(
    track: *mut Track,
    id: *const c_char,
) -> *mut ValueProperty {
    if track.is_null() || id.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        let Ok(str_id) = c_str.to_str() else {
            return ptr::null_mut();
        };
        match (*track).get_mut_property(str_id) {
            Some(property) => property,
            None => ptr::null_mut(),
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property_by_name(
    track: *mut Track,
    id: PropertyNames,
) -> *mut ValueProperty {
    if track.is_null() {
        return ptr::null_mut();
    }

    let track = unsafe { &mut *track };

    match track.properties.get_property_by_name_mut(id) {
        Some(property) => property,
        None => ptr::null_mut(),
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_property_by_name(
    track: *mut Track,
    id: PropertyNames,
) -> *mut PathProperty {
    if track.is_null() {
        return ptr::null_mut();
    }

    let track = unsafe { &mut *track };

    match track.path_properties.get_property_by_name_mut(id) {
        Some(property) => property,
        None => ptr::null_mut(),
    }
}

// register path property
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_path_property<'a>(
    track: *mut Track,
    id: *const c_char,
    property: *mut PathProperty,
) {
    if track.is_null() || id.is_null() || property.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            let property_clone = std::mem::take(&mut *property);

            (*track).register_path_property(str_id.to_string(), property_clone);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_property(
    track: *mut Track,
    id: *const c_char,
) -> *mut PathProperty {
    if track.is_null() || id.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        let Ok(str_id) = c_str.to_str() else {
            return ptr::null_mut();
        };
        match (*track).get_mut_path_property(str_id) {
            Some(property) => property,
            None => ptr::null_mut(),
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_properties_map(track: *mut Track) -> CPropertiesMap {
    if track.is_null() {
        return Default::default();
    }
    let track = unsafe { &mut *track };

    CPropertiesMap {
        position: &mut track.properties.position as *mut ValueProperty,
        rotation: &mut track.properties.rotation as *mut ValueProperty,
        scale: &mut track.properties.scale as *mut ValueProperty,
        local_rotation: &mut track.properties.local_rotation as *mut ValueProperty,
        local_position: &mut track.properties.local_position as *mut ValueProperty,
        dissolve: &mut track.properties.dissolve as *mut ValueProperty,
        dissolve_arrow: &mut track.properties.dissolve_arrow as *mut ValueProperty,
        time: &mut track.properties.time as *mut ValueProperty,
        cuttable: &mut track.properties.cuttable as *mut ValueProperty,
        color: &mut track.properties.color as *mut ValueProperty,
        attentuation: &mut track.properties.attentuation as *mut ValueProperty,
        fog_offset: &mut track.properties.fog_offset as *mut ValueProperty,
        height_fog_start_y: &mut track.properties.height_fog_start_y as *mut ValueProperty,
        height_fog_height: &mut track.properties.height_fog_height as *mut ValueProperty,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_properties_map(track: *mut Track) -> CPathPropertiesMap {
    if track.is_null() {
        return Default::default();
    }
    let track = unsafe { &mut *track };

    CPathPropertiesMap {
        position: &mut track.path_properties.position as *mut PathProperty,
        rotation: &mut track.path_properties.rotation as *mut PathProperty,
        scale: &mut track.path_properties.scale as *mut PathProperty,
        local_rotation: &mut track.path_properties.local_rotation as *mut PathProperty,
        local_position: &mut track.path_properties.local_position as *mut PathProperty,
        definite_position: &mut track.path_properties.definite_position as *mut PathProperty,
        dissolve: &mut track.path_properties.dissolve as *mut PathProperty,
        dissolve_arrow: &mut track.path_properties.dissolve_arrow as *mut PathProperty,
        cuttable: &mut track.path_properties.cuttable as *mut PathProperty,
        color: &mut track.path_properties.color as *mut PathProperty,
    }
}

// FFI functions for per-track game object modification callbacks
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_register_game_object_callback(
    track: *mut Track,
    callback: CGameObjectCallback,
    user_data: *mut std::ffi::c_void,
) -> *const fn(GameObject, bool) {
    if track.is_null() {
        return ptr::null();
    }

    unsafe {
        let track_ref = &mut *track;
        // Create a closure that captures the C callback and user data
        let rust_callback = move |game_object: GameObject, was_added: bool| {
            callback(game_object, was_added, user_data);
        };

        let rc = Rc::new(rust_callback);

        track_ref.register_game_object_callback(rc.clone());

        Rc::into_raw(rc) as *const fn(GameObject, bool)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_remove_game_object_callback(
    track: *mut Track,
    callback: *const fn(GameObject, bool),
) {
    if track.is_null() {
        return;
    }

    unsafe {
        let track_ref = &mut *track;
        // Create a closure that matches the one we want to remove
        let rc: Rc<fn(GameObject, bool)> = Rc::from_raw(callback);

        track_ref.remove_game_object_callback(rc);
    }
}
