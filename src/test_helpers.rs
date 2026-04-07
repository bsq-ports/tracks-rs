//! Collection of helper functions for tests. These are not intended to be used outside of tests, and may be removed or changed without a major version bump.


#[cfg(feature = "json")]
use serde_json::Value as JsonValue;

#[cfg(feature = "json")]
use crate::{point_definition::{FloatPointDefinition, Vector4PointDefinition, quaternion_point_definition, vector3_point_definition}, prelude::{BaseProviderContext, PointDefinitionLike}};


#[cfg(feature = "json")]
pub fn parse_float_point_definition(
    value: JsonValue,
    context: &mut BaseProviderContext,
) -> FloatPointDefinition {
    <FloatPointDefinition as PointDefinitionLike<f32>>::parse(value, context)
}

#[cfg(feature = "json")]
pub fn parse_vector3_point_definition(
    value: JsonValue,
    context: &mut BaseProviderContext,
) -> vector3_point_definition::Vector3PointDefinition {
    <vector3_point_definition::Vector3PointDefinition as PointDefinitionLike<glam::Vec3>>::parse(
        value, context,
    )
}

#[cfg(feature = "json")]
pub fn parse_vector4_point_definition(
    value: JsonValue,
    context: &mut BaseProviderContext,
) -> Vector4PointDefinition {
    <Vector4PointDefinition as PointDefinitionLike<glam::Vec4>>::parse(value, context)
}

#[cfg(feature = "json")]
pub fn parse_quaternion_point_definition(
    value: JsonValue,
    context: &mut BaseProviderContext,
) -> quaternion_point_definition::QuaternionPointDefinition {
    <quaternion_point_definition::QuaternionPointDefinition as PointDefinitionLike<glam::Quat>>::parse(
        value, context,
    )
}

pub fn interpolate_float_point_definition(
    definition: &FloatPointDefinition,
    time: f32,
    context: &BaseProviderContext,
) -> (f32, bool) {
    <FloatPointDefinition as PointDefinitionLike<f32>>::interpolate(definition, time, context)
}

pub fn interpolate_vector3_point_definition(
    definition: &vector3_point_definition::Vector3PointDefinition,
    time: f32,
    context: &BaseProviderContext,
) -> (glam::Vec3, bool) {
    <vector3_point_definition::Vector3PointDefinition as PointDefinitionLike<glam::Vec3>>::interpolate(
        definition, time, context,
    )
}

pub fn interpolate_vector4_point_definition(
    definition: &Vector4PointDefinition,
    time: f32,
    context: &BaseProviderContext,
) -> (glam::Vec4, bool) {
    <Vector4PointDefinition as PointDefinitionLike<glam::Vec4>>::interpolate(
        definition, time, context,
    )
}

pub fn interpolate_quaternion_point_definition(
    definition: &quaternion_point_definition::QuaternionPointDefinition,
    time: f32,
    context: &BaseProviderContext,
) -> (glam::Quat, bool) {
    <quaternion_point_definition::QuaternionPointDefinition as PointDefinitionLike<glam::Quat>>::interpolate(
        definition, time, context,
    )
}
