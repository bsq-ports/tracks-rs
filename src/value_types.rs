use glam::{Vec3, Vec4};

use crate::base_value::WrapBaseValueType;

/// The most components any [`ValueType`] has (`Vec4`). Sizes fixed scratch buffers.
pub(crate) const MAX_COMPONENTS: usize = 4;

/// Represents a type that can be used as a value in the system, such as a float, vector, or quaternion.
/// This trait defines the necessary operations and conversions for these types, allowing them to be used
/// interchangeably in the animation system.
///
/// This is what allows us to have a unified way of handling different types of values (like float, vec3, vec4) in the system,
/// and to perform operations like interpolation, addition, etc. on them without needing to know the specific type
pub trait ValueType:
    Default
    + Copy
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Mul<f32, Output = Self>
{
    const VALUE_COUNT: usize;

    fn base_type() -> WrapBaseValueType;

    fn from_translate_slice(values: &[f32]) -> Self;

    fn from_slice(values: &[f32]) -> Self;

    #[inline]
    fn value_lerp(a: Self, b: Self, t: f32) -> Self {
        a + (b - a) * t
    }

    #[inline]
    fn value_lerp_clamped(a: Self, b: Self, t: f32) -> Self {
        Self::value_lerp(a, b, t.clamp(0.0, 1.0))
    }

    /// Interpolation for points with Heck's `lerpHSV` flag. Only colours (`Vec4`) blend in HSV;
    /// every other type ignores the flag and interpolates normally, as in Heck.
    #[inline]
    fn value_lerp_hsv(a: Self, b: Self, t: f32) -> Self {
        Self::value_lerp(a, b, t)
    }
}

impl ValueType for f32 {
    const VALUE_COUNT: usize = 1;

    fn from_slice(values: &[f32]) -> Self {
        values[0]
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        values[0]
    }

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Float
    }
}

impl ValueType for Vec3 {
    const VALUE_COUNT: usize = 3;

    fn from_slice(values: &[f32]) -> Self {
        Vec3::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec3::from_slice(values)
    }

    fn base_type() -> WrapBaseValueType {
        unreachable!("Vec3 is not a valid base type for BaseValue")
    }
}

impl ValueType for Vec4 {
    const VALUE_COUNT: usize = 4;

    fn from_slice(values: &[f32]) -> Self {
        Vec4::from_slice(values)
    }

    fn from_translate_slice(values: &[f32]) -> Self {
        Vec4::from_slice(values)
    }

    fn base_type() -> WrapBaseValueType {
        WrapBaseValueType::Vec4
    }

    /// Heck's `Vector4PointDefinition` with `lerpHSV`: lerps hue, saturation and value unclamped, alpha linearly.
    fn value_lerp_hsv(a: Self, b: Self, t: f32) -> Self {
        let (hl, sl, vl) = rgb_to_hsv(a.x, a.y, a.z);
        let (hr, sr, vr) = rgb_to_hsv(b.x, b.y, b.z);
        let (r, g, bl) = hsv_to_rgb(
            f32::value_lerp(hl, hr, t),
            f32::value_lerp(sl, sr, t),
            f32::value_lerp(vl, vr, t),
        );
        Vec4::new(r, g, bl, f32::value_lerp(a.w, b.w, t))
    }
}

/// Port of Unity's `Color.RGBToHSV`. Hue is in `[0, 1)`.
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // offset into the hue wheel, the dominant channel, then the other two in wheel order
    let (offset, dominant, one, two) = if b > g && b > r {
        (4.0, b, r, g)
    } else if g > r {
        (2.0, g, b, r)
    } else {
        (0.0, r, g, b)
    };

    let v = dominant;
    if v == 0.0 {
        return (0.0, 0.0, v);
    }

    let diff = v - one.min(two);
    let (mut h, s) = if diff != 0.0 {
        (offset + (one - two) / diff, diff / v)
    } else {
        (offset + (one - two), 0.0)
    };
    h /= 6.0;
    if h < 0.0 {
        h += 1.0;
    }
    (h, s, v)
}

/// Port of Unity's `Color.HSVToRGB` with `hdr = true` (no clamping), the overload Heck uses.
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v);
    }
    if v == 0.0 {
        return (0.0, 0.0, 0.0);
    }

    let h6 = h * 6.0;
    let sector = h6.floor();
    let t = h6 - sector;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * t);
    let u = v * (1.0 - s * (1.0 - t));

    // an unclamped hue can land outside the wheel; Unity handles -1..=6 and leaves black otherwise
    match sector as i32 {
        0 | 6 => (v, u, p),
        1 => (q, v, p),
        2 => (p, v, u),
        3 => (p, q, v),
        4 => (u, p, v),
        5 | -1 => (v, p, q),
        _ => (0.0, 0.0, 0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: (f32, f32, f32), b: (f32, f32, f32)) -> bool {
        (a.0 - b.0).abs() < 1e-5 && (a.1 - b.1).abs() < 1e-5 && (a.2 - b.2).abs() < 1e-5
    }

    #[test]
    fn rgb_to_hsv_matches_unity() {
        assert!(close(rgb_to_hsv(1.0, 0.0, 0.0), (0.0, 1.0, 1.0)));
        assert!(close(rgb_to_hsv(0.0, 1.0, 0.0), (1.0 / 3.0, 1.0, 1.0)));
        assert!(close(rgb_to_hsv(0.0, 0.0, 1.0), (2.0 / 3.0, 1.0, 1.0)));
        assert!(close(rgb_to_hsv(1.0, 0.0, 1.0), (5.0 / 6.0, 1.0, 1.0)));
        // grey has no hue or saturation
        assert!(close(rgb_to_hsv(0.5, 0.5, 0.5), (0.0, 0.0, 0.5)));
        assert!(close(rgb_to_hsv(0.0, 0.0, 0.0), (0.0, 0.0, 0.0)));
    }

    #[test]
    fn hsv_round_trips() {
        for rgb in [
            (0.2, 0.4, 0.9),
            (0.9, 0.1, 0.3),
            (0.3, 0.8, 0.2),
            (1.0, 1.0, 0.0),
            (2.0, 0.5, 0.0),
        ] {
            let (h, s, v) = rgb_to_hsv(rgb.0, rgb.1, rgb.2);
            assert!(close(hsv_to_rgb(h, s, v), rgb), "{rgb:?}");
        }
    }

    #[test]
    fn hsv_lerp_goes_around_the_hue_wheel() {
        let red = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let blue = Vec4::new(0.0, 0.0, 1.0, 0.0);
        // halfway from hue 0 to hue 2/3 is hue 1/3: green, with alpha lerped linearly
        let mid = Vec4::value_lerp_hsv(red, blue, 0.5);
        assert!(
            mid.abs_diff_eq(Vec4::new(0.0, 1.0, 0.0, 0.5), 1e-5),
            "{mid}"
        );
        // other types ignore the flag
        assert_eq!(f32::value_lerp_hsv(0.0, 10.0, 0.5), 5.0);
    }
}
