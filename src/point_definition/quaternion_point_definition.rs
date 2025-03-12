use glam::{Quat, Vec3, vec3};

use crate::{
    easings::functions::Functions,
    modifiers::{
        Modifier,
        operation::Operation,
        quaternion_modifier::{QuaternionModifier, QuaternionValues, TRACKS_EULER_ROT},
    },
    point_data::{PointData, quaternion_point_data::QuaternionPointData},
    values::{AbstractValueProvider, ValueProvider, base_provider_context::BaseProviderContext},
};

use super::PointDefinition;

#[derive(Default)]
pub struct QuaternionPointDefinition {
    points: Vec<PointData>,
}

impl PointDefinition for QuaternionPointDefinition {
    type Value = Quat;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| p.has_base_provider())
    }

    fn get_points_mut(&mut self) -> &mut Vec<PointData> {
        &mut self.points
    }

    fn create_modifier(
        &self,
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
                    values[0].to_radians(),
                    values[1].to_radians(),
                    values[2].to_radians(),
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
        &self,
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
                    values[0].to_radians(),
                    values[1].to_radians(),
                    values[2].to_radians(),
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

    fn get_points(&self) -> &Vec<PointData> {
        &self.points
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> Quat {
        point.get_quaternion(context)
    }
}

impl QuaternionPointDefinition {
    #[cfg(feature = "json")]
    pub fn new(value: serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
