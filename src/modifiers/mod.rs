pub mod modifier;
pub mod operation;

pub mod base_modifier;
pub mod quaternion_modifier;

use crate::base_provider_context::BaseProviderContext;
use crate::modifiers::operation::Operation;
use crate::providers::{AbstractValueProvider, ValueProvider};

#[derive(Clone, Debug)]
pub enum ModifierValues<T> {
    Static(T),
    Dynamic(Vec<ValueProvider>),
}

impl<T> ModifierValues<T> {
    pub fn into_static_values(self) -> Option<T> {
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

pub trait ModifierLike<T> {
    const VALUE_COUNT: usize;

    fn get_raw_point(&self) -> T;
    fn get_modified_point(&self, context: &BaseProviderContext) -> T;

    fn has_base_provider(&self) -> bool;
    fn get_operation(&self) -> Operation;

    fn apply(
        &self,
        ivals: &[ValueProvider],
        context: &BaseProviderContext,
    ) -> [f32; Self::VALUE_COUNT] {
        let mut values = [0.0; Self::VALUE_COUNT];
        let mut i = 0;
        for value in ivals {
            for v in value.values(context) {
                if i >= Self::VALUE_COUNT {
                    break;
                }
                values[i] = v;
                i += 1;
            }
        }
        values
    }
}

pub fn shared_has_base_provider<T: ModifierLike<V>, V>(is_dynamic: bool, modifiers: &[T]) -> bool {
    match is_dynamic {
        true => true,
        false => modifiers.iter().any(|m| m.has_base_provider()),
    }
}
