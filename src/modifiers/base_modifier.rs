use glam::{Vec3, Vec4};

use crate::{
    base_value::BaseValue,
    modifiers::{
        ModifierLike, basic_modifier::BasicModifier, operation::Operation,
        quaternion_modifier::QuaternionModifier,
    },
    prelude::BaseProviderContext,
};

/// Modifiers represent small arithmetic transformations applied to point data.
///
/// These are parsed from the end of a point definition and are evaluated at
/// runtime against a `BaseProviderContext`. Modifiers operate component-wise
/// according to an `Operation` and may be typed (float, vec3, vec4, quaternion).
///
/// A typed wrapper around the concrete modifier implementations.
///
/// - `Float`: scalar modifier (`BasicModifier<f32>`)
/// - `Vector3`: 3-component modifier (`BasicModifier<Vec3>`)
/// - `Vector4`: 4-component modifier (`BasicModifier<Vec4>`)
/// - `Quaternion`: quaternion-specific modifier (`QuaternionModifier`)
///
/// Use the typed getters (`get_float`, `get_vector3`, ...) or the
/// `ModifierLike` methods to evaluate the modifier.
#[derive(Debug)]
pub enum BaseModifier {
    Float(BasicModifier<f32>),
    Vector3(BasicModifier<Vec3>),
    Vector4(BasicModifier<Vec4>),
    Quaternion(QuaternionModifier),
}

impl ModifierLike<BaseValue> for BaseModifier {
    // max of the value counts of the modifiers, used for filling values in translate
    const VALUE_COUNT: usize = 4;

    fn get_modified_point(&self, context: &BaseProviderContext) -> BaseValue {
        match self {
            BaseModifier::Float(modifier) => modifier.get_modified_point(context).into(),
            BaseModifier::Vector3(modifier) => modifier.get_modified_point(context).into(),
            BaseModifier::Vector4(modifier) => modifier.get_modified_point(context).into(),
            BaseModifier::Quaternion(modifier) => modifier.get_modified_point(context).into(),
        }
    }

    fn get_raw_point(&self) -> BaseValue {
        match self {
            BaseModifier::Float(modifier) => modifier.get_raw_point().into(),
            BaseModifier::Vector3(modifier) => modifier.get_raw_point().into(),
            BaseModifier::Vector4(modifier) => modifier.get_raw_point().into(),
            BaseModifier::Quaternion(modifier) => modifier.get_raw_point().into(),
        }
    }

    fn get_operation(&self) -> Operation {
        match self {
            BaseModifier::Float(modifier) => modifier.get_operation(),
            BaseModifier::Vector3(modifier) => modifier.get_operation(),
            BaseModifier::Vector4(modifier) => modifier.get_operation(),
            BaseModifier::Quaternion(modifier) => modifier.get_operation(),
        }
    }

    fn has_base_provider(&self) -> bool {
        match self {
            BaseModifier::Float(modifier) => modifier.has_base_provider(),
            BaseModifier::Vector3(modifier) => modifier.has_base_provider(),
            BaseModifier::Vector4(modifier) => modifier.has_base_provider(),
            BaseModifier::Quaternion(modifier) => modifier.has_base_provider(),
        }
    }
}
