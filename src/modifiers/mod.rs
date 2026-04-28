pub mod basic_modifier;
pub mod operation;

pub mod base_modifier;
pub mod quaternion_modifier;

use smallvec::SmallVec;

use crate::base_provider_context::BaseProviderContext;
use crate::modifiers::operation::Operation;
use crate::providers::{AbstractValueProvider, ValueProvider};

/// Representation of modifier input values.
///
/// Modifier values may be either a `Static` value (fully resolved numeric
/// data), or `Dynamic` where the value is described by one or more
/// `ValueProvider`s that must be evaluated against a `BaseProviderContext` at
/// runtime. The `SmallVec` optimizes the common single-provider case.
#[derive(Clone, Debug)]
pub enum ModifierValues<T> {
    /// Pre-computed static value.
    Static(T),
    /// Dynamic providers that produce the value when evaluated.
    Dynamic(SmallVec<[ValueProvider; 1]>),
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

/// Trait implemented by concrete modifier types.
///
/// `ModifierLike` provides the core operations a modifier must support:
/// - `VALUE_COUNT` describes how many numeric components the modifier can produce
///   (e.g., `1` for a float, `3` for a Vec3).
/// - `get_raw_point` returns the un-evaluated base point represented by the
///   modifier (useful for static inspection).
/// - `get_modified_point` evaluates the modifier against a `BaseProviderContext`
///   and returns the resulting typed value.
/// - `has_base_provider` and `get_operation` expose metadata used during parsing
///   and composition of modifiers.
pub trait ModifierLike<T> {
    /// Number of components produced by this modifier type.
    const VALUE_COUNT: usize;

    /// Get the raw (un-evaluated) point value.
    fn get_raw_point(&self) -> T;

    /// Evaluate the modifier against `context` and return the typed value.
    fn get_modified_point(&self, context: &BaseProviderContext) -> T;

    /// Whether this modifier depends on a base provider (strings like `baseX`).
    fn has_base_provider(&self) -> bool;

    /// The component-wise operation that composes this modifier (add, mul, ...).
    fn get_operation(&self) -> Operation;

    /// Helper to translate a slice of `ValueProvider`s into a fixed-size array of
    /// `f32` components used by modifiers. Providers are evaluated in order and
    /// fill the returned array up to `VALUE_COUNT`.
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

/// Shared helper used while parsing to determine if any modifier depends on a
/// base provider. If the point is already dynamic (`is_dynamic == true`) this
/// returns `true` immediately; otherwise it inspects the provided modifiers.
pub fn shared_has_base_provider<T: ModifierLike<V>, V>(is_dynamic: bool, modifiers: &[T]) -> bool {
    match is_dynamic {
        true => true,
        false => modifiers.iter().any(|m| m.has_base_provider()),
    }
}
