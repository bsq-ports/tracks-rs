use std::{ffi::c_void, slice};

use smallvec::SmallVec;

use crate::{
    base_provider_context::BaseProviderContext, ffi::types::WrappedValues,
    providers::ValueProviderValues,
};

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
    fn values(&self, context: &BaseProviderContext) -> ValueProviderValues {
        let c_values: WrappedValues = unsafe { (*self.fetch)(context, self.user_data) };
        // move to owned values
        let arr = unsafe { slice::from_raw_parts(c_values.values, c_values.length) };
        SmallVec::from_slice(arr)
    }
    
    fn is_rotation(&self, _context: &BaseProviderContext) -> bool {
        // FFI providers are assumed to be non-rotational by default, but this can be overridden by the provider implementation if needed.
        // TODO:
        false
    }
}
