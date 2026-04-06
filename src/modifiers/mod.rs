pub mod modifier;
pub mod operation;
pub mod quaternion_modifier;

use modifier::BasicModifier;
use glam::{Quat, Vec3, Vec4};
use smallvec::SmallVec;

use crate::base_provider_context::BaseProviderContext;
use crate::ffi::types::WrapBaseValue;
use crate::modifiers::operation::Operation;
use crate::modifiers::quaternion_modifier::QuaternionModifier;
use crate::providers::value::BaseValue;
use crate::providers::{AbstractValueProvider, ValueProvider};
use crate::values::ValueType;

pub type BaseModifier = BasicModifier<BaseValue>;

#[derive(Clone, Debug)]
pub enum ModifierValues<T> {
    Static(T),
    Dynamic(Vec<ValueProvider>),
}

/// Modifiers are added at the end of points to allow you to do basic arithmetic on points.
///  How these modifiers interact can be defined using operations, all of which are done componentwise.
// #[derive(Debug)]
// pub enum BaseModifier {
//     Float(Modifier<f32>),
//     Vector3(Modifier<Vec3>),
//     Vector4(Modifier<Vec4>),
//     Quaternion(QuaternionModifier),
// }

impl BaseModifier {
    pub fn get_float(&self, context: &BaseProviderContext) -> f32 {
        self.get_point(context).as_float().expect("not a float but tried to use as float")
    }

    pub fn get_vector3(&self, context: &BaseProviderContext) -> Vec3 {
        self.get_point(context).as_vec3().expect("not a vector3 but tried to use as vector3")
    }

    pub fn get_vector4(&self, context: &BaseProviderContext) -> Vec4 {
        self.get_point(context).as_vec4().expect("not a vector4 but tried to use as vector4")
    }

    pub fn get_quaternion(&self, context: &BaseProviderContext) -> Quat {
        self.get_point(context).as_quat().expect("not a quaternion but tried to use as quaternion")
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

pub trait ModifierLike {
    type Value;
    const VALUE_COUNT: usize;

    fn get_point(&self, context: &BaseProviderContext) -> Self::Value;
    fn get_raw_point(&self) -> Self::Value;
    fn translate(&self, values: &[f32]) -> Self::Value;
    fn get_operation(&self) -> Operation;
    fn has_base_provider(&self) -> bool;

    fn fill_values(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> SmallVec<[f32; 4]> {
        let mut values = SmallVec::with_capacity(Self::VALUE_COUNT);
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

    // Default convert implementation avoids heap allocations by filling a fixed-size
    // stack buffer (max 4 components) and passing a slice to `translate`.
    fn convert(&self, ivals: &[ValueProvider], context: &BaseProviderContext) -> Self::Value {
        let values = self.fill_values(ivals, context);
        self.translate(&values)
    }
}

pub fn shared_has_base_provider<T: ModifierLike>(is_dynamic: bool, modifiers: &[T]) -> bool {
    match is_dynamic {
        true => true,
        false => modifiers.iter().any(|m| m.has_base_provider()),
    }
}
