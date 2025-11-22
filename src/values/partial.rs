use std::{borrow::Cow, cell::RefCell};

use crate::base_provider_context::BaseProviderContext;

use super::UpdateableValues;

use super::AbstractValueProvider;

#[derive(Clone, Debug)]
pub struct PartialProviderValues {
    pub(crate) source: Vec<f32>,
    pub(crate) parts: Vec<usize>,
    pub(crate) values: Vec<f32>,
}

impl PartialProviderValues {
    pub fn new(source: Vec<f32>, parts: Vec<usize>) -> Self {
        Self {
            source,
            values: vec![0.0; parts.len()],
            parts,
        }
    }
}

impl AbstractValueProvider for PartialProviderValues {
    fn values<'a>(&'a self, _context: &BaseProviderContext) -> Cow<'a, [f32]> {
        std::borrow::Cow::Borrowed(&self.source)
    }
}

impl UpdateableValues for PartialProviderValues {
    fn update(&mut self, _t: f32) {
        for (i, &part) in self.parts.iter().enumerate() {
            self.values[i] = self.source[part];
        }
    }
}
