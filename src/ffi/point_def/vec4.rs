use crate::{
    base_provider_context::BaseProviderContext,
    ffi::{
        json::{self, FFIJsonValue},
        types::WrapVec4,
    },
    point_definition::{PointDefinition, vector4_point_definition::Vector4PointDefinition},
};

#[repr(C)]
pub struct Vector4InterpolationResult {
    pub value: WrapVec4,
    pub is_last: bool,
}

///VECTOR4 POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector4_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const Vector4PointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector4PointDefinition::new(value, unsafe { &*context }));

    (Box::leak(point_definition)) as _
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector4(
    point_definition: *const Vector4PointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> Vector4InterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    Vector4InterpolationResult {
        value: WrapVec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_count(
    point_definition: *const Vector4PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_has_base_provider(
    point_definition: *const Vector4PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
