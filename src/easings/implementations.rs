/// What `sqrt` is clamped to in the circ easings, so rounding past the end can never produce NaN.
/// The same value as Unity's `Mathf.Epsilon`, the smallest positive `f32`.
const SQRT_FLOOR: f32 = f32::from_bits(1);

// The easing functions are based on the ones in the original PR by Owen
// https://github.com/Aeroluna/Heck/pull/183/changes

pub const fn ease_linear(p: f32) -> f32 {
    p
}

pub fn ease_step(p: f32) -> f32 {
    p.floor()
}

/// Modeled after the parabola y = 2x - x^2
pub const fn ease_out_quad(p: f32) -> f32 {
    (2.0 - p) * p
}

pub const fn ease_in_quad(p: f32) -> f32 {
    p * p
}

pub const fn ease_in_out_quad(p: f32) -> f32 {
    // a slightly faster smoothstep
    let x = p - 0.5;
    (x - x * x.abs()) * 2.0 + 0.5
}

pub const fn ease_in_cubic(p: f32) -> f32 {
    p * p * p
}

pub const fn ease_out_cubic(p: f32) -> f32 {
    let f = 1.0 - p;
    1.0 - (f * f * f)
}

pub const fn ease_in_out_cubic(p: f32) -> f32 {
    let f = p - 0.5;
    let x = f.abs();
    ((4.0 * x - 6.0) * x + 3.0) * f + 0.5
}

pub const fn ease_in_quart(p: f32) -> f32 {
    p * p * p * p
}

pub const fn ease_out_quart(p: f32) -> f32 {
    let f = 1.0 - p;
    1.0 - (f * f * f * f)
}

pub const fn ease_in_out_quart(p: f32) -> f32 {
    let f = p - 0.5;
    let x = f.abs();
    let mut t = x * -8.0 + 16.0;
    t = t * x - 12.0;
    t = t * x + 4.0;
    t * f + 0.5
}

pub const fn ease_in_quint(p: f32) -> f32 {
    p * p * p * p * p
}

pub const fn ease_out_quint(p: f32) -> f32 {
    let f = 1.0 - p;
    1.0 - (f * f * f * f * f)
}

pub const fn ease_in_out_quint(p: f32) -> f32 {
    let f = p - 0.5;
    let x = f.abs();
    let mut t = x * 16.0 - 40.0;
    t = t * x + 40.0;
    t = t * x - 20.0;
    t = t * x + 5.0;
    t * f + 0.5
}

pub fn ease_in_sine(p: f32) -> f32 {
    1.0 - (std::f32::consts::FRAC_PI_2 * p).cos()
}

pub fn ease_out_sine(p: f32) -> f32 {
    (p * std::f32::consts::FRAC_PI_2).sin()
}

pub fn ease_in_out_sine(p: f32) -> f32 {
    let f = (std::f32::consts::FRAC_PI_2 * p).sin();
    f * f
}

pub fn ease_in_circ(p: f32) -> f32 {
    1.0 - (1.0 - p * p).max(SQRT_FLOOR).sqrt()
}

pub fn ease_out_circ(p: f32) -> f32 {
    ((2.0 - p) * p).max(SQRT_FLOOR).sqrt()
}

pub fn ease_in_out_circ(p: f32) -> f32 {
    if p < 0.5 {
        0.5 - (0.25 - p * p).max(SQRT_FLOOR).sqrt()
    } else {
        let q = p - 1.0;
        0.5 + (0.25 - q * q).max(SQRT_FLOOR).sqrt()
    }
}

pub fn ease_in_expo(p: f32) -> f32 {
    // rescaled so that f(0) = 0 and f(1) = 1, with no snapping at the start
    const S: f32 = 1.0 / 1023.0;
    if p <= 0.0 {
        p
    } else {
        2.0f32.powf(10.0 * p) * S - S
    }
}

pub fn ease_out_expo(p: f32) -> f32 {
    const S: f32 = 1024.0 / 1023.0;
    if p > 1.0 {
        p
    } else {
        S - S * 2.0f32.powf(-10.0 * p)
    }
}

pub fn ease_in_out_expo(p: f32) -> f32 {
    if p > 1.0 {
        return p;
    }

    let x = p * 20.0 - 10.0;
    const S: f32 = 512.0 / 1023.0;

    if x < 0.0 {
        // left half
        0.5 - (S - S * 2.0f32.powf(x))
    } else {
        // right half
        0.5 + (S - S * 2.0f32.powf(-x))
    }
}

pub fn ease_in_elastic(p: f32) -> f32 {
    (13.0 * std::f32::consts::FRAC_PI_2 * p).sin() * 2.0f32.powf(10.0 * (p - 1.0))
}

pub fn ease_out_elastic(p: f32) -> f32 {
    ((-13.0 * std::f32::consts::FRAC_PI_2 * (p + 1.0)).sin() * 2.0f32.powf(-10.0 * p)) + 1.0
}

pub fn ease_in_out_elastic(p: f32) -> f32 {
    if p < 0.5 {
        0.5 * (13.0 * std::f32::consts::FRAC_PI_2 * (2.0 * p)).sin()
            * 2.0f32.powf(10.0 * ((2.0 * p) - 1.0))
    } else {
        0.5 * (((-13.0 * std::f32::consts::FRAC_PI_2 * (2.0 * p)).sin()
            * 2.0f32.powf(-10.0 * ((2.0 * p) - 1.0)))
            + 2.0)
    }
}

pub fn ease_in_back(p: f32) -> f32 {
    (p * p * p) - (p * (p * std::f32::consts::PI).sin())
}

pub fn ease_out_back(p: f32) -> f32 {
    let f = 1.0 - p;
    1.0 - ((f * f * f) - (f * (f * std::f32::consts::PI).sin()))
}

pub fn ease_in_out_back(p: f32) -> f32 {
    if p < 0.5 {
        let f = 2.0 * p;
        0.5 * ((f * f * f) - (f * (f * std::f32::consts::PI).sin()))
    } else {
        let f = 1.0 - ((2.0 * p) - 1.0);
        (0.5 * (1.0 - ((f * f * f) - (f * (f * std::f32::consts::PI).sin())))) + 0.5
    }
}

/// Bouncing from 0 up to 1. Collision points: 4/11, 8/11, 9/11, 1.0
pub const fn ease_out_bounce(p: f32) -> f32 {
    // the minimum of four parabolas, one per bounce
    let a = (121.0 / 16.0) * p * p;
    let mut x = a;

    let q1 = p - (6.0 / 11.0);
    let b = (363.0 / 40.0) * q1 * q1 + (7.0 / 10.0);
    x = if b < x { b } else { x };

    let q2 = p - (179.0 / 220.0);
    let c = (4356.0 / 361.0) * q2 * q2 + (91.0 / 100.0);
    x = if c < x { c } else { x };

    let q3 = p - (19.0 / 20.0);
    let d = (54.0 / 5.0) * q3 * q3 + (973.0 / 1000.0);
    if d < x { d } else { x }
}

pub const fn ease_in_bounce(p: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - p)
}

pub const fn ease_in_out_bounce(p: f32) -> f32 {
    if p < 0.5 {
        0.5 * ease_in_bounce(2.0 * p)
    } else {
        (0.5 * ease_out_bounce((2.0 * p) - 1.0)) + 0.5
    }
}
