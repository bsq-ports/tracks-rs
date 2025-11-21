pub mod base_point_definition;
pub mod float_point_definition;
pub mod point_definition_interpolation;
pub mod quaternion_point_definition;
pub mod vector3_point_definition;
pub mod vector4_point_definition;

use std::str::FromStr;

use serde_json::Value as JsonValue;
use serde_json::json;

use crate::base_provider_context::BaseProviderContext;
use crate::ffi::types::WrapBaseValueType;
use crate::point_data::PointData;
use crate::{
    easings::functions::Functions,
    modifiers::{Modifier, operation::Operation},
    values::{ValueProvider, deserialize_values},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GroupType {
    Value,
    Flag,
    Modifier,
}

// The combined PointDefinition trait (acting as both BasePointDefinition and the templated PointDefinition<T>)

/// Point definitions are used to describe what happens over the course of an animation,
/// they are used slightly differently for different properties.
/// They consist of a collection of points over time.
pub trait PointDefinition {
    type Value: Default + Clone;

    // Required methods common to all definitions
    fn get_count(&self) -> usize;
    fn has_base_provider(&self) -> bool;
    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Self::Value;
    fn create_modifier(
        &self,
        values: Vec<ValueProvider>,
        modifiers: Vec<Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Modifier;
    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData;
    fn get_points_mut(&mut self) -> &mut Vec<PointData>;
    fn get_points(&self) -> &Vec<PointData>;
    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> Self::Value;
    fn get_type(&self) -> WrapBaseValueType;

    #[cfg(feature = "json")]
    fn deserialize_modifier(&self, list: &JsonValue, context: &BaseProviderContext) -> Modifier {
        let mut modifiers: Option<Vec<Modifier>> = None;
        let mut operation: Option<Operation> = None;
        let mut values: Option<Vec<ValueProvider>> = None;

        // Group values similar to PointDefinition::group_values
        for group in Self::group_values(list) {
            match group.0 {
                GroupType::Value => {
                    values = Some(deserialize_values(&group.1, context));
                }
                GroupType::Modifier => {
                    modifiers = Some(
                        group
                            .1
                            .iter()
                            .map(|m| self.deserialize_modifier(m, context))
                            .collect(),
                    );
                }
                GroupType::Flag => {
                    operation = Some(Operation::from_str(group.1[0].as_str().unwrap()).unwrap());
                }
            }
        }

        // Validate required fields
        //let values = values.expect("No points found.");
        //let operation = operation.expect("No operation found.");

        // Create modifier with collected values
        self.create_modifier(
            values.unwrap(),
            modifiers.unwrap_or_default(),
            operation.unwrap(),
            context,
        )
    }

    // Shared parse implementation
    #[cfg(feature = "json")]
    fn parse(&mut self, value: JsonValue, context: &BaseProviderContext) {
        let root: JsonValue = match value.as_array().unwrap()[0] {
            JsonValue::Array(_) => value,
            _ => {
                let mut cloned = value.as_array().unwrap().clone();
                cloned.push(json!(0));
                json!([cloned])
            }
        };

        let Some(array) = root.as_array() else { return };

        for raw_point in array {
            if raw_point.is_null() {
                continue;
            }

            let mut easing = Functions::EaseLinear;
            let mut modifiers: Option<Vec<Modifier>> = None;
            let mut flags: Option<Vec<String>> = None;
            let mut vals: Option<Vec<ValueProvider>> = None;

            // Group the values and flags. (Assuming each raw_point has a structure similar to the C++ JSON)
            for group in Self::group_values(raw_point) {
                match group.0 {
                    GroupType::Value => {
                        vals = Some(deserialize_values(&group.1, context));
                    }
                    GroupType::Modifier => {
                        modifiers = Some(
                            group
                                .1
                                .iter()
                                .map(|m| self.deserialize_modifier(m, context))
                                .collect(),
                        );
                    }
                    GroupType::Flag => {
                        // Convert the group values (group.1) into a Vec<String>
                        let flags_vec: Vec<String> = group
                            .1
                            .iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect();

                        // Set the flags collected from the group.
                        flags = Some(flags_vec);

                        // Find the first flag starting with "ease" just like in the C# code.
                        if let Some(ref flags_inner) = flags
                            && let Some(easing_string) =
                                flags_inner.iter().find(|flag| flag.starts_with("ease"))
                        {
                            easing =
                                Functions::from_str(easing_string).unwrap_or(Functions::EaseLinear);
                        }
                    }
                }
            }

            // Create point data only if we have values
            let Some(vs) = vals else { continue };

            let point_data = self.create_point_data(
                vs,
                flags.unwrap_or_default(),
                modifiers.unwrap_or_default(),
                easing,
                context,
            );
            self.get_points_mut().push(point_data);
        }
    }

    // Binary search algorithm to find the relevant interval
    fn search_index(&self, points: &[PointData], time: f32) -> (usize, usize) {
        let mut l = 0;
        let mut r = points.len();

        while l < r - 1 {
            let m = (l + r) / 2;
            let point_time = points[m].get_time();
            if point_time < time {
                l = m;
            } else {
                r = m;
            }
        }

        (l, r)
    }

    // Helper method to group values from a JSON value.
    // In a more complete implementation, you'd examine the JSON structure.
    #[cfg(feature = "json")]
    fn group_values(value: &JsonValue) -> Vec<(GroupType, Vec<&JsonValue>)> {
        use std::collections::HashMap;

        let JsonValue::Array(array) = value else {
            return vec![];
        };
        let values: Vec<&JsonValue> = array.iter().collect();

        let mut result: HashMap<GroupType, Vec<&JsonValue>> = HashMap::new();
        for val in &values {
            // group values by their type in the array
            let entry = match val {
                JsonValue::String(s) if !s.starts_with("base") => GroupType::Flag,
                JsonValue::Array(_) => GroupType::Modifier,
                _ => GroupType::Value,
            };
            result.entry(entry).or_default().push(val);
        }

        let result: Vec<(GroupType, Vec<&JsonValue>)> = result.into_iter().collect();

        result
    }

    // The main interpolation method. Returns a tuple (interpolated value, is_last_point)
    fn interpolate(&self, time: f32, context: &BaseProviderContext) -> (Self::Value, bool) {
        let points = self.get_points();

        if points.is_empty() {
            return (Self::Value::default(), true);
        }

        let last_point = points.last().unwrap();
        if last_point.get_time() <= time {
            return (self.get_point(last_point, context), true);
        }

        let first_point = points.first().unwrap();
        if first_point.get_time() >= time {
            return (self.get_point(first_point, context), false);
        }

        let (l, r) = self.search_index(points, time);
        let point_l = &points[l];
        let point_r = &points[r];

        let normal_time = if point_r.get_time() - point_l.get_time() != 0.0 {
            (time - point_l.get_time()) / (point_r.get_time() - point_l.get_time())
        } else {
            0.0
        };

        let eased_time = point_r.get_easing().interpolate(normal_time);
        (
            self.interpolate_points(points, l, r, eased_time, context),
            false,
        )
    }
}
