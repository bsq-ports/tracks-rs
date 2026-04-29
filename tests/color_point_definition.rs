use glam::Vec4;
use serde_json::json;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::point_definition::basic_point_definition::BasicPointDefinition;
use tracks_rs::prelude::PointDefinitionLike;

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn integration_color_parse_and_interpolate() {
    let js = json!([[0.0, 0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 4.0, 1.0]]);

    let mut ctx = BaseProviderContext::new();
    type Vector4PointDefinition = BasicPointDefinition<Vec4>;

    let def = Vector4PointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 2);

    let (v_mid, _last) = def.interpolate(0.5, &ctx);
    // expected halfway between (0,0,0,0) and (1,2,3,4)
    assert!(approx_eq(v_mid.x, 0.5, 1e-6));
    assert!(approx_eq(v_mid.y, 1.0, 1e-6));
    assert!(approx_eq(v_mid.z, 1.5, 1e-6));
    assert!(approx_eq(v_mid.w, 2.0, 1e-6));
}

#[test]
fn parse_with_swizzled_base_provider_for_color() {
    let mut ctx = BaseProviderContext::new();

    // set base color
    let base = Vec4::new(0.1, 0.2, 0.3, 0.4);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(base));

    let js = json!([["baseEnvironmentColor0.xyzw", 0.0]]);
    type Vector4PointDefinition = BasicPointDefinition<Vec4>;
    let def = Vector4PointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 1);

    let (v, _last) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v.x, base.x, 1e-6));
    assert!(approx_eq(v.y, base.y, 1e-6));
    assert!(approx_eq(v.z, base.z, 1e-6));
    assert!(approx_eq(v.w, base.w, 1e-6));

    // change base and verify it updates
    let new_base = Vec4::new(0.9, 0.8, 0.7, 0.6);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(new_base));
    let (v2, _last2) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v2.x, new_base.x, 1e-6));
    assert!(approx_eq(v2.y, new_base.y, 1e-6));
    assert!(approx_eq(v2.z, new_base.z, 1e-6));
    assert!(approx_eq(v2.w, new_base.w, 1e-6));
}

#[test]
fn base_provider_updates_reflect_in_color_definition_no_smoothing() {
    let mut ctx = BaseProviderContext::new();

    let js = json!([[0.0, 0.0, 0.0, 0.0, 0.0], ["baseEnvironmentColor0", 1.0]]);
    type Vector4PointDefinition = BasicPointDefinition<Vec4>;

    // initial base color
    let base = Vec4::new(1.0, 0.0, 0.0, 0.5);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(base));

    let def = Vector4PointDefinition::parse(js.clone(), &mut ctx);
    assert_eq!(def.get_count(), 2);

    let expected_half = |v: Vec4| v * 0.5;
    let (v_before, _last) = def.interpolate(0.5, &ctx);
    // inspect point values
    let _pts = def.get_points();
    let p0 = def.interpolate(0.0, &ctx).0;
    let p1 = def.interpolate(1.0, &ctx).0;
    println!("p0 = {:?}, p1 = {:?}", p0, p1);
    // debug output
    println!("v_before = {:?}, base = {:?}", v_before, base);
    // halfway between zero and base
    assert!(approx_eq(v_before.x, expected_half(base).x, 1e-6));
    assert!(approx_eq(v_before.y, expected_half(base).y, 1e-6));
    assert!(approx_eq(v_before.z, expected_half(base).z, 1e-6));
    assert!(approx_eq(v_before.w, expected_half(base).w, 1e-6));

    // change base color
    let new_base = Vec4::new(0.0, 1.0, 0.0, 0.25);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(new_base));

    let expected_half2 = |v: Vec4| v * 0.5;
    let (v_after, _last2) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_after.x, expected_half2(new_base).x, 1e-6));
    assert!(approx_eq(v_after.y, expected_half2(new_base).y, 1e-6));
    assert!(approx_eq(v_after.z, expected_half2(new_base).z, 1e-6));
    assert!(approx_eq(v_after.w, expected_half2(new_base).w, 1e-6));
}

#[test]
fn base_provider_updates_with_smoothing_swizzle_and_operator_for_color() {
    let mut ctx = BaseProviderContext::new();

    // Use smoothing and operator add on a swizzled provider
    let js = json!([
        [0.0, 0.0, 0.0, 0.0, 0.0],
        [
            "baseEnvironmentColor0.xyzw.s1",
            [0.1, 0.2, 0.3, 0.4, "opAdd"],
            1.0
        ]
    ]);
    type Vector4PointDefinition = BasicPointDefinition<Vec4>;

    let def = Vector4PointDefinition::parse(js, &mut ctx);

    // base color
    let base = Vec4::new(0.2, 0.3, 0.4, 0.5);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(base));

    // expected after applying modifier (add)
    let added = Vec4::new(0.1, 0.2, 0.3, 0.4);
    let expected = |v: Vec4| v + added;

    // before smoothing updates, should not yet equal expected
    let (v_before, _l) = def.interpolate(0.5, &ctx);
    let eps = 1e-2_f32;
    let equal_before = (v_before - expected(base) * 0.5).length() <= eps;
    assert!(!equal_before, "value unexpectedly already smoothed/added");

    // run smoothing update (full delta)
    ctx.update_providers(1.0);

    let (v_after, _l2) = def.interpolate(0.5, &ctx);
    // should be halfway between zero and expected (since static start is zero)
    assert!((v_after.x - expected(base).x * 0.5).abs() <= eps);
    assert!((v_after.y - expected(base).y * 0.5).abs() <= eps);
    assert!((v_after.z - expected(base).z * 0.5).abs() <= eps);
    assert!((v_after.w - expected(base).w * 0.5).abs() <= eps);

    // change base and update providers again to ensure smoothing persists
    let new_base = Vec4::new(0.5, 0.4, 0.3, 0.2);
    ctx.set_values("baseEnvironmentColor0", BaseValue::from(new_base));
    ctx.update_providers(1.0);

    let (v_final, _l3) = def.interpolate(0.5, &ctx);
    let expected_final = |v: Vec4| v + added;
    assert!((v_final.x - expected_final(new_base).x * 0.5).abs() <= eps);
    assert!((v_final.y - expected_final(new_base).y * 0.5).abs() <= eps);
    assert!((v_final.z - expected_final(new_base).z * 0.5).abs() <= eps);
    assert!((v_final.w - expected_final(new_base).w * 0.5).abs() <= eps);
}
