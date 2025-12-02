use crate::{
    base_provider_context::BaseProviderContext,
    ffi::{
        json::{self, FFIJsonValue},
        types::{WrapBaseValue, WrapBaseValueType},
    },
    point_definition::{
        PointDefinition, base_point_definition, float_point_definition::FloatPointDefinition,
        quaternion_point_definition::QuaternionPointDefinition,
        vector3_point_definition::Vector3PointDefinition,
        vector4_point_definition::Vector4PointDefinition,
    },
};

/// BASE POINT DEFINITION
///
/// # Safety
/// - `json` must be a valid pointer to an `FFIJsonValue` or null if not used by the specific constructor.
/// - `context` must be a valid, non-null pointer to a live `BaseProviderContext` for the duration of this call.
/// - The returned pointer is owned by the caller and must be freed by calling `base_point_definition_free`.
/// - This function may panic on invalid input; unwinding across the FFI boundary is undefined behaviour.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_point_definition(
    json: *const FFIJsonValue,
    ty: WrapBaseValueType,
    context: *mut BaseProviderContext,
) -> *mut base_point_definition::BasePointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let context = unsafe { &*context };

    let point_definition: base_point_definition::BasePointDefinition = match ty {
        WrapBaseValueType::Vec3 => Vector3PointDefinition::parse(value, context).into(),
        WrapBaseValueType::Quat => QuaternionPointDefinition::parse(value, context).into(),
        WrapBaseValueType::Vec4 => Vector4PointDefinition::parse(value, context).into(),
        WrapBaseValueType::Float => FloatPointDefinition::parse(value, context).into(),
        WrapBaseValueType::Unknown => {
            panic!("Cannot create BasePointDefinition with Unknown type");
        }
    };

    let point_definition = Box::new(point_definition);

    (Box::leak(point_definition)) as _
}

/// BASE POINT DEFINITION FREE
///
/// # Safety
/// - `point_definition` must be a pointer previously returned by `tracks_make_base_point_definition`.
/// - After calling this function the pointer is invalid and must not be used again.
/// - Passing a null pointer is allowed and is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn base_point_definition_free(
    point_definition: *mut base_point_definition::BasePointDefinition,
) {
    if point_definition.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw(point_definition);
    };
}

/// Interpolate a base point definition at a given time.
///
/// # Safety
/// - `point_definition` must be a valid, non-null pointer to a `BasePointDefinition`.
/// - `is_last_out` must be a valid, non-null pointer to a `bool` to receive the `is_last` flag.
/// - `context` must be a valid pointer to `BaseProviderContext` for the duration of the call.
/// - This function does not take ownership of any pointers passed in.
/// - Do not rely on the returned `WrapBaseValue` containing any borrowed references; it is an owned value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_interpolate_base_point_definition(
    point_definition: *const base_point_definition::BasePointDefinition,
    time: f32,
    is_last_out: *mut bool,
    context: *mut BaseProviderContext,
) -> WrapBaseValue {
    let point_definition = unsafe { &*point_definition };
    let (value, is_last) = point_definition.interpolate(time, unsafe { &*context });
    unsafe { *is_last_out = is_last };

    WrapBaseValue::from(value)
}

/// Return number of points in the point definition.
///
/// Safety:
/// - `point_definition` must be a valid, non-null pointer to a `BasePointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_count(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

/// Check whether the point definition references a base provider.
///
/// Safety:
/// - `point_definition` must be a valid, non-null pointer to a `BasePointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_has_base_provider(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}

/// Get the `WrapBaseValueType` of the point definition.
/// Safety:
/// - `point_definition` must be a valid, non-null pointer to a `BasePointDefinition`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_get_type(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> WrapBaseValueType {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_type()
}
