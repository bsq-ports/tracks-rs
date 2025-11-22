use crate::{
    base_provider_context::BaseProviderContext,
    ffi::json::{self, FFIJsonValue},
    point_definition::{PointDefinition, float_point_definition::FloatPointDefinition},
};

#[repr(C)]
pub struct FloatInterpolationResult {
    pub value: f32,
    pub is_last: bool,
}

/// FLOAT POINT DEFINITION
///
/// # Safety
/// - `json` may be null; if non-null it must point to a valid `FFIJsonValue`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_float_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const FloatPointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(FloatPointDefinition::parse(value, unsafe { &*context }));

    (Box::leak(point_definition)) as _
}

/// Interpolate a float point definition at `time`.
///
/// # Safety
/// - `point_definition` must be a valid pointer to a `FloatPointDefinition`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_float(
    point_definition: *const FloatPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> FloatInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    FloatInterpolationResult { value, is_last }
}

/// # Safety
/// - `point_definition` must be a valid pointer to a `FloatPointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_count(
    point_definition: *const FloatPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

/// # Safety
/// - `point_definition` must be a valid pointer to a `FloatPointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_float_has_base_provider(
    point_definition: *const FloatPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
