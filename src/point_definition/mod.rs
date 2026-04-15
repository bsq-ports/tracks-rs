pub mod base_point_definition;
pub mod basic_point_definition;
pub mod point_definition_interpolation;

// specific handling
pub mod quaternion_point_definition;
pub mod vector3_point_definition;

pub type FloatPointDefinition = basic_point_definition::BasicPointDefinition<f32>;
pub type Vector4PointDefinition = basic_point_definition::BasicPointDefinition<glam::Vec4>;

use std::str::FromStr;

#[cfg(feature = "json")]
use serde_json::Value as JsonValue;

#[cfg(feature = "json")]
use serde_json::json;

use crate::base_provider_context::BaseProviderContext;
use crate::base_value::WrapBaseValueType;
use crate::modifiers::ModifierLike;
use crate::point_data::PointDataLike;
use crate::{
    easings::functions::Functions, modifiers::operation::Operation, providers::ValueProvider,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GroupType {
    Value,
    Flag,
    Modifier,
}

/// Point definitions are used to describe what happens over the course of an animation,
/// they are used slightly differently for different properties.
/// They consist of a collection of points over time.
pub trait PointDefinitionLike<T>: Default
where
    T: Default + Clone,
{
    type Modifier: ModifierLike<T>;
    type PointData: PointDataLike<T>;

    // Required methods common to all definitions
    fn get_count(&self) -> usize;
    fn has_base_provider(&self) -> bool;
    fn interpolate_points(
        &self,
        l: &Self::PointData,
        r: &Self::PointData,
        l_index: usize,
        r_index: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> T;
    fn create_modifier(
        values: Vec<ValueProvider>,
        modifiers: Vec<Self::Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Self::Modifier;
    fn create_point_data(
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Self::Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Self::PointData;
    // fn get_points_mut(&mut self) -> &mut Vec<PointData>;
    fn get_points(&self) -> &[Self::PointData];

    fn get_type(&self) -> WrapBaseValueType;

    fn new(points: Vec<Self::PointData>) -> Self;

    /// Deserializes a JSON value into a Modifier. This is used for parsing modifiers from JSON.
    #[cfg(feature = "json")]
    fn deserialize_modifier(list: &JsonValue, context: &mut BaseProviderContext) -> Self::Modifier {
        let mut modifiers: Option<Vec<Self::Modifier>> = None;
        let mut operation: Option<Operation> = None;
        let mut values: Option<Vec<ValueProvider>> = None;

        // Group values similar to PointDefinition::group_values
        for group in group_values(list) {
            match group.0 {
                GroupType::Value => {
                    use crate::prelude::deserialize_values;

                    values = Some(deserialize_values(&group.1, context));
                }
                GroupType::Modifier => {
                    modifiers = Some(
                        group
                            .1
                            .iter()
                            .map(|m| Self::deserialize_modifier(m, context))
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
        Self::create_modifier(
            values.expect("No values found."),
            modifiers.unwrap_or_default(),
            operation.expect("No operation found."),
            context,
        )
    }

    /// Parses a JSON value into a PointDefinition.
    ///
    /// Accepted input shapes (compat with Heck spec):
    /// - An array of points: `[[p0], [p1], ...]` (each point is itself an array)
    /// - A single point shorthand: `[v0, v1, ..., time]` (top-level array that is not an array-of-arrays)
    /// - A single-point-without-time shorthand: `[v0, v1, ...]` — this will be treated as time `0`.
    ///
    /// This method normalizes the input into an array-of-points form and then groups values/modifiers/flags
    /// for each point. It is defensive: non-array input or empty arrays return `Self::default()`.
    #[cfg(feature = "json")]
    fn parse(value: JsonValue, context: &mut BaseProviderContext) -> Self
    where
        Self: Sized,
    {
        let root: JsonValue = match value.as_array().unwrap()[0] {
            JsonValue::Array(_) => value,
            _ => {
                let mut cloned: Vec<JsonValue> = value.as_array().unwrap().clone();
                cloned.push(json!(0));
                json!([cloned])
            }
        };

        let Some(array) = root.as_array() else {
            return Self::default();
        };

        let mut points: Vec<Self::PointData> = vec![];
        for raw_point in array {
            if raw_point.is_null() {
                continue;
            }

            let mut easing = Functions::EaseLinear;
            let mut modifiers: Option<Vec<Self::Modifier>> = None;
            let mut flags: Option<Vec<String>> = None;
            let mut vals: Option<Vec<ValueProvider>> = None;

            // Group the values and flags. (Assuming each raw_point has a structure similar to the C++ JSON)
            for group in group_values(raw_point) {
                match group.0 {
                    GroupType::Value => {
                        use crate::prelude::deserialize_values;

                        vals = Some(deserialize_values(&group.1, context));
                    }
                    GroupType::Modifier => {
                        modifiers = Some(
                            group
                                .1
                                .iter()
                                .map(|m| Self::deserialize_modifier(m, context))
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

            let point_data = Self::create_point_data(
                vs,
                flags.unwrap_or_default(),
                modifiers.unwrap_or_default(),
                easing,
                context,
            );
            points.push(point_data);
        }

        Self::new(points)
    }

    // The main interpolation method. Returns a tuple (interpolated value, is_last_point)
    fn interpolate(&self, time: f32, context: &BaseProviderContext) -> (T, bool) {
        let points = self.get_points();

        if points.is_empty() {
            return (T::default(), true);
        }

        let last_point = points.last().unwrap();
        if last_point.get_time() <= time {
            return (last_point.get_point(context), true);
        }

        let first_point = points.first().unwrap();
        if first_point.get_time() >= time {
            return (first_point.get_point(context), false);
        }

        let (l, r) = search_index(points, time);
        let point_l = &points[l];
        let point_r = &points[r];

        let time_delta = point_r.get_time() - point_l.get_time();
        let normal_time = if time_delta != 0.0 {
            (time - point_l.get_time()) / (time_delta)
        } else {
            0.0
        };

        let eased_time = point_r.get_easing().interpolate(normal_time);
        (
            self.interpolate_points(point_l, point_r, l, r, eased_time, context),
            false,
        )
    }
}

// Binary search algorithm to find the relevant interval
fn search_index<P: PointDataLike<T>, T>(points: &[P], time: f32) -> (usize, usize) {
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
