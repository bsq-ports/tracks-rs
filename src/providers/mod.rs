use crate::base_provider_context::BaseProviderContext;
use std::{cell::RefCell, rc::Rc};

pub mod base;
#[cfg(feature = "ffi")]
pub mod base_ffi;
pub mod partial;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;

#[cfg(feature = "json")]
use serde_json::Value as JsonValue;
use smallvec::SmallVec;

// pub enum ValueProviderValues {
//     /// Represents an array of values, where each value is a float. The length of the array can vary,
//     ///  but it is typically used to represent a vector of values (e.g., Vec3 would have 3 values).
//     Vec(SmallVec<[f32; 5]>),
//     /// [T, time] e.g for a Vec3 it would be [x, y, z, time]
//     PointData(BaseValue, f32),
//     /// We know exactly the type of the value, so we can store it directly without the need for dynamic dispatch or type erasure.
//     BaseValues(BaseValue),
// }

pub type ValueProviderValues = SmallVec<[f32; 5]>;

/// Abstract value provider
/// that provides values
/// based on the context
/// and the values
pub trait AbstractValueProvider {
    /// Get an array of values
    /// the values are [T, time] e.g for a Vec3 it would be [x, y, z, time]
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues;

    fn is_rotation(&self, _context: &BaseProviderContext) -> bool {
        false
    }
}

/// Update values on demand
/// from the source values
///
/// Delta is time based
pub trait UpdateableValues: AbstractValueProvider {
    /// Update the values from the source
    /// delta is the amount to progress from the source to target
    fn update(&mut self, delta: f32, context: &BaseProviderContext);
}

/// Value provider
/// without virtual dispatch
#[derive(Clone, Debug)]
pub enum ValueProvider {
    Static(r#static::StaticValues),
    BaseProvider(base::BaseProviderValues),
    PartialProvider(partial::PartialProviderValues),
    SmoothProviders(Rc<RefCell<smooth::SmoothProvidersValues>>),
    SmoothRotationProviders(Rc<RefCell<smooth_rot::SmoothRotationProvidersValues>>),
}

impl AbstractValueProvider for ValueProvider {
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues {
        match self {
            ValueProvider::Static(v) => v.values(context),
            ValueProvider::BaseProvider(v) => v.values(context),
            ValueProvider::PartialProvider(v) => v.values(context),
            ValueProvider::SmoothProviders(v) => {
                let borrow = v.borrow();
                borrow.values(context)
            }
            ValueProvider::SmoothRotationProviders(v) => {
                let borrow = v.borrow();
                borrow.values(context)
            }
        }
    }
}

impl UpdateableValues for ValueProvider {
    fn update(&mut self, delta: f32, context: &crate::base_provider_context::BaseProviderContext) {
        match self {
            ValueProvider::Static(_) => {}
            ValueProvider::BaseProvider(_) => {}
            ValueProvider::PartialProvider(_v) => {}
            ValueProvider::SmoothProviders(v) => v.borrow_mut().update(delta, context),
            ValueProvider::SmoothRotationProviders(v) => v.borrow_mut().update(delta, context),
        }
    }
}

impl ValueProvider {
    /// Check if the provider is updateable
    pub fn is_updateable(&self) -> bool {
        matches!(
            self,
            ValueProvider::PartialProvider(_)
                | ValueProvider::SmoothProviders(_)
                | ValueProvider::SmoothRotationProviders(_)
        )
    }
}

// Values deserialization
/// Creates a new instance of [`BaseProviderValues`] using the provided base values.
///
/// # Arguments
///
/// * `base` - Clone of the base values used to initialize the provider.
#[cfg(feature = "json")]
pub fn deserialize_values(
    value: &[&JsonValue],
    context: &mut BaseProviderContext,
) -> Vec<ValueProvider> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, v) in value.iter().enumerate() {
        if let JsonValue::String(s) = v {
            close(&mut result, value.to_vec(), start, i);
            start = i + 1;

            let base = context.get_value_provider(s);
            result.push(base);
        }
    }

    close(&mut result, value.to_vec(), start, value.len());
    result
}

#[cfg(feature = "json")]
fn close(result: &mut Vec<ValueProvider>, raw_values: Vec<&JsonValue>, open: usize, end: usize) {
    use r#static::StaticValues;

    if end <= open {
        return;
    }

    let values: SmallVec<[f32; 4]> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();
    result.push(ValueProvider::Static(StaticValues::new(values)));
}
