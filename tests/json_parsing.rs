use serde_json::json;
use tracks_rs::{
    base_provider_context::BaseProviderContext,
    base_value::BaseValue,
    point_definition::{
        PointDefinitionLike, Vector4PointDefinition, basic_point_definition::BasicPointDefinition,
        quaternion_point_definition::QuaternionPointDefinition,
        vector3_point_definition::Vector3PointDefinition,
    },
    quaternion_utils::QuaternionUtilsExt,
};

// NOTE: The `is_last`/`last` boolean returned by `interpolate(time)` follows the
// Heck C# semantics used in the reference implementation:
// - If the last point's time is <= requested `time`, `last` is true and the last
//   point's value is returned.
// - If the first point's time is >= requested `time`, `last` is false and the
//   first point's value is returned.
// - For times strictly between two points, `last` is false and interpolation is
//   performed between the surrounding points.
// Tests below assert `is_last` according to this rule.

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
    // Single-point shorthand should be considered the last point at time 0
    assert!(float_is_last);

    let vec3_definition = Vector3PointDefinition::parse(json!([1.0, 2.0, 3.0]), &mut context);
    let (vec3_value, vec3_is_last) = vec3_definition.interpolate(0.0, &context);
    assert_eq!(vec3_value, glam::Vec3::new(1.0, 2.0, 3.0));
    // Single-point shorthand should be considered the last point at time 0
    assert!(vec3_is_last);

    let vec4_definition = Vector4PointDefinition::parse(json!([0.1, 0.2, 0.3, 0.4]), &mut context);
    let (vec4_value, vec4_is_last) = vec4_definition.interpolate(0.0, &context);
    assert_eq!(vec4_value, glam::Vec4::new(0.1, 0.2, 0.3, 0.4));
    // Single-point shorthand should be considered the last point at time 0
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

#[test]
fn parses_vector3_from_smoothed_and_swizzled_base_provider() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseHeadPosition",
        BaseValue::from(glam::Vec3::new(10.0, 20.0, 30.0)),
    );

    let definition =
        Vector3PointDefinition::parse(json!([["baseHeadPosition.zyx.s0_5"]]), &mut context);
    assert!(definition.has_base_provider());

    // Advance smoothing by delta=1.0 with multiplier 0.5.
    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert_eq!(value, glam::Vec3::new(15.0, 10.0, 5.0));
    assert!(is_last);
}

#[test]
fn parses_vector4_with_base_provider_modifier_op_mul() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseNote0Color",
        BaseValue::from(glam::Vec4::new(1.0, 0.5, 0.25, 1.0)),
    );

    let definition = Vector4PointDefinition::parse(
        json!([["baseNote0Color", [0.5, 0.25, 2.0, 1.0, "opMul"]]]),
        &mut context,
    );
    assert!(definition.has_base_provider());

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert_eq!(value, glam::Vec4::new(0.5, 0.125, 0.5, 1.0));
    assert!(!is_last);
}

#[test]
fn parses_float_from_smoothed_and_swizzled_base_provider() {
    let mut context = BaseProviderContext::new();
    context.set_values("baseSongTime", BaseValue::from(2.0_f32));

    let definition =
        BasicPointDefinition::<f32>::parse(json!([["baseSongTime.x.s0_5"]]), &mut context);
    assert!(definition.has_base_provider());

    // Advance smoothing by delta=1.0 with multiplier 0.5.
    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert!((value - 1.0).abs() < 1e-6);
    assert!(is_last);
}

#[test]
fn parses_quaternion_from_smoothed_base_provider() {
    let mut context = BaseProviderContext::new();
    let target_euler = glam::Vec3::new(12.0, -34.0, 56.0);
    let target_quat = glam::Quat::from_unity_euler_degrees(&target_euler);
    context.set_values("baseHeadRotation", BaseValue::from(target_quat));

    let definition =
        QuaternionPointDefinition::parse(json!([["baseHeadRotation.s1"]]), &mut context);
    assert!(definition.has_base_provider());

    // Advance smoothing fully so provider reaches target quaternion.
    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    let eps = 1e-5_f32;
    assert!((value.x - target_quat.x).abs() < eps);
    assert!((value.y - target_quat.y).abs() < eps);
    assert!((value.z - target_quat.z).abs() < eps);
    assert!((value.w - target_quat.w).abs() < eps);
    assert!(is_last);
}

#[test]
fn parses_quaternion_from_smoothed_base_provider_s10() {
    let mut context = BaseProviderContext::new();
    let target_euler = glam::Vec3::new(12.0, -34.0, 56.0);
    let target_quat = glam::Quat::from_unity_euler_degrees(&target_euler);
    context.set_values("baseHeadRotation", BaseValue::from(target_quat));

    let definition =
        QuaternionPointDefinition::parse(json!([["baseHeadRotation.s10"]]), &mut context);
    assert!(definition.has_base_provider());

    // With multiplier 10 and delta=1.0 this should advance fully to the target
    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    let eps = 1e-5_f32;
    println!("value: {:?}, target: {:?}", value, target_quat);
    assert!((value.x - target_quat.x).abs() < eps);
    assert!((value.y - target_quat.y).abs() < eps);
    assert!((value.z - target_quat.z).abs() < eps);
    assert!((value.w - target_quat.w).abs() < eps);
    assert!(is_last);
}

#[test]
fn parses_vector3_from_smoothed_base_provider_s10() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseHeadPosition",
        BaseValue::from(glam::Vec3::new(10.0, 20.0, 30.0)),
    );

    let definition = Vector3PointDefinition::parse(json!([["baseHeadPosition.s10"]]), &mut context);
    assert!(definition.has_base_provider());

    // With multiplier 10 and delta=1.0 this should advance fully to the target
    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert_eq!(value, glam::Vec3::new(10.0, 20.0, 30.0));
    assert!(is_last);
}

// ---------------------------------------------------------------------------
// is_last behavior tests
// These tests codify the rule from the reference C# implementation:
// - If the last point's time <= requested `time`, `is_last` should be true.
// - If the first point's time >= requested `time` (and the previous condition
//   didn't fire), `is_last` is false and the first point is returned.
// - For times strictly between two points, `is_last` is false.
// We add small cases for float and Vec3 to make expectations explicit.
// ---------------------------------------------------------------------------

#[test]
fn is_last_two_point_boundaries_float() {
    let mut ctx = BaseProviderContext::new();
    // Two points: value 0 at time 0, value 1 at time 1
    let def = BasicPointDefinition::<f32>::parse(json!([[0.0, 0.0], [1.0, 1.0]]), &mut ctx);

    // At time 0 -> first point should be selected, not last
    let (_v0, is_last0) = def.interpolate(0.0, &ctx);
    assert!(
        !is_last0,
        "At the first point boundary is_last should be false"
    );

    // At time 1 -> equals last point time -> is_last should be true
    let (_v1, is_last1) = def.interpolate(1.0, &ctx);
    assert!(
        is_last1,
        "At the last point boundary is_last should be true"
    );
}

#[test]
fn is_last_single_point_shorthand_float() {
    let mut ctx = BaseProviderContext::new();
    // Single-point shorthand: `[0.5]` becomes a single point (time implicitly 0).
    // According to the rule, for time==0 the last-point check should hit and
    // `is_last` should be true.
    let def = BasicPointDefinition::<f32>::parse(json!([0.5]), &mut ctx);
    let (_v, is_last) = def.interpolate(0.0, &ctx);
    assert!(
        is_last,
        "Single-point shorthand at time 0 should be considered last"
    );
}

#[test]
fn is_last_single_point_explicit_array_float() {
    let mut ctx = BaseProviderContext::new();
    // Single-point as explicit array with time 0: [[value, time]]
    // This is also the last point; for time==0 the last-point condition holds.
    let def = BasicPointDefinition::<f32>::parse(json!([[0.5, 0.0]]), &mut ctx);
    let (_v, is_last) = def.interpolate(0.0, &ctx);
    assert!(
        is_last,
        "Single explicit point at time 0 should be considered last"
    );
}

#[test]
fn is_last_two_point_boundaries_vec3() {
    let mut ctx = BaseProviderContext::new();
    // Vec3 version: two points at times 0 and 1
    let def = Vector3PointDefinition::parse(
        json!([[0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0]]),
        &mut ctx,
    );

    let (_s, s_last) = def.interpolate(0.0, &ctx);
    assert!(!s_last, "Vec3: at first boundary is_last should be false");

    let (_e, e_last) = def.interpolate(1.0, &ctx);
    assert!(e_last, "Vec3: at last boundary is_last should be true");
}

#[test]
#[should_panic(expected = "modifier point must have 3 numbers")]
fn panics_when_vec3_modifier_receives_extra_scalar_from_base_head_s10() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseHeadPosition",
        BaseValue::from(glam::Vec3::new(10.0, 20.0, 30.0)),
    );

    // `baseHeadPosition.s10` is a smooth provider specifier. Adding a scalar makes the
    // modifier payload expand to 4 numbers, which currently trips the vec3 arity assert.
    let _definition = Vector3PointDefinition::parse(
        json!([[0.0, 0.0, 0.0, ["baseHeadPosition.s10", 1.0, "opMul"]]]),
        &mut context,
    );
}

#[test]
fn parses_vec3_modifier_from_base_head_s10_without_panicking() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseHeadPosition",
        BaseValue::from(glam::Vec3::new(10.0, 20.0, 30.0)),
    );

    let definition = Vector3PointDefinition::parse(
        json!([[1.0, 1.0, 1.0, ["baseHeadPosition.s10", "opMul"]]]),
        &mut context,
    );
    assert!(definition.has_base_provider());

    context.update_providers(1.0);

    let (value, is_last) = definition.interpolate(0.0, &context);
    assert_eq!(value, glam::Vec3::new(10.0, 20.0, 30.0));
    assert!(!is_last);
}

#[test]
#[should_panic(expected = "modifier point must have 1 numbers")]
fn panics_when_float_modifier_receives_vec3_from_base_head_s10() {
    let mut context = BaseProviderContext::new();
    context.set_values(
        "baseHeadPosition",
        BaseValue::from(glam::Vec3::new(10.0, 20.0, 30.0)),
    );

    // A float modifier cannot accept the 3-component smooth provider produced by
    // `baseHeadPosition.s10`, so this should continue to fail fast.
    let _definition = BasicPointDefinition::<f32>::parse(
        json!([[0.0, ["baseHeadPosition.s10", "opMul"]]]),
        &mut context,
    );
}
