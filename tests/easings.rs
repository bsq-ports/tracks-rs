use std::f32::consts::{FRAC_PI_2, PI};

use tracks_rs::easings::functions::Functions;

const ALL: [&str; 32] = [
    "easeLinear", "easeStep",
    "easeInQuad", "easeOutQuad", "easeInOutQuad",
    "easeInCubic", "easeOutCubic", "easeInOutCubic",
    "easeInQuart", "easeOutQuart", "easeInOutQuart",
    "easeInQuint", "easeOutQuint", "easeInOutQuint",
    "easeInSine", "easeOutSine", "easeInOutSine",
    "easeInCirc", "easeOutCirc", "easeInOutCirc",
    "easeInExpo", "easeOutExpo", "easeInOutExpo",
    "easeInElastic", "easeOutElastic", "easeInOutElastic",
    "easeInBack", "easeOutBack", "easeInOutBack",
    "easeInBounce", "easeOutBounce", "easeInOutBounce",
];

fn easing(name: &str) -> Functions {
    name.parse().unwrap_or_else(|_| panic!("unknown easing {name}"))
}

/// The curves before https://github.com/Aeroluna/Heck/pull/183, for the easings that were only
/// simplified or reordered and so must still match.
fn old(name: &str, p: f32) -> f32 {
    match name {
        "easeOutQuad" => -(p * (p - 2.0)),
        "easeInOutQuad" if p < 0.5 => 2.0 * p * p,
        "easeInOutQuad" => (-2.0 * p * p) + (4.0 * p) - 1.0,
        "easeOutCubic" => (p - 1.0).powi(3) + 1.0,
        "easeInOutCubic" if p < 0.5 => 4.0 * p * p * p,
        "easeInOutCubic" => 0.5 * (2.0 * p - 2.0).powi(3) + 1.0,
        "easeOutQuart" => (p - 1.0).powi(3) * (1.0 - p) + 1.0,
        "easeInOutQuart" if p < 0.5 => 8.0 * p.powi(4),
        "easeInOutQuart" => -8.0 * (p - 1.0).powi(4) + 1.0,
        "easeOutQuint" => (p - 1.0).powi(5) + 1.0,
        "easeInOutQuint" if p < 0.5 => 16.0 * p.powi(5),
        "easeInOutQuint" => 0.5 * (2.0 * p - 2.0).powi(5) + 1.0,
        "easeInSine" => ((p - 1.0) * FRAC_PI_2).sin() + 1.0,
        "easeInOutSine" => 0.5 * (1.0 - (p * PI).cos()),
        "easeInCirc" => 1.0 - (1.0 - p * p).sqrt(),
        "easeOutCirc" => ((2.0 - p) * p).sqrt(),
        "easeInOutCirc" if p < 0.5 => 0.5 * (1.0 - (1.0 - 4.0 * p * p).sqrt()),
        "easeInOutCirc" => 0.5 * ((-(2.0 * p - 3.0) * (2.0 * p - 1.0)).sqrt() + 1.0),
        _ => unreachable!("{name}"),
    }
}

#[test]
fn every_easing_starts_at_0_and_ends_at_1() {
    for name in ALL {
        let f = easing(name);
        let (start, end) = (f.interpolate(0.0), f.interpolate(1.0));
        assert!(start.abs() < 1e-3, "{name}(0) = {start}");
        assert!((end - 1.0).abs() < 1e-3, "{name}(1) = {end}");
    }
}

#[test]
fn easings_are_never_nan_on_the_unit_interval() {
    for name in ALL {
        let f = easing(name);
        for i in 0..=1000 {
            let p = i as f32 / 1000.0;
            let v = f.interpolate(p);
            assert!(v.is_finite(), "{name}({p}) = {v}");
        }
    }
}

#[test]
fn circ_easings_survive_rounding_past_the_end() {
    // the argument of the sqrt goes slightly negative, which used to give NaN
    for name in ["easeInCirc", "easeOutCirc", "easeInOutCirc"] {
        for p in [1.0 + 1e-6, 1.0 + 1e-3, -1e-6] {
            let v = easing(name).interpolate(p);
            assert!(!v.is_nan(), "{name}({p}) = {v}");
        }
    }
}

#[test]
fn simplified_easings_match_the_old_curves() {
    let same = [
        "easeOutQuad", "easeInOutQuad", "easeOutCubic", "easeInOutCubic", "easeOutQuart",
        "easeInOutQuart", "easeOutQuint", "easeInOutQuint", "easeInSine", "easeInOutSine",
        "easeInCirc", "easeOutCirc", "easeInOutCirc",
    ];
    for name in same {
        let f = easing(name);
        for i in 0..=1000 {
            let p = i as f32 / 1000.0;
            let (new, old) = (f.interpolate(p), old(name, p));
            if old.is_nan() {
                continue;
            }
            assert!((new - old).abs() < 2e-3, "{name}({p}): new {new}, old {old}");
        }
    }
}

#[test]
fn expo_easings_no_longer_snap() {
    // the old versions jumped by about 0.001 at the first sample after 0
    assert!(easing("easeInExpo").interpolate(1e-4) < 1e-3);
    assert!(easing("easeOutExpo").interpolate(1.0 - 1e-4) < 1.0 + 1e-3);
    assert!((easing("easeInOutExpo").interpolate(0.5) - 0.5).abs() < 1e-4);
    // upper bound is kept for compatibility
    assert_eq!(easing("easeInOutExpo").interpolate(1.5), 1.5);
}

#[test]
fn bounce_is_continuous() {
    let f = easing("easeOutBounce");
    let mut prev = f.interpolate(0.0);
    for i in 1..=10_000 {
        let v = f.interpolate(i as f32 / 10_000.0);
        assert!((v - prev).abs() < 0.01, "jump at {}: {prev} -> {v}", i as f32 / 10_000.0);
        prev = v;
    }
}
