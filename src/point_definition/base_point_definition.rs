use glam::{Quat, Vec3, Vec4};

use crate::easings::functions::Functions;

use crate::modifiers::operation::Operation;

use crate::modifiers::BaseModifier;

use crate::point_definition::basic_point_definition::BasicPointDefinition;
use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;
use crate::providers::ValueProvider;

use crate::base_provider_context::BaseProviderContext;

use crate::point_data::BasePointData;

use crate::providers::value::BaseValue;

use super::PointDefinitionLike;

/// Point definitions are used to describe what happens over the course of an animation,
/// they are used slightly differently for different properties.
/// They consist of a collection of points over time.
#[derive(Debug, Clone)]
pub enum BasePointDefinition {
    Float(BasicPointDefinition<f32>),
    Vector3(BasicPointDefinition<Vec3>),
    Vector4(BasicPointDefinition<Vec4>),
    Quaternion(QuaternionPointDefinition),
}



impl PointDefinitionLike for BasePointDefinition {
    type Value = BaseValue;
    type PointData = BasePointData;
    type Modifer = BaseModifier;

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
        points: &[BasePointData],
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
        _values: Vec<ValueProvider>,
        _modifiers: Vec<BaseModifier>,
        _operation: Operation,
        _context: &BaseProviderContext,
    ) -> BaseModifier {
        unimplemented!(
            "Cannot create Modifier directly from BasePointDefinition; use specific point definition types instead."
        )
    }

    fn create_point_data(
        _values: Vec<ValueProvider>,
        _flags: Vec<String>,
        _modifiers: Vec<BaseModifier>,
        _easing: Functions,
        _context: &BaseProviderContext,
    ) -> BasePointData {
        unimplemented!(
            "Cannot create PointData directly from BasePointDefinition; use specific point definition types instead."
        )
    }

    fn get_points(&self) -> &[BasePointData] {
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

    fn get_point(&self, point: &BasePointData, context: &BaseProviderContext) -> Self::Value {
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

    fn new(_points: Vec<BasePointData>) -> Self {
        unimplemented!(
            "Cannot create BasePointDefinition directly; use specific point definition types instead."
        )
    }
}

impl Default for BasePointDefinition {
    fn default() -> Self {
        BasePointDefinition::Float(Default::default())
    }
}

impl From<QuaternionPointDefinition> for BasePointDefinition {
    fn from(point_definition: QuaternionPointDefinition) -> Self {
        BasePointDefinition::Quaternion(point_definition)
    }
}
