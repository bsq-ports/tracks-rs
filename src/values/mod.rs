use crate::base_provider_context::BaseProviderContext;
use base::BaseProviderValues;
use serde_json::Value as JsonValue;
use std::borrow::Cow;

pub mod base;
#[cfg(feature = "ffi")]
pub mod base_ffi;
pub mod quat;
pub mod smooth;
pub mod smooth_rot;
pub mod r#static;
pub mod updatable;
pub mod value;

/// Abstract value provider
/// that provides values
/// based on the context
/// and the values
pub trait AbstractValueProvider {
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]>;
}

/// Update values on demand
/// from the source values
///
/// Delta is time based
pub trait UpdateableValues: AbstractValueProvider {
    /// Update the values from the source
    /// delta is the amount to progress from the source to target
    fn update(&mut self, delta: f32);
}

/// Value provider
/// without virtual dispatch
#[derive(Clone, Debug)]
pub enum ValueProvider {
    Static(r#static::StaticValues),
    BaseProvider(base::BaseProviderValues),
    QuaternionProvider(quat::QuaternionProviderValues),
    PartialProvider(updatable::PartialProviderValues),
    SmoothProviders(smooth::SmoothProvidersValues),
    SmoothRotationProviders(smooth_rot::SmoothRotationProvidersValues),
}

impl AbstractValueProvider for ValueProvider {
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]> {
        match self {
            ValueProvider::Static(v) => v.values(context),
            ValueProvider::BaseProvider(v) => v.values(context),
            ValueProvider::QuaternionProvider(v) => v.values(context),
            ValueProvider::PartialProvider(v) => v.values(context),
            ValueProvider::SmoothProviders(v) => v.values(context),
            ValueProvider::SmoothRotationProviders(v) => v.values(context),
        }
    }
}

impl UpdateableValues for ValueProvider {
    fn update(&mut self, delta: f32) {
        match self {
            ValueProvider::Static(_) => {}
            ValueProvider::BaseProvider(_) => {}
            ValueProvider::QuaternionProvider(_) => {}
            ValueProvider::PartialProvider(v) => v.update(delta),
            ValueProvider::SmoothProviders(v) => v.update(delta),
            ValueProvider::SmoothRotationProviders(v) => v.update(delta),
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

// Helper function for linear interpolation
fn clamp_lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
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

    let values: Vec<f32> = raw_values[open..end]
        .iter()
        .filter_map(|v| v.as_f64().map(|i| i as f32))
        .collect();
    result.push(ValueProvider::Static(StaticValues::new(&values)));
}
