use crate::{
    base_provider_context::BaseProviderContext,
    ffi::{
        json::{self, FFIJsonValue},
        types::WrapVec4,
    },
    point_definition::{PointDefinitionLike, Vector4PointDefinition},
};

#[repr(C)]
pub struct Vector4InterpolationResult {
    pub value: WrapVec4,
    pub is_last: bool,
}

/// VECTOR4 POINT DEFINITION
///
/// # Safety
/// - `json` may be null; if non-null it must point to a valid `FFIJsonValue`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_vector4_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const Vector4PointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(Vector4PointDefinition::parse(value, unsafe {
        &mut *context
    }));

    (Box::leak(point_definition)) as _
}

/// Interpolate a Vector4 point definition at `time`.
///
/// # Safety
/// - `point_definition` must be a valid pointer to a `Vector4PointDefinition`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
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

/// # Safety
/// - `point_definition` must be a valid pointer to a `Vector4PointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_count(
    point_definition: *const Vector4PointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

/// # Safety
/// - `point_definition` must be a valid pointer to a `Vector4PointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_vector4_has_base_provider(
    point_definition: *const Vector4PointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

/// Interpolates a Vector4 point definition at each of `len` times, writing the values to `out`.
/// Per-element `is_last` flags are not reported. Null pointers or `len == 0` are a no-op.
///
/// # Safety
/// - `point_definition` must be a valid pointer to a `Vector4PointDefinition`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
/// - `times` must point to `len` readable floats, and `out` to `len` writable `WrapVec4`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_vector4_batch(
    point_definition: *const Vector4PointDefinition,
    times: *const f32,
    out: *mut WrapVec4,
    len: usize,
    context: *const BaseProviderContext,
) {
    unsafe { super::interpolate_batch(point_definition, times, out, len, context) }
}
