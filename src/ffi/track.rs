use crate::animation::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
    tracks::{PropertyNames, Track},
};
use std::{
    ffi::{CStr, CString, c_char},
    ptr,
};

#[repr(C)]
pub struct CPropertiesMap {
    // using the pointers reduces the size of the struct
    // to 112 bytes from 312 bytes

    // Noodle
    pub position: *const ValueProperty,
    pub rotation: *const ValueProperty,
    pub scale: *const ValueProperty,
    pub local_rotation: *const ValueProperty,
    pub local_position: *const ValueProperty,
    pub dissolve: *const ValueProperty,
    pub dissolve_arrow: *const ValueProperty,
    pub time: *const ValueProperty,
    pub cuttable: *const ValueProperty,

    // Chroma
    pub color: *const ValueProperty,
    pub attentuation: *const ValueProperty,       // PropertyType::linear
    pub fog_offset: *const ValueProperty,         // PropertyType::linear
    pub height_fog_start_y: *const ValueProperty, // PropertyType::linear
    pub height_fog_height: *const ValueProperty,  // PropertyType::linear
}

#[repr(C)]
pub struct CPathPropertiesMap<'a> {
    pub position: *mut PathProperty<'a>,
    pub rotation: *mut PathProperty<'a>,
    pub scale: *mut PathProperty<'a>,
    pub local_rotation: *mut PathProperty<'a>,
    pub local_position: *mut PathProperty<'a>,
    pub definite_position: *mut PathProperty<'a>,
    pub dissolve: *mut PathProperty<'a>,
    pub dissolve_arrow: *mut PathProperty<'a>,
    pub cuttable: *mut PathProperty<'a>,
    pub color: *mut PathProperty<'a>,
}

impl Default for CPropertiesMap {
    fn default() -> Self {
        CPropertiesMap {
            position: ptr::null(),
            rotation: ptr::null(),
            scale: ptr::null(),
            local_rotation: ptr::null(),
            local_position: ptr::null(),
            dissolve: ptr::null(),
            dissolve_arrow: ptr::null(),
            time: ptr::null(),
            cuttable: ptr::null(),
            color: ptr::null(),
            attentuation: ptr::null(),
            fog_offset: ptr::null(),
            height_fog_start_y: ptr::null(),
            height_fog_height: ptr::null(),
        }
    }
}

impl Default for CPathPropertiesMap<'_> {
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
pub extern "C" fn track_create() -> *mut Track<'static> {
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
pub unsafe extern "C" fn track_register_game_object(
    track: *mut Track,
    game_object: *mut GameObject,
) {
    if track.is_null() || game_object.is_null() {
        return;
    }

    unsafe {
        let game_object_clone = (*game_object).clone();
        (*track).register_game_object(game_object_clone);
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
            let property_clone = *property;
            (*track).register_property(str_id.to_string(), property_clone);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property(
    track: *const Track,
    id: *const c_char,
) -> *const ValueProperty {
    if track.is_null() || id.is_null() {
        return ptr::null();
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            match (*track).get_property(str_id) {
                Some(property) => property,
                None => ptr::null(),
            }
        } else {
            ptr::null()
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_property_by_name(
    track: *const Track,
    id: PropertyNames,
) -> *const ValueProperty {
    if track.is_null() {
        return ptr::null();
    }

    let track = unsafe { &*track };

    match track.properties.get_property_by_name(id) {
        Some(property) => property,
        None => ptr::null(),
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
    track: *mut Track<'a>,
    id: *const c_char,
    property: *mut PathProperty<'a>,
) {
    if track.is_null() || id.is_null() || property.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(id);
        if let Ok(str_id) = c_str.to_str() {
            let property_clone = (*property).clone();
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
pub unsafe extern "C" fn track_get_properties_map(track: *const Track) -> CPropertiesMap {
    if track.is_null() {
        return Default::default();
    }
    let track = unsafe { &*track };
    

    CPropertiesMap {
        position: &track.properties.position as *const ValueProperty,
        rotation: &track.properties.rotation as *const ValueProperty,
        scale: &track.properties.scale as *const ValueProperty,
        local_rotation: &track.properties.local_rotation as *const ValueProperty,
        local_position: &track.properties.local_position as *const ValueProperty,
        dissolve: &track.properties.dissolve as *const ValueProperty,
        dissolve_arrow: &track.properties.dissolve_arrow as *const ValueProperty,
        time: &track.properties.time as *const ValueProperty,
        cuttable: &track.properties.cuttable as *const ValueProperty,
        color: &track.properties.color as *const ValueProperty,
        attentuation: &track.properties.attentuation as *const ValueProperty,
        fog_offset: &track.properties.fog_offset as *const ValueProperty,
        height_fog_start_y: &track.properties.height_fog_start_y as *const ValueProperty,
        height_fog_height: &track.properties.height_fog_height as *const ValueProperty,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn track_get_path_properties_map(track: *mut Track<'_>) -> CPathPropertiesMap<'_> {
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