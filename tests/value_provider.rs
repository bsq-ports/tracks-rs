use glam::{Quat, Vec3, Vec4};
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::prelude::AbstractValueProvider;
use tracks_rs::providers::ValueProvider;

// Helper to check is_rotation for a provider string
fn check_is_rotation(expr: &str, expected: bool) {
    let mut ctx = BaseProviderContext::new();

    // set some base values used by providers
    ctx.set_values("baseHeadRotation", BaseValue::from(Quat::IDENTITY));
    ctx.set_values("baseHeadPosition", BaseValue::from(Vec3::new(1.0, 2.0, 3.0)));
    ctx.set_values("baseNote0Color", BaseValue::from(Vec4::new(0.1, 0.2, 0.3, 1.0)));

    let provider = ctx.get_value_provider(expr);
    let is_rot = provider.is_rotation(&ctx);
    assert_eq!(is_rot, expected, "provider '{}' is_rotation expected {} got {}", expr, expected, is_rot);
}

#[test]
fn is_rotation_static_quat_provider() {
    // explicit quaternion base should be rotation
    check_is_rotation("baseHeadRotation", true);
}

#[test]
fn is_rotation_static_vec3_euler_provider() {
    // vec3 base interpreted as rotation when used as rotation provider
    // here we simulate by using smoothed rotation provider syntax
    check_is_rotation("baseHeadRotation.s1", true);
}

#[test]
fn is_rotation_swizzled_xyz_on_rotation() {
    // swizzle on rotation provider should still be rotation
    check_is_rotation("baseHeadRotation.xyz", true);
}

#[test]
fn is_rotation_swizzled_xy_on_position_false() {
    // swizzling position to xy should not be rotation
    check_is_rotation("baseHeadPosition.xy", false);
}

#[test]
fn is_rotation_smoothed_position_false() {
    // smoothing a position should not mark it as rotation
    check_is_rotation("baseHeadPosition.s1", false);
}

#[test]
fn is_rotation_smoothed_rotation_true() {
    // smoothing applied to a rotation provider should be rotation
    check_is_rotation("baseHeadRotation.s0_5", true);
}

#[test]
fn is_rotation_swizzle_then_smooth_rotation_true() {
    // combined swizzle and smoothing on rotation
    check_is_rotation("baseHeadRotation.xyz.s1", true);
}

#[test]
fn is_rotation_color_swizzle_false() {
    // color swizzle should not be rotation
    check_is_rotation("baseNote0Color.xyz", false);
}
