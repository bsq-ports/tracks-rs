use crate::easings::functions::Functions;

use crate::modifiers::operation::Operation;

use crate::modifiers::Modifier;

use crate::values::ValueProvider;

use crate::base_provider_context::BaseProviderContext;

use crate::point_data::PointData;

use crate::values::value::BaseValue;

use super::{
    PointDefinition, float_point_definition, quaternion_point_definition, vector3_point_definition,
    vector4_point_definition,
};

#[derive(Debug)]
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
    
    fn get_type(&self) -> crate::ffi::types::WrapBaseValueType {
        match self {
            BasePointDefinition::Float(_) => crate::ffi::types::WrapBaseValueType::Float,
            BasePointDefinition::Vector3(_) => crate::ffi::types::WrapBaseValueType::Vec3,
            BasePointDefinition::Vector4(_) => crate::ffi::types::WrapBaseValueType::Vec4,
            BasePointDefinition::Quaternion(_) => crate::ffi::types::WrapBaseValueType::Quat,
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
