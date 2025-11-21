use crate::{
    base_provider_context::BaseProviderContext,
    ffi::{
        json::{self, FFIJsonValue},
        types::WrapQuat,
    },
    point_definition::{PointDefinition, quaternion_point_definition::QuaternionPointDefinition},
};

#[repr(C)]
pub struct QuaternionInterpolationResult {
    pub value: WrapQuat,
    pub is_last: bool,
}

/// QUATERNION POINT DEFINITION
///
/// # Safety
/// - `json` may be null; if non-null it must point to a valid `FFIJsonValue`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_quat_point_definition(
    json: *const FFIJsonValue,
    context: *mut BaseProviderContext,
) -> *const QuaternionPointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let point_definition = Box::new(QuaternionPointDefinition::new(value, unsafe { &*context }));

    (Box::leak(point_definition)) as _
}

/// Interpolate a Quaternion point definition at `time`.
///
/// # Safety
/// - `point_definition` must be a valid pointer to a `QuaternionPointDefinition`.
/// - `context` must be a valid pointer to a `BaseProviderContext`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_quat(
    point_definition: *const QuaternionPointDefinition,
    time: f32,
    context: *mut BaseProviderContext,
) -> QuaternionInterpolationResult {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    QuaternionInterpolationResult {
        value: WrapQuat {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        },
        is_last,
    }
}

/// # Safety
/// - `point_definition` must be a valid pointer to a `QuaternionPointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_count(
    point_definition: *const QuaternionPointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

/// # Safety
/// - `point_definition` must be a valid pointer to a `QuaternionPointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_quat_has_base_provider(
    point_definition: *const QuaternionPointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
