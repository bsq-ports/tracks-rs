use glam::FloatExt;

use crate::{
    base_provider_context::BaseProviderContext, easings::functions::Functions, modifiers::{
        float_modifier::{FloatModifier, FloatValues}, operation::Operation, Modifier
    }, point_data::{float_point_data::FloatPointData, PointData}, values::{AbstractValueProvider, ValueProvider}
};

use super::PointDefinition;

#[derive(Default)]
pub struct FloatPointDefinition {
    points: Vec<PointData>,
}

impl PointDefinition for FloatPointDefinition {
    type Value = f32;

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
            // Single static value
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 1 => {
                FloatValues::Static(static_val.values(context)[0])
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 1, "Float modifier point must have 1 number");
                FloatValues::Dynamic(values)
            }
        };
        Modifier::Float(FloatModifier::new(val, modifiers, operation))
    }

    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        _flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.

        let (value, time) = match &values[..] {
            // [x, y]
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 2 => {
                let values = static_val.values(context);
                let point = values[0];
                (FloatValues::Static(point), values[1])
            }

            _ => {
                // validate and get time
                let values_len: usize = values.iter().map(|v| v.values(context).len()).sum();

                let time = if values_len == 2 {
                    values
                        .last()
                        .and_then(|v| v.values(context).last().copied())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };

                (FloatValues::Dynamic(values), time)
            }
        };

        PointData::Float(FloatPointData::new(value, time, modifiers, easing))
    }

    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> f32 {
        let point_l = points[l].get_float(context);
        let point_r = points[r].get_float(context);

        f32::lerp(point_l, point_r, time)
    }

    fn get_points(&self) -> &Vec<PointData> {
        &self.points
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> f32 {
        point.get_float(context)
    }
}

impl FloatPointDefinition {
    /// Constructor equivalent – parses the provided JSON immediately.
    #[cfg(feature = "json")]
    pub fn new(value: serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
