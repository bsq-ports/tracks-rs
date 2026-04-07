use serde_json::json;
use tracks_rs::{
    base_provider_context::BaseProviderContext,
    point_definition::{
        PointDefinitionLike, Vector4PointDefinition, basic_point_definition::BasicPointDefinition,
        quaternion_point_definition::QuaternionPointDefinition,
        vector3_point_definition::Vector3PointDefinition,
    }, quaternion_utils::QuaternionUtilsExt,
};

#[test]
fn parses_float_point_definition_from_heck_json() {
    let mut context = BaseProviderContext::new();
    let definition = BasicPointDefinition::<f32>::parse(json!([0.5]), &mut context);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert!((value - 0.5).abs() < 1e-6);
    assert!(is_last);
}

#[test]
fn parses_vector3_point_definition_from_heck_json() {
    let mut context = BaseProviderContext::new();
    let definition = Vector3PointDefinition::parse(
        json!([[0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0]]),
        &mut context,
    );

    let (start, start_is_last) = definition.interpolate(0.0, &context);
    assert_eq!(start, glam::Vec3::ZERO);
    assert!(!start_is_last);

    let (end, end_is_last) = definition.interpolate(1.0, &context);
    assert_eq!(end, glam::Vec3::new(1.0, 2.0, 3.0));
    assert!(end_is_last);
}

#[test]
fn parses_color_point_definition_from_heck_json() {
    let mut context = BaseProviderContext::new();
    let definition = Vector4PointDefinition::parse(json!([0.25, 0.5, 0.75, 1.0]), &mut context);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert_eq!(value, glam::Vec4::new(0.25, 0.5, 0.75, 1.0));
    assert!(is_last);
}

#[test]
fn parses_single_point_shorthand_forms_from_heck_json() {
    let mut context = BaseProviderContext::new();

    let float_definition = BasicPointDefinition::<f32>::parse(json!([0.5]), &mut context);
    let (float_value, float_is_last) = float_definition.interpolate(0.0, &context);
    assert!((float_value - 0.5).abs() < 1e-6);
    assert!(float_is_last);

    let vec3_definition = Vector3PointDefinition::parse(json!([1.0, 2.0, 3.0]), &mut context);
    let (vec3_value, vec3_is_last) = vec3_definition.interpolate(0.0, &context);
    assert_eq!(vec3_value, glam::Vec3::new(1.0, 2.0, 3.0));
    assert!(vec3_is_last);

    let vec4_definition = Vector4PointDefinition::parse(json!([0.1, 0.2, 0.3, 0.4]), &mut context);
    let (vec4_value, vec4_is_last) = vec4_definition.interpolate(0.0, &context);
    assert_eq!(vec4_value, glam::Vec4::new(0.1, 0.2, 0.3, 0.4));
    assert!(vec4_is_last);
}

#[test]
fn parses_quaternion_point_definition_from_heck_json() {
    let mut context = BaseProviderContext::new();
    let definition = QuaternionPointDefinition::parse(
        json!([[0.0, 0.0, 0.0, 0.0], [0.0, -90.0, 0.0, 0.5]]),
        &mut context,
    );

    let (start, start_is_last) = definition.interpolate(0.0, &context);
    assert_eq!(start, glam::Quat::IDENTITY);
    assert!(!start_is_last);

    let (end, end_is_last) = definition.interpolate(0.5, &context);
    let expected = glam::Quat::from_unity_euler_degrees(&glam::Vec3::new(0.0, -90.0, 0.0));
    assert_eq!(end, expected);
    assert!(end_is_last);
}
