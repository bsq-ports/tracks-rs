pub mod float_point_definition;
pub mod quaternion_point_definition;
pub mod vector3_point_definition;
pub mod vector4_point_definition;

use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

use serde_json::Value as JsonValue;
use serde_json::json;

use crate::point_data::PointData;
use crate::values::value::BaseValue;
use crate::{
    easings::functions::Functions,
    modifiers::{Modifier, operation::Operation},
    values::{ValueProvider, base_provider_context::BaseProviderContext, deserialize_values},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GroupType {
    Value,
    Flag,
    Modifier,
}

// The combined PointDefinition trait (acting as both BasePointDefinition and the templated PointDefinition<T>)
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



pub enum BasePointDefinition {
    Float(float_point_definition::FloatPointDefinition),
    Vector3(vector3_point_definition::Vector3PointDefinition),
    Vector4(vector4_point_definition::Vector4PointDefinition),
    Quaternion(quaternion_point_definition::QuaternionPointDefinition),
}

impl PointDefinition for BasePointDefinition {
    type Value = BaseValue;

    fn get_count(&self) -> usize {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.get_count()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.get_count()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.get_count()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.get_count()
            }
        }
    }

    fn has_base_provider(&self) -> bool {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.has_base_provider()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.has_base_provider()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.has_base_provider()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.has_base_provider()
            }
        }
    }

    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Self::Value {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                let v = float_point_definition.interpolate_points(points, l, r, time, context);
                BaseValue::Float(v)
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                let v = vector3_point_definition.interpolate_points(points, l, r, time, context);
                BaseValue::Vector3(v)
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                let v = vector4_point_definition.interpolate_points(points, l, r, time, context);
                BaseValue::Vector4(v)
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                let v = quaternion_point_definition.interpolate_points(points, l, r, time, context);
                BaseValue::Quaternion(v)
            }
        }
    }

    fn create_modifier(
        &self,
        values: Vec<ValueProvider>,
        modifiers: Vec<Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Modifier {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.create_modifier(values, modifiers, operation, context)
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.create_modifier(values, modifiers, operation, context)
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.create_modifier(values, modifiers, operation, context)
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.create_modifier(values, modifiers, operation, context)
            }
        }
    }

    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.create_point_data(values, flags, modifiers, easing, context)
            }
            BasePointDefinition::Vector3(vector3_point_definition) => vector3_point_definition
                .create_point_data(values, flags, modifiers, easing, context),
            BasePointDefinition::Vector4(vector4_point_definition) => vector4_point_definition
                .create_point_data(values, flags, modifiers, easing, context),
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition
                    .create_point_data(values, flags, modifiers, easing, context)
            }
        }
    }

    fn get_points_mut(&mut self) -> &mut Vec<PointData> {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.get_points_mut()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.get_points_mut()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.get_points_mut()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.get_points_mut()
            }
        }
    }

    fn get_points(&self) -> &Vec<PointData> {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.get_points()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.get_points()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.get_points()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.get_points()
            }
        }
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> Self::Value {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                BaseValue::Float(float_point_definition.get_point(point, context))
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                BaseValue::Vector3(vector3_point_definition.get_point(point, context))
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                BaseValue::Vector4(vector4_point_definition.get_point(point, context))
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                BaseValue::Quaternion(quaternion_point_definition.get_point(point, context))
            }
        }
    }
}

impl From<float_point_definition::FloatPointDefinition> for BasePointDefinition {
    fn from(point_definition: float_point_definition::FloatPointDefinition) -> Self {
        BasePointDefinition::Float(point_definition)
    }
}
impl From<vector3_point_definition::Vector3PointDefinition> for BasePointDefinition {
    fn from(point_definition: vector3_point_definition::Vector3PointDefinition) -> Self {
        BasePointDefinition::Vector3(point_definition)
    }
}
impl From<vector4_point_definition::Vector4PointDefinition> for BasePointDefinition {
    fn from(point_definition: vector4_point_definition::Vector4PointDefinition) -> Self {
        BasePointDefinition::Vector4(point_definition)
    }
}   
impl From<quaternion_point_definition::QuaternionPointDefinition> for BasePointDefinition {
    fn from(point_definition: quaternion_point_definition::QuaternionPointDefinition) -> Self {
        BasePointDefinition::Quaternion(point_definition)
    }
}