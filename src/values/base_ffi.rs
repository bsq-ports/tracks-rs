use std::{
    borrow::Cow,
    ffi::{self, c_void},
    slice,
};

use crate::{ffi::types::WrappedValues, values::base_provider_context::BaseProviderContext};

use super::AbstractValueProvider;

pub type BaseFFIProvider = unsafe extern "C" fn(&BaseProviderContext, *mut c_void) -> WrappedValues;

pub struct BaseFFIProviderValues {
    pub fetch: *const BaseFFIProvider,
    pub user_data: *mut c_void,
}

impl BaseFFIProviderValues {
    pub fn new(fetch: *const BaseFFIProvider, user_data: *mut c_void) -> Self {
        Self { fetch, user_data }
    }
}

impl AbstractValueProvider for BaseFFIProviderValues {
    fn values<'a>(&'a self, context: &BaseProviderContext) -> Cow<'a, [f32]> {
        let c_values: WrappedValues = unsafe { (*self.fetch)(context, self.user_data) };
        // move to owned values
        let arr = unsafe { slice::from_raw_parts(c_values.values, c_values.length) };
        Cow::Owned(arr.to_vec())
    }
}
