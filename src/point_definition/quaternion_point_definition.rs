use std::rc::Rc;

use glam::{Quat, Vec3, vec3};

use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        Modifier,
        operation::Operation,
        quaternion_modifier::{QuaternionModifier, QuaternionValues, TRACKS_EULER_ROT},
    },
    point_data::{PointData, quaternion_point_data::QuaternionPointData},
    values::{AbstractValueProvider, ValueProvider},
};

use super::PointDefinition;

#[derive(Default, Debug, Clone)]
pub struct QuaternionPointDefinition {
    points: Rc<[PointData]>,
}

impl PointDefinition for QuaternionPointDefinition {
    type Value = Quat;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| p.has_base_provider())
    }

    fn get_type(&self) -> crate::ffi::types::WrapBaseValueType {
        crate::ffi::types::WrapBaseValueType::Quat
    }

    fn create_modifier(
        values: Vec<ValueProvider>,
        modifiers: Vec<Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Modifier {
        let val = match values.as_slice() {
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 3 => {
                let values = static_val.values(context);
                let raw_vector = vec3(values[0], values[1], values[2]);
                let quat = Quat::from_euler(
                    TRACKS_EULER_ROT,
                    values[2].to_radians(),
                    values[0].to_radians(),
                    values[1].to_radians(),
                );
                QuaternionValues::Static(raw_vector, quat)
            }
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 3, "Vector3 modifier point must have 3 numbers");
                QuaternionValues::Dynamic(values)
            }
        };

        Modifier::Quaternion(QuaternionModifier::new(val, modifiers, operation))
    }

    fn create_point_data(
        values: Vec<ValueProvider>,
        _flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData {
        let (base_values, time) = match values.as_slice() {
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 4 => {
                let values = static_val.values(context);
                let raw_vector_point = Vec3::new(values[0], values[1], values[2]);
                let quat = Quat::from_euler(
                    TRACKS_EULER_ROT,
                    values[2].to_radians(),
                    values[0].to_radians(),
                    values[1].to_radians(),
                );
                (QuaternionValues::Static(raw_vector_point, quat), values[3])
            }
            _ => {
                let values_len: usize = values.iter().map(|v| v.values(context).len()).sum();
                let time = if values_len == 4 {
                    values
                        .last()
                        .and_then(|v| v.values(context).last().copied())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };
                (QuaternionValues::Dynamic(values), time)
            }
        };

        PointData::Quaternion(QuaternionPointData::new(
            base_values,
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Quat {
        let point_l = points[l].get_quaternion(context);
        let point_r = points[r].get_quaternion(context);
        point_l.slerp(point_r, time)
    }

    fn get_points(&self) -> &[PointData] {
        &self.points
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> Quat {
        point.get_quaternion(context)
    }

    fn new(points: Vec<PointData>) -> Self {
        Self {
            points: Rc::from(points),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{EulerRot, Quat};
    use serde_json::json;

    use crate::{
        base_provider_context::BaseProviderContext,
        modifiers::quaternion_modifier::TRACKS_EULER_ROT,
        point_data::quaternion_point_data::QuaternionPointData, point_definition::PointDefinition,
    };

    // Use Unity's Euler rotation order (ZXY(Ex)) for expected values in tests
    const UNITY_EULER: EulerRot = EulerRot::ZXYEx;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn local_rotation_interpolation_and_quat_values() {
        // _localRotation array
        let js = json!([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.1],
            [0.0, -90.0, 0.0, 0.2],
            [-90.0, -90.0, 0.0, 0.3]
        ]);

        let mut ctx = BaseProviderContext::new();

        let def = QuaternionPointDefinition::parse(js, &ctx);

        // sanity
        assert_eq!(def.get_count(), 4);

        // Initial (time 0.0)
        let (q0, is_last0) = def.interpolate(0.0, &ctx);
        let e0 = q0.to_euler(UNITY_EULER);
        assert!(approx_eq(e0.0.to_degrees(), 0.0, 1e-3));
        assert!(approx_eq(e0.1.to_degrees(), 0.0, 1e-3));
        assert!(approx_eq(e0.2.to_degrees(), 0.0, 1e-3));
        // identity quaternion expected
        assert!(approx_eq(q0.x, 0.0, 1e-3));
        assert!(approx_eq(q0.y, 0.0, 1e-3));
        assert!(approx_eq(q0.z, 0.0, 1e-3));
        assert!(approx_eq(q0.w, 1.0, 1e-3));

        // Intermediate between 0.1 and 0.2 -> t = 0.15 -> normalized 0.5 between those points
        let (q_mid, is_last_mid) = def.interpolate(0.15, &ctx);
        // Build expected by slerping the endpoint quaternions
        let q_l = Quat::from_euler(
            UNITY_EULER,
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            0.0f32.to_radians(),
        );
        let q_r = Quat::from_euler(
            UNITY_EULER,
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            (-90.0f32).to_radians(),
        );
        let expected_mid = q_l.slerp(q_r, 0.5);

        // compare quaternion components
        assert!(approx_eq(q_mid.x, expected_mid.x, 1e-3));
        assert!(approx_eq(q_mid.y, expected_mid.y, 1e-3));
        assert!(approx_eq(q_mid.z, expected_mid.z, 1e-3));
        assert!(approx_eq(q_mid.w, expected_mid.w, 1e-3));

        // compare euler angles (converted to degrees)
        let e_mid = q_mid.to_euler(UNITY_EULER);
        let e_expected = expected_mid.to_euler(UNITY_EULER);
        assert!(approx_eq(
            e_mid.0.to_degrees(),
            e_expected.0.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_mid.1.to_degrees(),
            e_expected.1.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_mid.2.to_degrees(),
            e_expected.2.to_degrees(),
            1e-2
        ));

        // Final (time 0.3)
        let (q_final, is_last_final) = def.interpolate(0.3, &ctx);
        let expected_final = Quat::from_euler(
            UNITY_EULER,
            0.0f32.to_radians(),
            (-90.0f32).to_radians(),
            (-90.0f32).to_radians(),
        );

        assert!(approx_eq(q_final.x, expected_final.x, 1e-3));
        assert!(approx_eq(q_final.y, expected_final.y, 1e-3));
        assert!(approx_eq(q_final.z, expected_final.z, 1e-3));
        assert!(approx_eq(q_final.w, expected_final.w, 1e-3));

        let e_final = q_final.to_euler(UNITY_EULER);
        let e_expected_final = expected_final.to_euler(UNITY_EULER);
        assert!(approx_eq(
            e_final.0.to_degrees(),
            e_expected_final.0.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_final.1.to_degrees(),
            e_expected_final.1.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_final.2.to_degrees(),
            e_expected_final.2.to_degrees(),
            1e-2
        ));
        assert!(is_last_final);
    }

    #[test]
    fn local_rotation_from_unity_quats() {
        // Build quaternion points directly (Unity-style xyzw)
        let ctx = BaseProviderContext::new();

        // Build quaternions from the same euler angles as the previous test
        let q0 = Quat::from_euler(
            TRACKS_EULER_ROT,
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            0.0f32.to_radians(),
        );
        let q1 = Quat::from_euler(
            TRACKS_EULER_ROT,
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            0.0f32.to_radians(),
        );
        let q2 = Quat::from_euler(
            TRACKS_EULER_ROT,
            0.0f32.to_radians(),
            0.0f32.to_radians(),
            (-90.0f32).to_radians(),
        );
        let q3 = Quat::from_euler(
            TRACKS_EULER_ROT,
            0.0f32.to_radians(),
            (-90.0f32).to_radians(),
            (-90.0f32).to_radians(),
        );

        let p0 = PointData::Quaternion(QuaternionPointData::new(
            QuaternionValues::Static(Vec3::new(0.0, 0.0, 0.0), q0),
            0.0,
            vec![],
            Functions::EaseLinear,
        ));
        let p1 = PointData::Quaternion(QuaternionPointData::new(
            QuaternionValues::Static(Vec3::new(0.0, 0.0, 0.0), q1),
            0.1,
            vec![],
            Functions::EaseLinear,
        ));
        let p2 = PointData::Quaternion(QuaternionPointData::new(
            QuaternionValues::Static(Vec3::new(0.0, -90.0, 0.0), q2),
            0.2,
            vec![],
            Functions::EaseLinear,
        ));
        let p3 = PointData::Quaternion(QuaternionPointData::new(
            QuaternionValues::Static(Vec3::new(-90.0, -90.0, 0.0), q3),
            0.3,
            vec![],
            Functions::EaseLinear,
        ));

        let def = QuaternionPointDefinition::new(vec![p0, p1, p2, p3]);

        // sanity
        assert_eq!(def.get_count(), 4);

        // initial
        let (qi0, last0) = def.interpolate(0.0, &ctx);
        let e0 = qi0.to_euler(TRACKS_EULER_ROT);
        assert!(approx_eq(e0.0.to_degrees(), 0.0, 1e-3));
        assert!(approx_eq(e0.1.to_degrees(), 0.0, 1e-3));
        assert!(approx_eq(e0.2.to_degrees(), 0.0, 1e-3));
        assert!(approx_eq(qi0.x, q0.x, 1e-3));
        assert!(approx_eq(qi0.y, q0.y, 1e-3));
        assert!(approx_eq(qi0.z, q0.z, 1e-3));
        assert!(approx_eq(qi0.w, q0.w, 1e-3));
        assert!(!last0);

        // mid (0.15) slerp between q1 and q2
        let (qmid, lastmid) = def.interpolate(0.15, &ctx);
        let expected_mid = q1.slerp(q2, 0.5);
        assert!(approx_eq(qmid.x, expected_mid.x, 1e-3));
        assert!(approx_eq(qmid.y, expected_mid.y, 1e-3));
        assert!(approx_eq(qmid.z, expected_mid.z, 1e-3));
        assert!(approx_eq(qmid.w, expected_mid.w, 1e-3));
        let e_mid = qmid.to_euler(TRACKS_EULER_ROT);
        let e_expected = expected_mid.to_euler(TRACKS_EULER_ROT);
        assert!(approx_eq(
            e_mid.0.to_degrees(),
            e_expected.0.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_mid.1.to_degrees(),
            e_expected.1.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_mid.2.to_degrees(),
            e_expected.2.to_degrees(),
            1e-2
        ));
        assert!(!lastmid);

        // final
        let (qf, lastf) = def.interpolate(0.3, &ctx);
        assert!(approx_eq(qf.x, q3.x, 1e-3));
        assert!(approx_eq(qf.y, q3.y, 1e-3));
        assert!(approx_eq(qf.z, q3.z, 1e-3));
        assert!(approx_eq(qf.w, q3.w, 1e-3));
        let e_f = qf.to_euler(TRACKS_EULER_ROT);
        let e_expected_f = q3.to_euler(TRACKS_EULER_ROT);
        assert!(approx_eq(
            e_f.0.to_degrees(),
            e_expected_f.0.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_f.1.to_degrees(),
            e_expected_f.1.to_degrees(),
            1e-2
        ));
        assert!(approx_eq(
            e_f.2.to_degrees(),
            e_expected_f.2.to_degrees(),
            1e-2
        ));
        assert!(lastf);
    }
}
