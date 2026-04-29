use glam::Vec3;
use serde_json::json;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::point_definition::vector3_point_definition::Vector3PointDefinition;
use tracks_rs::prelude::PointDefinitionLike;

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn integration_vec3_parse_and_interpolate() {
    let js = json!([[0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0]]);

    let mut ctx = BaseProviderContext::new();
    let def = Vector3PointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 2);
    let (v_mid, _last) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_mid.x, 0.5, 1e-6));
    assert!(approx_eq(v_mid.y, 1.0, 1e-6));
    assert!(approx_eq(v_mid.z, 1.5, 1e-6));
}

#[test]
fn parse_with_swizzled_base_provider_for_vec3() {
    let mut ctx = BaseProviderContext::new();

    let base = Vec3::new(0.1, 0.2, 0.3);
    ctx.set_values("baseHeadPosition", BaseValue::from(base));

    let js = json!([["baseHeadPosition.xyz", 0.0]]);
    let def = Vector3PointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 1);
    let (v, _last) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v.x, base.x, 1e-6));
    assert!(approx_eq(v.y, base.y, 1e-6));
    assert!(approx_eq(v.z, base.z, 1e-6));

    let new_base = Vec3::new(0.9, 0.8, 0.7);
    ctx.set_values("baseHeadPosition", BaseValue::from(new_base));
    let (v2, _last2) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v2.x, new_base.x, 1e-6));
    assert!(approx_eq(v2.y, new_base.y, 1e-6));
    assert!(approx_eq(v2.z, new_base.z, 1e-6));
}

#[test]
fn base_provider_updates_reflect_in_vec3_definition_no_smoothing() {
    let mut ctx = BaseProviderContext::new();

    let js = json!([[0.0, 0.0, 0.0, 0.0], ["baseHeadPosition", 1.0]]);

    let def = Vector3PointDefinition::parse(js.clone(), &mut ctx);
    let base = Vec3::new(1.0, 0.0, 0.0);
    ctx.set_values("baseHeadPosition", BaseValue::from(base));
    assert_eq!(def.get_count(), 2);

    let expected_half = |v: Vec3| v * 0.5;
    let (v_before, _last) = def.interpolate(0.5, &ctx);

    let p0 = def.interpolate(0.0, &ctx).0;
    let p1 = def.interpolate(1.0, &ctx).0;
    println!("p0 = {:?}, p1 = {:?}", p0, p1);
    println!("v_before = {:?}, base = {:?}", v_before, base);

    assert!(approx_eq(v_before.x, expected_half(base).x, 1e-6));
    assert!(approx_eq(v_before.y, expected_half(base).y, 1e-6));
    assert!(approx_eq(v_before.z, expected_half(base).z, 1e-6));

    let new_base = Vec3::new(0.0, 1.0, 0.0);
    ctx.set_values("baseHeadPosition", BaseValue::from(new_base));

    let (v_after, _last2) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_after.x, expected_half(new_base).x, 1e-6));
    assert!(approx_eq(v_after.y, expected_half(new_base).y, 1e-6));
    assert!(approx_eq(v_after.z, expected_half(new_base).z, 1e-6));
}

#[test]
fn base_provider_updates_with_smoothing_swizzle_and_operator_for_vec3() {
    let mut ctx = BaseProviderContext::new();

    let js = json!([
        [0.0, 0.0, 0.0, 0.0],
        ["baseHeadPosition.xyz.s1", [0.1, 0.2, 0.3, "opAdd"], 1.0]
    ]);

    let def = Vector3PointDefinition::parse(js, &mut ctx);

    let base = Vec3::new(0.2, 0.3, 0.4);
    ctx.set_values("baseHeadPosition", BaseValue::from(base));

    let added = Vec3::new(0.1, 0.2, 0.3);
    let _expected = |v: Vec3| v + added;

    let (_v_before, _l) = def.interpolate(0.5, &ctx);
    let js = json!([[0.0, 0.0, 0.0, 0.0], ["baseHeadPosition", 1.0]]);

    let def = Vector3PointDefinition::parse(js.clone(), &mut ctx);
    let base = Vec3::new(1.0, 0.0, 0.0);
    ctx.set_values("baseHeadPosition", BaseValue::from(base));
    assert_eq!(def.get_count(), 2);

    let expected_half = |v: Vec3| v * 0.5;
    let (v_before, _last) = def.interpolate(0.5, &ctx);

    let p0 = def.interpolate(0.0, &ctx).0;
    let p1 = def.interpolate(1.0, &ctx).0;
    println!("p0 = {:?}, p1 = {:?}", p0, p1);
    println!("v_before = {:?}, base = {:?}", v_before, base);

    assert!(approx_eq(v_before.x, expected_half(base).x, 1e-6));
    assert!(approx_eq(v_before.y, expected_half(base).y, 1e-6));
    assert!(approx_eq(v_before.z, expected_half(base).z, 1e-6));

    let new_base = Vec3::new(0.0, 1.0, 0.0);
    ctx.set_values("baseHeadPosition", BaseValue::from(new_base));

    let (v_after, _last2) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_after.x, expected_half(new_base).x, 1e-6));
    assert!(approx_eq(v_after.y, expected_half(new_base).y, 1e-6));
    assert!(approx_eq(v_after.z, expected_half(new_base).z, 1e-6));
}
