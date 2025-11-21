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

///BASE POINT DEFINITION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_make_base_point_definition(
    json: *const FFIJsonValue,
    ty: WrapBaseValueType,
    context: *mut BaseProviderContext,
) -> *mut base_point_definition::BasePointDefinition {
    let value = unsafe { json::convert_json_value_to_serde(json) };
    let context = unsafe { &*context };

    let point_definition: base_point_definition::BasePointDefinition = match ty {
        WrapBaseValueType::Vec3 => Vector3PointDefinition::new(value, context).into(),
        WrapBaseValueType::Quat => QuaternionPointDefinition::new(value, context).into(),
        WrapBaseValueType::Vec4 => Vector4PointDefinition::new(value, context).into(),
        WrapBaseValueType::Float => FloatPointDefinition::new(value, context).into(),
        WrapBaseValueType::Unknown => {
            panic!("Cannot create BasePointDefinition with Unknown type");
        }
    };

    let point_definition = Box::new(point_definition);

    (Box::leak(point_definition)) as _
}

///BASE POINT DEFINITION
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_count(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> usize {
    let point_definition = unsafe { &*point_definition };
    point_definition.get_count()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_base_point_definition_has_base_provider(
    point_definition: *const base_point_definition::BasePointDefinition,
) -> bool {
    let point_definition = unsafe { &*point_definition };
    point_definition.has_base_provider()
}
