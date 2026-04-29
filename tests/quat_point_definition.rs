use glam::{Quat, Vec3};
use serde_json::json;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::prelude::AbstractValueProvider;
use tracks_rs::quaternion_utils::QuaternionUtilsExt;
use tracks_rs::{
    point_definition::quaternion_point_definition::QuaternionPointDefinition,
    prelude::PointDefinitionLike,
};

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

fn ang_diff(a: f32, b: f32) -> f32 {
    let d = a - b;
    (d + 180.0).rem_euclid(360.0) - 180.0
}

fn quat_approx_assert(q1: Quat, q2: Quat, eps: f32) {
    let q1_euler = q1.to_unity_euler_degrees();
    let q2_euler = q2.to_unity_euler_degrees();

    assert!(
        approx_eq(ang_diff(q1_euler.x, q2_euler.x), 0.0, eps),
        "x mismatch: {} vs {}",
        q1_euler.x,
        q2_euler.x
    );
    assert!(
        approx_eq(ang_diff(q1_euler.y, q2_euler.y), 0.0, eps),
        "y mismatch: {} vs {}",
        q1_euler.y,
        q2_euler.y
    );
    assert!(
        approx_eq(ang_diff(q1_euler.z, q2_euler.z), 0.0, eps),
        "z mismatch: {} vs {}",
        q1_euler.z,
        q2_euler.z
    );
}

fn not_quat_approx_assert(q1: Quat, q2: Quat, eps: f32) {
    let q1_euler = q1.to_unity_euler_degrees();
    let q2_euler = q2.to_unity_euler_degrees();

    assert!(
        !approx_eq(ang_diff(q1_euler.x, q2_euler.x), 0.0, eps)
            || !approx_eq(ang_diff(q1_euler.y, q2_euler.y), 0.0, eps)
            || !approx_eq(ang_diff(q1_euler.z, q2_euler.z), 0.0, eps),
        "quaternions unexpectedly approximately equal: {} vs {}",
        q1_euler,
        q2_euler
    );
}

#[test]
fn integration_quaternion_parse_and_interpolate() {
    let js = json!([
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.1],
        [0.0, -90.0, 0.0, 0.2],
        [-90.0, -90.0, 0.0, 0.3]
    ]);

    let mut ctx = BaseProviderContext::new();

    let def = QuaternionPointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 4);

    let (q_mid, _last) = def.interpolate(0.15, &ctx);

    let q_l = Quat::from_unity_euler_degrees(Vec3::new(0.0f32, 0.0f32, 0.0f32));
    let q_r = Quat::from_unity_euler_degrees(Vec3::new(0.0f32, -90.0f32, 0.0f32));
    let expected_mid = q_l.slerp(q_r, 0.5);

    quat_approx_assert(q_mid, expected_mid, 1e-3);
}

#[test]
fn quaternion_point_definition_parse_from_smoothed_provider_s14() {
    let mut ctx = BaseProviderContext::new();

    // choose an euler triple and convert to quaternion
    let euler = Vec3::new(12.0_f32, -34.0_f32, 56.0_f32);
    let q = Quat::from_unity_euler_degrees(euler);

    ctx.set_values("baseHeadRotation", BaseValue::from(q));

    // Parse a quaternion point definition that references the smoothed rotation provider
    let def = QuaternionPointDefinition::parse(json!(["baseHeadRotation.s14"]), &mut ctx);
    assert!(def.has_base_provider());

    // With a large multiplier (14) and delta=1.0 the smoothing should progress to the target
    ctx.update_providers(1.0);

    let (value, is_last) = def.interpolate(0.0, &ctx);

    let eps = 1e-4_f32;
    assert!((value.x - q.x).abs() <= eps, "x mismatch");
    assert!((value.y - q.y).abs() <= eps, "y mismatch");
    assert!((value.z - q.z).abs() <= eps, "z mismatch");
    assert!((value.w - q.w).abs() <= eps, "w mismatch");
    assert!(
        is_last,
        "parsed quaternion from smoothed provider should be last after full update"
    );
}

#[test]
fn parse_with_swizzled_base_provider() {
    let mut ctx = BaseProviderContext::new();

    // set base rotation to a known euler triple (store as Vec3 so a swizzle returns euler components)
    let euler = Vec3::new(10.0_f32, 20.0_f32, -30.0_f32);
    let q = Quat::from_unity_euler_degrees(euler);
    ctx.set_values("baseHeadRotation", BaseValue::from(q));

    // Use a provider for the three euler components and a trailing time value
    let js = json!([["baseHeadRotation.xyz", 0.0]]);
    let def = QuaternionPointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 1);

    let (qi, _last) = def.interpolate(0.0, &ctx);
    // expect the quaternion returned to correspond to the original Euler angles
    quat_approx_assert(qi, q, 1e-3);

    // now update
    let new_euler = Vec3::new(-45.0_f32, 60.0_f32, 90.0_f32);
    let new_q = Quat::from_unity_euler_degrees(new_euler);
    ctx.set_values("baseHeadRotation", BaseValue::from(new_q));

    // and check
    let (qi2, _last2) = def.interpolate(0.0, &ctx);
    quat_approx_assert(qi2, new_q, 1e-3);
}

#[test]
fn simple_base_swizzle_usage() {
    let mut ctx = BaseProviderContext::new();
    ctx.set_values(
        "baseHeadRotation",
        BaseValue::from(Quat::from_unity_euler_degrees(Vec3::new(4.0, 5.0, 6.0))),
    );

    let provider = ctx.get_value_provider("baseHeadRotation.yx");
    let vals = provider.values(&ctx);

    assert!(approx_eq(vals[0], 5.0, 1e-3));
    assert!(approx_eq(vals[1], 4.0, 1e-3));
}

#[test]
fn base_provider_updates_reflect_in_quaternion_definition_no_smoothing() {
    let mut ctx = BaseProviderContext::new();

    // static start point (identity)
    let initial_euler = Vec3::new(0.0, 0.0, 0.0);
    let initial_quat = Quat::from_unity_euler_degrees(initial_euler);
    let js = json!([
        [initial_euler.x, initial_euler.y, initial_euler.z, 0.0],
        ["baseHeadRotation", 1.0]
    ]);

    // initial base euler values (degrees)
    let head_rot_euler = Vec3::new(0.0, 0.0, 30.0);
    let head_rot = Quat::from_unity_euler_degrees(head_rot_euler);
    ctx.set_values("baseHeadRotation", BaseValue::from(head_rot));

    let def = QuaternionPointDefinition::parse(js.clone(), &mut ctx);
    assert_eq!(def.get_count(), 2);

    // interpolate at midway (0.5) -> slerp between identity and base quaternion
    let (q_before, _last) = def.interpolate(0.5, &ctx);
    quat_approx_assert(q_before, initial_quat.slerp(head_rot, 0.5), 1e-3);

    // change base provider to a different rotation
    let head_rot_euler = Vec3::new(0.0, 0.0, 90.0);
    let head_rot = Quat::from_unity_euler_degrees(head_rot_euler);
    ctx.set_values("baseHeadRotation", BaseValue::from(head_rot));

    let (q_after, _last2) = def.interpolate(0.5, &ctx);

    // it should be 0.5 interpolation between identity and the new base quaternion, not the original one, since there is no smoothing
    let q_after = q_after;
    quat_approx_assert(q_after, initial_quat.slerp(head_rot, 0.5), 1e-3);
}

#[test]
fn base_provider_updates_with_smoothing_swizzle_and_operator() {
    let mut ctx = BaseProviderContext::new();

    // smoothing loses precision, so use a larger epsilon for these checks
    let eps = 1e-1;

    // point: static at 0, dynamic from smoothed+swizzled base provider with an additive modifier
    let js = json!([
        [0.0, 0.0, 0.0, 0.0],
        ["baseHeadRotation.zxy.s1", [10.0, 0.0, 0.0, "opAdd"], 1.0]
    ]);
    let def = QuaternionPointDefinition::parse(js, &mut ctx);

    let head_rot_euler = Vec3::new(0.0, 0.0, 30.0);
    let head_rot = Quat::from_unity_euler_degrees(head_rot_euler);

    // apply the swizzle and operator to the expected rotation
    let expected_rot = |quat: Quat| {
        let euler = quat.to_unity_euler_degrees();
        Quat::from_unity_euler_degrees(Vec3::new(euler.z + 10.0, euler.x, euler.y))
    };

    // initial base euler values
    ctx.set_values("baseHeadRotation", BaseValue::from(head_rot));

    {
        let (q_interpolated_e, _last) = def.interpolate(0.5, &ctx);
        let q_interpolated = q_interpolated_e;

        let _ = q_interpolated.to_unity_euler_degrees();
        not_quat_approx_assert(q_interpolated, expected_rot(q_interpolated), eps);
    }

    ctx.update_providers(1.0);

    {
        let (q_interpolated_e, _last) = def.interpolate(0.5, &ctx);
        let q_interpolated = q_interpolated_e;

        quat_approx_assert(
            q_interpolated,
            Quat::IDENTITY.slerp(expected_rot(head_rot), 0.5),
            eps,
        );
    }

    // ensure smooth providers are updated to current base values
    // required for smoothing to work correctly since it needs to know the current state as the starting point for smoothing to the new target

    let head_rot_euler = Vec3::new(0.0, 0.0, 90.0);
    let head_rot = Quat::from_unity_euler_degrees(head_rot_euler);
    ctx.set_values("baseHeadRotation", BaseValue::from(head_rot));
    ctx.update_providers(1.0);

    {
        let (q_interpolated_e, _last) = def.interpolate(0.5, &ctx);
        let q_interpolated = q_interpolated_e;

        quat_approx_assert(
            q_interpolated,
            Quat::IDENTITY.slerp(expected_rot(head_rot), 0.5),
            eps,
        );
    }
}
