use crate::{
    base_provider_context::BaseProviderContext,
    ffi::{
        json::{self, FFIJsonValue},
        types::WrapVec3,
    },
    point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition},
};

#[repr(C)]
pub struct Vector3InterpolationResult {
    pub value: WrapVec3,
    pub is_last: bool,
}

///VECTOR3 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector3_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const Vector3PointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector3PointDefinition::new(value, unsafe { &*context }));

    (Box::leak(point_definition)) as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector3(
    point_definition: *const Vector3PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector3InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector3InterpolationResult {
        value: WrapVec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_count(
    point_definition: *const Vector3PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector3_has_base_provider(
    point_definition: *const Vector3PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
