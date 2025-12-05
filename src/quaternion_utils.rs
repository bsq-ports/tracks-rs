use glam::{EulerRot, Quat, Vec3};

// May be ZXYEx or YXZ
// When using Quat::from_euler, match the order in the a, b, and c parameters
pub const TRACKS_EULER_ROT: EulerRot = EulerRot::ZXYEx;

pub trait QuaternionUtilsExt {
    fn to_unity_euler_degrees(&self) -> Vec3;
    fn from_unity_euler_degrees(euler: &Vec3) -> Quat;
}

impl QuaternionUtilsExt for Quat {
    /// Convert quaternion to euler degrees
    fn to_unity_euler_degrees(&self) -> Vec3 {
        // to_euler returns angles in the order matching TRACKS_EULER_ROT.
        // For `ZXY` (Z then X then Y) the tuple is (a=Z, b=X, c=Y).
        // Unity's Vector3(eulerAngles) is (x: rotation about X, y: rotation about Y, z: rotation about Z),
        // so map (a,b,c) -> (b, c, a).
        let (a, b, c) = self.to_euler(TRACKS_EULER_ROT);
        Vec3::new(b.to_degrees(), c.to_degrees(), a.to_degrees())
    }

    /// Convert euler degrees to quaternion
    fn from_unity_euler_degrees(euler: &Vec3) -> Quat {
        Quat::from_euler(
            TRACKS_EULER_ROT,
            euler.z.to_radians(),
            euler.x.to_radians(),
            euler.y.to_radians(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn unity_quat_roundtrip_euler_examples() {
        let cases = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(30.0, -45.0, 10.0),
            Vec3::new(-90.0, -90.0, 0.0),
        ];

        for e in cases {
            // build via convenience function
            let q = Quat::from_unity_euler_degrees(&e);
            // round-trip back to euler degrees
            let e2 = q.to_unity_euler_degrees();

            assert!(
                approx_eq(e.x, e2.x, 1e-3),
                "x mismatch: {} vs {}",
                e.x,
                e2.x
            );
            assert!(
                approx_eq(e.y, e2.y, 1e-3),
                "y mismatch: {} vs {}",
                e.y,
                e2.y
            );
            assert!(
                approx_eq(e.z, e2.z, 1e-3),
                "z mismatch: {} vs {}",
                e.z,
                e2.z
            );
        }
    }

    #[test]
    fn unity_quat_matches_explicit_from_euler_ordering() {
        // pick a specific Euler triple and verify Quat::from_unity_euler_degrees
        // is equivalent to constructing with the underlying from_euler call used by the impl
        let e = Vec3::new(12.0_f32, -34.0_f32, 56.0_f32);

        let q_via_helper = Quat::from_unity_euler_degrees(&e);

        // The implementation uses TRACKS_EULER_ROT and passes components (z, x, y) as radians
        let q_expected = Quat::from_euler(
            TRACKS_EULER_ROT,
            e.z.to_radians(),
            e.x.to_radians(),
            e.y.to_radians(),
        );

        assert!(approx_eq(q_via_helper.x, q_expected.x, 1e-4));
        assert!(approx_eq(q_via_helper.y, q_expected.y, 1e-4));
        assert!(approx_eq(q_via_helper.z, q_expected.z, 1e-4));
        assert!(approx_eq(q_via_helper.w, q_expected.w, 1e-4));
    }
}
