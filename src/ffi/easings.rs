use std::os::raw::c_float;

use crate::easings::functions::Functions;

/// C-compatible wrapper for easing functions
#[unsafe(no_mangle)]
pub extern "C" fn interpolate_easing(easing_function: Functions, t: c_float) -> c_float {
    easing_function.interpolate(t)
}

/// Gets an easing function by index (useful for FFI where enums might be troublesome)
/// Returns Functions::EaseLinear if the index is out of bounds
#[unsafe(no_mangle)]
pub extern "C" fn get_easing_function_by_index(index: i32) -> Functions {
    match index {
        0 => Functions::EaseLinear,
        1 => Functions::EaseStep,
        2 => Functions::EaseInQuad,
        3 => Functions::EaseOutQuad,
        4 => Functions::EaseInOutQuad,
        5 => Functions::EaseInCubic,
        6 => Functions::EaseOutCubic,
        7 => Functions::EaseInOutCubic,
        8 => Functions::EaseInQuart,
        9 => Functions::EaseOutQuart,
        10 => Functions::EaseInOutQuart,
        11 => Functions::EaseInQuint,
        12 => Functions::EaseOutQuint,
        13 => Functions::EaseInOutQuint,
        14 => Functions::EaseInSine,
        15 => Functions::EaseOutSine,
        16 => Functions::EaseInOutSine,
        17 => Functions::EaseInCirc,
        18 => Functions::EaseOutCirc,
        19 => Functions::EaseInOutCirc,
        20 => Functions::EaseInExpo,
        21 => Functions::EaseOutExpo,
        22 => Functions::EaseInOutExpo,
        23 => Functions::EaseInElastic,
        24 => Functions::EaseOutElastic,
        25 => Functions::EaseInOutElastic,
        26 => Functions::EaseInBack,
        27 => Functions::EaseOutBack,
        28 => Functions::EaseInOutBack,
        29 => Functions::EaseInBounce,
        30 => Functions::EaseOutBounce,
        31 => Functions::EaseInOutBounce,
        _ => Functions::EaseLinear,
    }
}

/// Gets the total number of available easing functions
#[unsafe(no_mangle)]
pub extern "C" fn get_easing_function_count() -> i32 {
    32 // Update this if you add more functions
}
