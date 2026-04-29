use serde_json::json;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::point_definition::basic_point_definition::BasicPointDefinition;
use tracks_rs::prelude::PointDefinitionLike;

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

#[test]
fn integration_f32_parse_and_interpolate() {
    let js = json!([[0.0, 0.0], [1.0, 1.0]]);

    let mut ctx = BaseProviderContext::new();
    type FloatPointDefinition = BasicPointDefinition<f32>;

    let def = FloatPointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 2);

    let (v_mid, _last) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_mid, 0.5, 1e-6));
}

#[test]
fn parse_with_base_combo_provider_for_f32() {
    let mut ctx = BaseProviderContext::new();

    let base = 0.42_f32;
    ctx.set_values("baseCombo", BaseValue::from(base));

    let js = json!([["baseCombo", 0.0]]);
    type FloatPointDefinition = BasicPointDefinition<f32>;
    let def = FloatPointDefinition::parse(js, &mut ctx);
    assert_eq!(def.get_count(), 1);

    let (v, _last) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v, base, 1e-6));

    let new_base = 0.9_f32;
    ctx.set_values("baseCombo", BaseValue::from(new_base));
    let (v2, _last2) = def.interpolate(0.0, &ctx);
    assert!(approx_eq(v2, new_base, 1e-6));
}

#[test]
fn base_combo_updates_reflect_in_f32_definition_no_smoothing() {
    let mut ctx = BaseProviderContext::new();

    let js = json!([[0.0, 0.0], ["baseCombo", 1.0]]);
    type FloatPointDefinition = BasicPointDefinition<f32>;

    let base = 1.0_f32;
    ctx.set_values("baseCombo", BaseValue::from(base));

    let def = FloatPointDefinition::parse(js.clone(), &mut ctx);
    assert_eq!(def.get_count(), 2);

    let expected_half = |v: f32| v * 0.5;
    let (v_before, _last) = def.interpolate(0.5, &ctx);

    let p0 = def.interpolate(0.0, &ctx).0;
    let p1 = def.interpolate(1.0, &ctx).0;
    println!("p0 = {:?}, p1 = {:?}", p0, p1);
    println!("v_before = {:?}, base = {:?}", v_before, base);

    assert!(approx_eq(v_before, expected_half(base), 1e-6));

    let new_base = 0.25_f32;
    ctx.set_values("baseCombo", BaseValue::from(new_base));

    let (v_after, _last2) = def.interpolate(0.5, &ctx);
    assert!(approx_eq(v_after, expected_half(new_base), 1e-6));
}

#[test]
fn base_combo_updates_with_smoothing_and_operator_for_f32() {
    let mut ctx = BaseProviderContext::new();

    let js = json!([[0.0, 0.0], ["baseCombo.s1", [0.5, "opAdd"], 1.0]]);
    type FloatPointDefinition = BasicPointDefinition<f32>;

    let def = FloatPointDefinition::parse(js, &mut ctx);

    let base = 0.2_f32;
    ctx.set_values("baseCombo", BaseValue::from(base));

    let added = 0.5_f32;
    let expected = |v: f32| v + added;

    let (v_before, _l) = def.interpolate(0.5, &ctx);
    let eps = 1e-2_f32;
    let equal_before = (v_before - expected(base) * 0.5).abs() <= eps;
    assert!(!equal_before, "value unexpectedly already smoothed/added");

    ctx.update_providers(1.0);

    let (v_after, _l2) = def.interpolate(0.5, &ctx);
    assert!((v_after - expected(base) * 0.5).abs() <= eps);

    let new_base = 0.8_f32;
    ctx.set_values("baseCombo", BaseValue::from(new_base));
    ctx.update_providers(1.0);

    let (v_final, _l3) = def.interpolate(0.5, &ctx);
    let expected_final = |v: f32| v + added;
    assert!((v_final - expected_final(new_base) * 0.5).abs() <= eps);
}
