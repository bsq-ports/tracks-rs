#![feature(let_chains)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_pattern)]
#![feature(new_range_api)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(box_into_inner)]

use std::{backtrace::Backtrace, panic::PanicHookInfo};

use tracing::error;
use tracing_error::SpanTrace;

pub mod context;
pub mod easings;

#[cfg(feature = "ffi")]
pub mod ffi;

pub mod modifiers;
pub mod point_data;
pub mod point_definition;
pub mod values;
pub mod animation;
pub mod base_provider_context;


#[cfg(target_os = "android")]
#[ctor::ctor]
fn main() {
    use tracing::info;

    paranoid_android::init("tracks");
    std::panic::set_hook(panic_hook(true, true));

    info!("setup HI");
}

/// Returns a panic handler, optionally with backtrace and spantrace capture.
pub fn panic_hook(
    backtrace: bool,
    spantrace: bool,
) -> Box<dyn Fn(&PanicHookInfo) + Send + Sync + 'static> {
    // Mostly taken from https://doc.rust-lang.org/src/std/panicking.rs.html
    Box::new(move |info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<dyn Any>",
            },
        };

        error!(target: "panic", "panicked at '{}', {}", msg, location);
        if backtrace {
            error!(target: "panic", "{:?}", Backtrace::force_capture());
        }
        if spantrace {
            error!(target: "panic", "{:?}", SpanTrace::capture());
        }
    })
}
