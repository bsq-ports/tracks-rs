use std::borrow::Cow;

use super::{UpdateableValues, clamp_lerp};

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct SmoothProvidersValues {
    pub(crate) source: Vec<f32>,
    pub(crate) mult: f32,
    pub(crate) values: Vec<f32>,
}

impl SmoothProvidersValues {
    pub fn new(source: Vec<f32>, mult: f32) -> Self {
        Self {
            source: source.clone(),
            mult,
            values: vec![0.0; source.len()],
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        Cow::Borrowed(self.values.as_ref())
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32) {
        for i in 0..self.source.len() {
            self.values[i] = clamp_lerp(self.values[i], self.source[i], delta);
        }
    }
}
