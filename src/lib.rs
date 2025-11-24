#![feature(impl_trait_in_assoc_type)]
#![feature(slice_pattern)]
#![feature(new_range_api)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(box_into_inner)]
#![feature(unboxed_closures)]

#[cfg(feature = "ffi")]
pub mod ffi;

pub mod animation;
pub mod base_provider_context;
pub mod easings;
pub mod modifiers;
pub mod point_data;
pub mod point_definition;
pub mod values;
