pub mod float_modifier;
pub mod operation;
pub mod quaternion_modifier;
pub mod vector3_modifier;
pub mod vector4_modifier;

use float_modifier::FloatModifier;
use glam::{Quat, Vec3, Vec4};
use quaternion_modifier::QuaternionModifier;
use vector3_modifier::Vector3Modifier;
use vector4_modifier::Vector4Modifier;

use crate::base_provider_context::BaseProviderContext;
use crate::modifiers::operation::Operation;
use crate::values::{AbstractValueProvider, ValueProvider};

#[derive(Clone, Debug)]
pub enum ModifierValues<T> {
    Static(T),
    Dynamic(Vec<ValueProvider>),
}

pub enum Modifier {
    Float(FloatModifier),
    Vector3(Vector3Modifier),
    Vector4(Vector4Modifier),
    Quaternion(QuaternionModifier),
}

impl Modifier {
    pub fn get_float(&self, context: &BaseProviderContext) -> f32 {
        if let Modifier::Float(modifier) = self {
            modifier.get_point(context)
        } else {
            panic!("Invalid modifier type");
        }
    }

    pub fn get_vector3(&self, context: &BaseProviderContext) -> Vec3 {
        if let Modifier::Vector3(modifier) = self {
            modifier.get_point(context)
        } else {
            panic!("Invalid modifier type");
        }
    }

    pub fn get_vector4(&self, context: &BaseProviderContext) -> Vec4 {
        if let Modifier::Vector4(modifier) = self {
            modifier.get_point(context)
        } else {
            panic!("Invalid modifier type");
        }
    }

    pub fn get_quaternion(&self, context: &BaseProviderContext) -> Quat {
        if let Modifier::Quaternion(modifier) = self {
            modifier.get_point(context)
        } else {
            panic!("Invalid modifier type");
        }
    }

    pub fn get_operation(&self) -> Operation {
        match self {
            Modifier::Float(modifier) => modifier.get_operation(),
            Modifier::Vector3(modifier) => modifier.get_operation(),
            Modifier::Vector4(modifier) => modifier.get_operation(),
            Modifier::Quaternion(modifier) => modifier.get_operation(),
        }
    }

    pub fn has_base_provider(&self) -> bool {
        match self {
            Modifier::Float(modifier) => modifier.has_base_provider(),
            Modifier::Vector3(modifier) => modifier.has_base_provider(),
            Modifier::Vector4(modifier) => modifier.has_base_provider(),
            Modifier::Quaternion(modifier) => modifier.has_base_provider(),
        }
    }
}

impl<T> ModifierValues<T> {
    pub fn static_values(self) -> Option<T> {
        match self {
            ModifierValues::Static(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_static_values(&self) -> Option<&T> {
        match self {
            ModifierValues::Static(s) => Some(s),
            _ => None,
        }
    }
}

pub trait ModifierBase {
    type Value;
    const VALUE_COUNT: usize;

    fn get_point(&self, context: &BaseProviderContext) -> Self::Value;
    fn get_raw_point(&self) -> Self::Value;
    fn translate(&self, values: &[f32]) -> Self::Value;
    fn get_operation(&self) -> Operation;
    fn has_base_provider(&self) -> bool;

    fn fill_values(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> Vec<f32> {
        let mut values = Vec::with_capacity(Self::VALUE_COUNT);
        for value in ivals {
            for v in value.values(context).iter().copied() {
                if values.len() < Self::VALUE_COUNT {
                    values.push(v);
                } else {
                    return values;
                }
            }
        }
        values
    }

    fn convert(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> Self::Value {
        let values = self.fill_values(ivals, context);
        self.translate(&values)
    }
}

pub fn shared_has_base_provider(is_dynamic: bool, modifiers: &[Modifier]) -> bool {
    match is_dynamic {
        true => true,
        false => modifiers.iter().any(|m| m.has_base_provider()),
    }
}
