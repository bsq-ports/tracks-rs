use std::{
    borrow::Cow,
    cell::{RefCell, RefMut},
};

use super::{UpdateableValues, ValueProvider, clamp_lerp};

use crate::base_provider_context::BaseProviderContext;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct SmoothProvidersValues {
    pub(crate) source: Box<ValueProvider>,
    pub(crate) mult: f32,
    pub(crate) values: Vec<f32>,
    pub(crate) has_updated: bool,
}

impl SmoothProvidersValues {
    pub fn new(source: Box<ValueProvider>, mult: f32) -> Self {
        Self {
            source,
            mult,
            values: Vec::new(),
            has_updated: false,
        }
    }
}

impl AbstractValueProvider for SmoothProvidersValues {
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]> {
        println!("SmoothProvidersValues values called {}", self.has_updated);
        let mut values = self.values.clone();
        if values.len() != self.source.values(context).len() {
            values = vec![0.0; self.source.values(context).len()];
        }
        values.into()
    }
}

impl UpdateableValues for SmoothProvidersValues {
    fn update(&mut self, delta: f32, context: &BaseProviderContext) {
        println!("SmoothProvidersValues update called {}", self.has_updated);
        self.has_updated = true;
        let source_values = self.source.values(context);
        if self.values.len() != source_values.len() {
            self.values = vec![0.0; source_values.len()];
        }
        for i in 0..self.values.len() {
            self.values[i] = clamp_lerp(self.values[i], source_values[i], delta);
        }
    }
}
