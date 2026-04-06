use crate::{
    base_provider_context::BaseProviderContext,
    base_value::{BaseValue, WrapBaseValueType},
};

use super::{PointDefinitionLike, base_point_definition::BasePointDefinition};

/// A structure to manage interpolation between two point definitions over time.
#[derive(Default, Debug, Clone)]
pub struct PointDefinitionInterpolation {
    pub interpolate_time: f32,
    // use refs here to avoid mass cloning
    pub prev_point: Option<BasePointDefinition>,
    pub point: Option<BasePointDefinition>,
    ty: WrapBaseValueType,
}

impl PointDefinitionInterpolation {
    pub fn new(point: Option<BasePointDefinition>, ty: WrapBaseValueType) -> Self {
        PointDefinitionInterpolation {
            interpolate_time: 0.0,
            prev_point: None,
            point,
            ty,
        }
    }

    pub fn empty(ty: WrapBaseValueType) -> Self {
        PointDefinitionInterpolation {
            interpolate_time: 0.0,
            prev_point: None,
            point: None,
            ty,
        }
    }

    pub fn get_type(&self) -> WrapBaseValueType {
        self.ty
    }

    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<BasePointDefinition>) {
        self.interpolate_time = 0.0;
        self.prev_point = self.point.take();
        self.point = new_point_data;

        if let Some(point_data) = &self.point {
            assert!(
                point_data.get_type() == self.ty,
                "PointDefinitionInterpolation type mismatch: expected {:?}, got {:?}",
                self.ty,
                point_data.get_type()
            );
        }
    }

    /// Interpolate between the previous and current point definitions at the given time.
    /// Returns None if there are no points to interpolate.
    pub fn interpolate(&self, time: f32, context: &BaseProviderContext) -> Option<BaseValue> {
        match (&self.prev_point, &self.point) {
            (Some(prev_point_data), Some(point_data)) => {
                let a = prev_point_data.interpolate(time, context).0;
                let b = point_data.interpolate(time, context).0;

                let result = BaseValue::lerp(a, b, self.interpolate_time);

                Some(result)
            }
            (None, Some(point_data)) => Some(point_data.interpolate(time, context).0),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::easings::functions::Functions;
    use crate::modifiers::quaternion_modifier::QuaternionValues;
    use crate::point_data::basic_point_data::BasicPointData;
    use crate::point_data::quaternion_point_data::QuaternionPointData;
    use crate::{base_provider_context::BaseProviderContext, modifiers::ModifierValues};
    use glam::{Quat, Vec3, Vec4};

    type FloatPointDefinition =
        crate::point_definition::basic_point_definition::BasicPointDefinition<f32>;
    type Vector3PointDefinition =
        crate::point_definition::vector3_point_definition::Vector3PointDefinition;
    type Vector4PointDefinition =
        crate::point_definition::basic_point_definition::BasicPointDefinition<Vec4>;
    type QuaternionPointDefinition =
        crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;

    #[test]
    fn test_float_interpolation_midpoint() {
        let prev = FloatPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let next = FloatPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(10.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(20.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let prev_bp =
            crate::point_definition::base_point_definition::BasePointDefinition::Float(prev);
        let next_bp =
            crate::point_definition::base_point_definition::BasePointDefinition::Float(next);

        let mut interp = PointDefinitionInterpolation::new(Some(next_bp), WrapBaseValueType::Float);
        interp.prev_point = Some(prev_bp);
        interp.interpolate_time = 0.5;

        let ctx = BaseProviderContext::new();

        let result = interp.interpolate(0.25, &ctx).unwrap();
        let v = result.as_float().unwrap();
        // expected: prev interpolate at 0.25 = 2.5, next interpolate at 0.25 = 12.5, lerp between them at 0.5 = 7.5
        assert!((v - 7.5).abs() < 1e-6);
    }

    #[test]
    fn test_vector3_and_vector4_interpolation() {
        // Vector3
        let prev_v3 = Vector3PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec3::new(0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let next_v3 = Vector3PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec3::new(6.0, 6.0, 6.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let prev_bp_v3 = BasePointDefinition::Vector3(prev_v3);
        let next_bp_v3 = BasePointDefinition::Vector3(next_v3);

        let mut interp_v3 =
            PointDefinitionInterpolation::new(Some(next_bp_v3), WrapBaseValueType::Vec3);
        interp_v3.prev_point = Some(prev_bp_v3);
        interp_v3.interpolate_time = 0.5;

        let ctx = BaseProviderContext::new();
        let result_v3 = interp_v3.interpolate(0.25, &ctx).unwrap();
        let v3 = result_v3.as_vec3().unwrap();
        // prev interp at 0.25 = (0->3) = 0.75, next interp = (3->6) = 3.75, lerp -> (0.75+3.75)/2 = 2.25 per component
        assert!((v3.x - 2.25).abs() < 1e-6);

        // Vector4
        let prev_v4 = Vector4PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(0.0, 0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(4.0, 4.0, 4.0, 4.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let next_v4 = Vector4PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(4.0, 4.0, 4.0, 4.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(8.0, 8.0, 8.0, 8.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let prev_bp_v4 =
            crate::point_definition::base_point_definition::BasePointDefinition::Vector4(prev_v4);
        let next_bp_v4 =
            crate::point_definition::base_point_definition::BasePointDefinition::Vector4(next_v4);

        let mut interp_v4 =
            PointDefinitionInterpolation::new(Some(next_bp_v4), WrapBaseValueType::Vec4);
        interp_v4.prev_point = Some(prev_bp_v4);
        interp_v4.interpolate_time = 0.5;

        let result_v4 = interp_v4.interpolate(0.25, &ctx).unwrap();
        let v4 = result_v4.as_vec4().unwrap();
        // prev interp at 0.25 = 1.0, next interp = 5.0, lerp -> 3.0 per component
        assert!((v4.x - 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_quaternion_interpolation_slerp() {
        let q1 = Quat::from_array([0.0, 0.0, 0.0, 1.0]);
        let q2 = Quat::from_array([0.0, 0.0, 1.0, 0.0]);

        let prev_q = QuaternionPointDefinition::new(vec![QuaternionPointData::new(
            QuaternionValues::Static(Vec3::ZERO, q1),
            0.0,
            vec![],
            Functions::EaseLinear,
        )]);

        let next_q = QuaternionPointDefinition::new(vec![QuaternionPointData::new(
            QuaternionValues::Static(Vec3::ZERO, q2),
            0.0,
            vec![],
            Functions::EaseLinear,
        )]);

        let prev_bp_q =
            crate::point_definition::base_point_definition::BasePointDefinition::Quaternion(prev_q);
        let next_bp_q =
            crate::point_definition::base_point_definition::BasePointDefinition::Quaternion(next_q);

        let mut interp_q =
            PointDefinitionInterpolation::new(Some(next_bp_q), WrapBaseValueType::Quat);
        interp_q.prev_point = Some(prev_bp_q);
        interp_q.interpolate_time = 0.25;

        let ctx = BaseProviderContext::new();
        let result_q = interp_q.interpolate(0.0, &ctx).unwrap();
        let got = result_q.as_quat().unwrap();

        let expected = Quat::slerp(q1, q2, 0.25);
        assert!((got.x - expected.x).abs() < 1e-6);
        assert!((got.y - expected.y).abs() < 1e-6);
        assert!((got.z - expected.z).abs() < 1e-6);
        assert!((got.w - expected.w).abs() < 1e-6);
    }
}
