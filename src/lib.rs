//! tracks-rs — overview and quick start pointers
//!
//! This crate implements the runtime for Heck-style Tracks & Points animations.
//! Key areas and entry points to study:
//!
//! - `base_provider_context` — runtime "bases" (score, colors, transforms, time).
//!   - Creation: `BaseProviderContext::new()` (Rust) or `base_provider_context_create()` (FFI).
//!   - Set/get: `set_values` / `get_values`, and FFI wrappers in `src/ffi/base_provider_context.rs`.
//! - `point_definition` — parsing and sampling of point definitions. See the `PointDefinition` trait
//!   (`parse`, `interpolate` / `interpolate_points`) and concrete implementations in
//!   `src/point_definition/*` (float, vec3, vec4, quaternion).
//! - `point_data` — the concrete time/value storage used by definitions at sample time.
//! - `values` — deserializes JSON point values into `ValueProvider`s. String-to-base conversion
//!   happens in `deserialize_values()` which calls `BaseProviderContext::get_value_provider()`.
//! - `modifiers` — implement arithmetic, composition, and base lookups applied to sampled values.
//! - `ffi` (optional) — C-compatible bindings and factories for hosts; check `src/ffi/mod.rs` and
//!   `src/ffi/base_provider_context.rs` for how hosts create/drive the runtime.
//! - `animation` + `quaternion_utils` — helpers for applying sampled values and rotation math.
//!
//! Runtime flow (high level): host updates `BaseProviderContext` → events register tracks/point
//! definitions → sampling uses `PointDefinition` + `PointData` and may query `BaseProviderContext`
//! during modifier evaluation → final values are applied by the consumer.
//!
//! See `benches/` and unit tests in `src/point_definition/` for minimal examples of usage.
#![feature(trait_alias)]
#![feature(unboxed_closures)]

#[cfg(feature = "ffi")]
pub mod ffi;

pub mod animation;
pub mod base_provider_context;
pub mod easings;
pub mod modifiers;
pub mod point_data;
pub mod point_definition;
pub mod providers;
pub mod values;

pub mod quaternion_utils;

/// Lightweight prelude for external consumers.
///
/// Use `use tracks_rs::prelude::*;` to import the common API without polluting
/// the crate root with many names.
pub mod prelude {
    pub use crate::base_provider_context::BaseProviderContext;
    pub use crate::point_definition::PointDefinitionLike;
    pub use crate::providers::{ValueProvider, AbstractValueProvider, UpdateableValues, deserialize_values};
    pub use crate::modifiers::BaseModifier;
    pub use crate::easings::functions::Functions;
}
