use glam::{Quat, Vec3};
use tracks_rs::base_value::BaseValue;
use tracks_rs::providers::AbstractValueProvider;
use tracks_rs::quaternion_utils::QuaternionUtilsExt;
use tracks_rs::{base_provider_context::BaseProviderContext, providers::UpdateableValues};

#[test]
fn swizzle_partial_provider_returns_components() {
    let mut ctx = BaseProviderContext::new();

    // set a Vec3 base value
    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(1.0, 2.0, 3.0)),
    );

    // request a swizzled provider
    let mut provider = ctx.get_value_provider("baseHeadPosition.xy");

    assert!(provider.is_updateable());

    ctx.update_providers(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_slice();

    assert_eq!(slice, &[1.0_f32, 2.0_f32]);
}

#[test]
fn smoothing_on_vector_reaches_target_on_full_delta() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(10.0, 20.0, 30.0)),
    );

    let mut provider = ctx.get_value_provider("baseHeadPosition.s1");

    assert!(provider.is_updateable());

    // full delta should move values to source exactly
    ctx.update_providers(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_slice();

    assert_eq!(slice, &[10.0_f32, 20.0_f32, 30.0_f32]);
}

#[test]
fn smoothing_on_quaternion_produces_expected_euler() {
    let mut ctx = BaseProviderContext::new();

    // pick an euler triple and convert to quaternion
    let euler = Vec3::new(12.0_f32, -34.0_f32, 56.0_f32);
    let q = Quat::from_unity_euler_degrees(&euler);

    ctx.set_values("baseHeadRotation", BaseValue::from(q));

    let mut provider = ctx.get_value_provider("baseHeadRotation.s1");

    assert!(provider.is_updateable());

    ctx.update_providers(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_slice();

    // values are Unity-style euler degrees (x,y,z) as f32
    // compare approximately
    let eps = 1e-3_f32;
    assert!((slice[0] - euler.x).abs() <= eps, "x mismatch");
    assert!((slice[1] - euler.y).abs() <= eps, "y mismatch");
    assert!((slice[2] - euler.z).abs() <= eps, "z mismatch");
}

#[test]
fn swizzle_three_components_returns_components() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(1.0, 2.0, 3.0)),
    );

    let mut provider = ctx.get_value_provider("baseHeadPosition.xyz");

    assert!(provider.is_updateable());

    ctx.update_providers(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_slice();

    assert_eq!(slice, &[1.0_f32, 2.0_f32, 3.0_f32]);
}

#[test]
fn swizzle_reorder_and_duplicate_returns_expected() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(4.0, 5.0, 6.0)),
    );

    let mut provider_yx = ctx.get_value_provider("baseHeadPosition.yx");
    ctx.update_providers(0.0);
    assert_eq!(provider_yx.values(&ctx).as_slice(), &[5.0_f32, 4.0_f32]);

    let mut provider_xx = ctx.get_value_provider("baseHeadPosition.xx");
    ctx.update_providers(0.0);
    assert_eq!(provider_xx.values(&ctx).as_slice(), &[4.0_f32, 4.0_f32]);
}

#[test]
fn smoothing_vector_fractional_delta_moves_partway() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(10.0, 20.0, 30.0)),
    );

    // use multiplier 0.5 (s0_5) and full delta=1.0 -> t = 0.5
    let mut provider = ctx.get_value_provider("baseHeadPosition.s0_5");

    assert!(provider.is_updateable());

    ctx.update_providers(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_slice();

    assert_eq!(slice, &[5.0_f32, 10.0_f32, 15.0_f32]);
}

#[test]
fn incremental_smoothing_vector_two_updates() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(10.0, 20.0, 30.0)),
    );

    // use multiplier 0.5 (s0_5)
    let provider = ctx.get_value_provider("baseHeadPosition.s0_5");

    assert!(provider.is_updateable());

    // first update via context: moves to 50% -> [5,10,15]
    ctx.update_providers(1.0);
    let vals = provider.values(&ctx);
    assert_eq!(vals.as_slice(), &[5.0_f32, 10.0_f32, 15.0_f32]);

    // change the base provider simultaneously to a new target
    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(20.0, 40.0, 60.0)),
    );

    // second update via context: moves halfway from previous values toward new target
    ctx.update_providers(1.0);
    let vals2 = provider.values(&ctx);
    let slice2 = vals2.as_slice();

    let eps = 1e-6_f32;
    assert!((slice2[0] - 12.5_f32).abs() <= eps, "x mismatch");
    assert!((slice2[1] - 25.0_f32).abs() <= eps, "y mismatch");
    assert!((slice2[2] - 37.5_f32).abs() <= eps, "z mismatch");
}

#[test]
fn incremental_smoothing_small_delta_steps() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(10.0, 20.0, 30.0)),
    );

    // multiplier 1.0 (s1)
    let provider = ctx.get_value_provider("baseHeadPosition.s1");

    assert!(provider.is_updateable());

    // first update with delta=0.5 via context -> half-way
    ctx.update_providers(0.5);
    let vals = provider.values(&ctx);
    let eps = 1e-6_f32;
    assert!(
        (vals.as_slice()[0] - 5.0_f32).abs() <= eps,
        "first x mismatch"
    );

    // change base provider before the next update
    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(20.0, 40.0, 60.0)),
    );

    // second update with delta=0.5 via context -> moves half the remaining distance towards new target
    ctx.update_providers(0.5);
    let vals2 = provider.values(&ctx);
    assert!(
        (vals2.as_slice()[0] - 12.5_f32).abs() <= eps,
        "second x mismatch"
    );
}
