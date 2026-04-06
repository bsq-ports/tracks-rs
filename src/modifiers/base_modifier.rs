use glam::{Quat, Vec3, Vec4};

use crate::{
    modifiers::{
        ModifierLike, modifier::BasicModifier, operation::Operation,
        quaternion_modifier::QuaternionModifier,
    },
    prelude::BaseProviderContext,
    providers::value::BaseValue,
};

/// Modifiers are added at the end of points to allow you to do basic arithmetic on points.
///  How these modifiers interact can be defined using operations, all of which are done componentwise.
#[derive(Debug)]
pub enum BaseModifier {
    Float(BasicModifier<f32>),
    Vector3(BasicModifier<Vec3>),
    Vector4(BasicModifier<Vec4>),
    Quaternion(QuaternionModifier),
}

impl BaseModifier {
    pub fn get_float(&self, context: &BaseProviderContext) -> f32 {
        self.get_modified_point(context)
            .as_float()
            .expect("not a float but tried to use as float")
    }

    pub fn get_vector3(&self, context: &BaseProviderContext) -> Vec3 {
        self.get_modified_point(context)
            .as_vec3()
            .expect("not a vector3 but tried to use as vector3")
    }

    pub fn get_vector4(&self, context: &BaseProviderContext) -> Vec4 {
        self.get_modified_point(context)
            .as_vec4()
            .expect("not a vector4 but tried to use as vector4")
    }

    pub fn get_quaternion(&self, context: &BaseProviderContext) -> Quat {
        self.get_modified_point(context)
            .as_quat()
            .expect("not a quaternion but tried to use as quaternion")
    }
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
