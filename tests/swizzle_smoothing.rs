use tracks_rs::providers::AbstractValueProvider;
use tracks_rs::{base_provider_context::BaseProviderContext, providers::UpdateableValues};
use tracks_rs::providers::value::BaseValue;
use glam::{Vec3, Quat};
use tracks_rs::quaternion_utils::QuaternionUtilsExt;

#[test]
fn swizzle_partial_provider_returns_components() {
    let mut ctx = BaseProviderContext::new();

    // set a Vec3 base value
    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(1.0, 2.0, 3.0)));

    // request a swizzled provider
    let mut provider = ctx.get_value_provider("baseHeadPosition.xy");

    assert!(provider.is_updateable());

    provider.update(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_ref();

    assert_eq!(slice, &[1.0_f32, 2.0_f32]);
}

#[test]
fn smoothing_on_vector_reaches_target_on_full_delta() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(10.0, 20.0, 30.0)));

    let mut provider = ctx.get_value_provider("baseHeadPosition.s1");

    assert!(provider.is_updateable());

    // full delta should move values to source exactly
    provider.update(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_ref();

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

    provider.update(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_ref();

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

    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(1.0, 2.0, 3.0)));

    let mut provider = ctx.get_value_provider("baseHeadPosition.xyz");

    assert!(provider.is_updateable());

    provider.update(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_ref();

    assert_eq!(slice, &[1.0_f32, 2.0_f32, 3.0_f32]);
}

#[test]
fn swizzle_reorder_and_duplicate_returns_expected() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(4.0, 5.0, 6.0)));

    let mut provider_yx = ctx.get_value_provider("baseHeadPosition.yx");
    provider_yx.update(0.0);
    assert_eq!(provider_yx.values(&ctx).as_ref(), &[5.0_f32, 4.0_f32]);

    let mut provider_xx = ctx.get_value_provider("baseHeadPosition.xx");
    provider_xx.update(0.0);
    assert_eq!(provider_xx.values(&ctx).as_ref(), &[4.0_f32, 4.0_f32]);
}

#[test]
fn smoothing_vector_fractional_delta_moves_partway() {
    let mut ctx = BaseProviderContext::new();

    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(10.0, 20.0, 30.0)));

    // use multiplier 0.5 (s0_5) and full delta=1.0 -> t = 0.5
    let mut provider = ctx.get_value_provider("baseHeadPosition.s0_5");

    assert!(provider.is_updateable());

    provider.update(1.0);

    let vals = provider.values(&ctx);
    let slice = vals.as_ref();

    assert_eq!(slice, &[5.0_f32, 10.0_f32, 15.0_f32]);
}
