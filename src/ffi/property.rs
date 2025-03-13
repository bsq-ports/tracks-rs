use crate::animation::property::PathProperty;
use crate::ffi::types::{WrapBaseValue, WrapBaseValueType};
use crate::values::base_provider_context::BaseProviderContext;
use crate::values::value::BaseValue;

#[repr(C)]
pub struct CValueProperty {
    has_value: bool,
    value: WrapBaseValue,
}

impl From<Option<BaseValue>> for CValueProperty {
    fn from(prop: Option<BaseValue>) -> Self {
        match prop {
            Some(base_value) => CValueProperty {
                has_value: true,
                value: WrapBaseValue::from(base_value),
            },
            None => CValueProperty {
                has_value: false,
                value: WrapBaseValue {
                    ty: WrapBaseValueType::Float,
                    value: unsafe { std::mem::zeroed() },
                },
            },
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn path_property_create() -> *mut PathProperty<'static> {
    Box::into_raw(Box::new(PathProperty::default()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_finish(ptr: *mut PathProperty) {
    if !ptr.is_null() {
        unsafe {
            let inner = &mut *ptr;
            inner.finish();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_free(ptr: *mut PathProperty) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_get_time(ptr: *const PathProperty) -> f32 {
    if ptr.is_null() {
        return 0.0;
    }
    unsafe {
        let inner = &*ptr;
        inner.time
    }
}

#[unsafe(no_mangle)]
pub unsafe  extern "C" fn path_property_set_time(ptr: *mut PathProperty, time: f32) {
    if !ptr.is_null() {
        unsafe {
            let inner = &mut *ptr;
            inner.time = time;
        }
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn path_property_interpolate(ptr: *mut PathProperty, time: f32, context: *mut BaseProviderContext) {
    if ptr.is_null() || context.is_null() {
        return;
    }
    unsafe {
        let context = &mut *context;
        let inner = &mut *ptr;
        inner.interpolate(time, context);
    }
}
