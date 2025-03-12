use std::{
    ffi::{CStr, c_char},
    slice,
};

use super::types;

/// JSON FFI
#[repr(C)]
#[derive(Debug)]
pub enum JsonValueType {
    Number,
    Null,
    String,
    Array,
}

#[repr(C)]
pub struct FFIJsonValue {
    pub value_type: JsonValueType,
    pub data: JsonValueData,
}

#[repr(C)]
pub union JsonValueData {
    pub number_value: f64,
    pub string_value: *const c_char,
    pub array: *const JsonArray,
}

#[repr(C)]
pub struct JsonArray {
    pub elements: *const FFIJsonValue,
    pub length: usize,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_number(value: f64) -> FFIJsonValue {
    FFIJsonValue {
        value_type: JsonValueType::Number,
        data: JsonValueData {
            number_value: value,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_string(value: *const c_char) -> FFIJsonValue {
    FFIJsonValue {
        value_type: JsonValueType::String,
        data: JsonValueData {
            string_value: value,
        },
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_create_json_array(
    elements: *const FFIJsonValue,
    length: usize,
) -> FFIJsonValue {
    // Allocate JsonArray on the heap so it persists after function returns
    let array = Box::new(JsonArray { elements, length });

    // Leak the box to keep it alive (caller is responsible for freeing)
    let array_ptr = Box::into_raw(array);

    FFIJsonValue {
        value_type: JsonValueType::Array,
        data: JsonValueData { array: array_ptr },
    }
}

/// Convert the FFI JsonValue to a serde_json::Value
pub(crate) unsafe fn convert_json_value_to_serde(
    json_value: *const FFIJsonValue,
) -> serde_json::Value {
    if json_value.is_null() {
        return serde_json::Value::Null;
    }

    let json_value = unsafe { &*json_value };
    match json_value.value_type {
        JsonValueType::Null => serde_json::Value::Null,
        JsonValueType::Number => serde_json::Value::Number(
            serde_json::Number::from_f64(unsafe { json_value.data.number_value }).unwrap(),
        ),
        JsonValueType::String => {
            let c_str = unsafe { CStr::from_ptr(json_value.data.string_value) };
            let str_slice = c_str.to_str().unwrap_or_default();
            serde_json::Value::String(str_slice.to_owned())
        }
        JsonValueType::Array => {
            let array_ptr = unsafe { json_value.data.array };
            if array_ptr.is_null() {
                return serde_json::Value::Array(Vec::new());
            }

            let array = unsafe { &*array_ptr };

            // Validate array length - prevent unreasonable allocations
            // 10 million elements should be more than enough for any reasonable JSON array
            // while preventing buffer overflows from corrupted memory
            const MAX_SAFE_ARRAY_LENGTH: usize = 10_000_000;

            if array.elements.is_null() || array.length == 0 || array.length > MAX_SAFE_ARRAY_LENGTH
            {
                println!(
                    "Invalid array length or null elements pointer: {}",
                    array.length
                );
                return serde_json::Value::Array(Vec::new());
            }

            // Create a safe slice from the raw parts
            let elements = unsafe { slice::from_raw_parts(array.elements, array.length) };
            let mut json_array = Vec::new(); // Don't pre-allocate with potentially corrupted capacity

            for element in elements.iter() {
                json_array.push(unsafe { convert_json_value_to_serde(element) });
            }

            serde_json::Value::Array(json_array)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracks_free_json_value(json_value: *mut FFIJsonValue) {
    // Free the memory allocated for the JsonValue
    // This is a simple implementation that doesn't handle nested structures
    // For a complete implementation, you would need to recursively free all nested elements
    if !json_value.is_null() {
        drop(unsafe { Box::from_raw(json_value) });
    }
}
