# Tracks Rust Engine
This is a Rust port of the [Heck](https://github.com/Aeroluna/Heck/) Base Providers, Tracks and Point definition functionality without depending on any game runtime.

This README gives short explanations and tiny examples to get started with the main areas of the crate.

---

## Base providers

Base providers are the runtime sources of values that point definitions and modifiers can read.
Use `BaseProviderContext` to store and query base values (score, time, colors, transforms, ...).

Minimal example — set/read a base value:

```rust
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;

fn main() {
	let mut ctx = BaseProviderContext::new();

	// set a float base value (e.g. song time)
	ctx.set_values("baseSongTime", BaseValue::from(12.5f32));

	// read it back
	let val = ctx.get_values("baseSongTime");
	println!("song time = {:?}", val.as_float());
}
```

You can also obtain cached `ValueProvider`s from the context using `get_value_provider` (useful when parsing provider expressions like `baseHeadPosition.x` or smoothed variants `baseSongTime.s0_5`).

---
## Providers

Providers are the runtime building blocks that supply numeric/vector/quaternion data to point definitions and modifiers.

- `Static` — a fixed literal value from JSON (e.g. `[1.0, 2.0, 3.0]`).
- `BaseProvider` — references to `BaseProviderContext` values (strings starting with `base`, e.g. `"baseSongTime"` or `"baseHeadPosition.x"`).
- `PartialProvider` — swizzled views into vector/quaternion providers (e.g. `.x`, `.xy`).
- `SmoothProviders` / `SmoothRotationProviders` — time-smoothing wrappers created from specs like `s1` or `s0_5`.

The crate exposes a helper to convert a JSON slice into a `Vec<ValueProvider>` when the `json` feature is enabled:

```rust
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::providers::deserialize_values;
use serde_json::json;

fn main() {
	let mut ctx = BaseProviderContext::new();

	// Mixed static numbers and base provider references. Strings that start with
	// "base" are turned into BaseProvider entries; numeric sequences become Static entries.
	let raw = json!([0.1, "baseSongTime", 1.5, "baseHeadPosition.x"]);

	// convert to Vec<&Value> as expected by `deserialize_values`
	let arr: Vec<&serde_json::Value> = raw.as_array().unwrap().iter().collect();
	let providers = deserialize_values(&arr, &mut ctx);

	// `providers` now contains a sequence of ValueProvider variants
	println!("parsed providers: {:?}", providers);
}
```

Use `BaseProviderContext::get_value_provider` if you need to parse a single provider expression and cache it for repeated sampling (e.g. `baseSongTime.s0_5`, `baseHeadPosition.xy`).

---

## Tracks

Tracks are containers of named properties and path animations. `TracksHolder` manages multiple `Track` instances and provides stable keys.

Minimal example — create and register a track:

```rust
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::animation::track::Track;

fn main() {
	let mut holder = TracksHolder::new();

	let mut track = Track::default();
	track.name = "my_track".to_string();

	let key = holder.add_track(track);
	let stored = holder.get_track(key).unwrap();
	assert_eq!(stored.name, "my_track");
}
```

Properties on `Track` are strongly typed (e.g. `position` is a Vec3 property). Use the provided `ValueProperty` and `PathProperty` API to set and query values when driving animations.

---

## Point Definitions

Point definitions describe how values change over time. The crate provides several implementations (float, vec3, vec4, quaternion) via the `PointDefinitionLike` trait.

If you enable the `json` feature you can parse Heck-compatible point JSON into point definitions with the provided helpers.

Minimal example — parse a simple float definition (requires `features = ["json"]`):

```rust
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::point_definition::FloatPointDefinition;
use serde_json::json;

fn main() {
	let mut ctx = BaseProviderContext::new();

	// A two-point definition: value 0 at time 0, value 1 at time 1
	let def_json = json!([[0.0, 0.0], [1.0, 1.0]]);

	let def = FloatPointDefinition::parse(def_json, &mut ctx);
	let (value, finished) = def.interpolate(0.5, &ctx);
	println!("interpolated float = {:?}, finished={} ", value, finished);
}
```

---

### Complex JSON parsing example (providers, flags, smoothing)

The parser recognizes three logical groups inside each point entry:

- Values: numeric literals and `base*` strings (these become `ValueProvider`s).
- Modifiers: nested arrays describing modifier composition (the parser calls `deserialize_modifier` recursively).
- Flags: plain strings (not starting with `base`) — used for easing names, smoothing hints like `splineCatmullRom`, and other markers.

Example: a two-point float definition where the second point reads from a base provider and uses easing:

```rust
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::point_definition::FloatPointDefinition;
use serde_json::json;

fn main() {
	let mut ctx = BaseProviderContext::new();

	// Point 0: static value 0 at time 0
	// Point 1: value sampled from baseSongTime at time 1 with easing flag
	let complex = json!([
		[0.0, 0.0],
		["baseSongTime", 1.0, "easeInOutQuad"]
	]);

	let def = FloatPointDefinition::parse(complex, &mut ctx);
	let (v_mid, finished) = def.interpolate(0.5, &ctx);
	println!("value at t=0.5 = {:?}, finished = {}", v_mid, finished);
}
```

Notes:

- Use `"base..."` strings to reference context-provided values. The parser converts those into `BaseProvider` entries so modifiers and interpolation can sample live base data.
- Provider smoothing (e.g. `s0_5`) and swizzles (e.g. `.x`, `.xy`) are handled by `BaseProviderContext::get_value_provider` when the provider string contains dots or smoothing prefixes.
- Modifier arrays (nested JSON arrays inside a point) are parsed recursively and turned into modifier objects via `PointDefinitionLike::deserialize_modifier` and `create_modifier` implementations. See `src/modifiers/` and `src/point_definition/` for the concrete formats supported.


## CoroutineManager

`CoroutineManager` orchestrates time-based events: it schedules and polls coroutines that animate `Track` properties over song time.

Typical host usage:

- Create a `CoroutineManager` and `TracksHolder`.
- When an event occurs, build an `EventData` and call `start_event_coroutine` (the manager converts beatmap duration -> song-time seconds using `bpm`).
- Each frame call `poll_events(song_time, &ctx, &mut holder)` to advance active coroutines.

Minimal example — queue an animate-track event and poll until completion (requires the `json` feature for `FloatPointDefinition::parse`):

```rust
use tracks_rs::animation::coroutine_manager::CoroutineManager;
use tracks_rs::animation::events::{EventData, EventType};
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::animation::track::ValuePropertyHandle;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::easings::functions::Functions;
use tracks_rs::point_definition::FloatPointDefinition;
use serde_json::json;

fn main() {
	let mut ctx = BaseProviderContext::new();
	let mut holder = TracksHolder::new();
	let mut manager = CoroutineManager::default();

	// create and register a track
	let mut track = tracks_rs::animation::track::Track::default();
	track.name = "queued_track".to_string();
	let key = holder.add_track(track);

	// build a simple two-point float definition
	let def_json = json!([[0.0, 0.0], [1.0, 1.0]]);
	let float_def = FloatPointDefinition::parse(def_json, &mut ctx);
	let base_def = tracks_rs::point_definition::BasePointDefinition::from(float_def);

	let event = EventData {
		raw_duration: 1.0, // beats
		easing: Functions::EaseLinear,
		repeat: 0,
		start_song_time: 0.0,
		property: EventType::AnimateTrack(ValuePropertyHandle::new("position")),
		track_key: key,
		point_data: Some(base_def),
	};

	// queue it and run a simple poll loop
	let bpm = 120.0f32;
	let mut song_time = 0.0f32;
	manager.start_event_coroutine(bpm, song_time, &ctx, &mut holder, event);

	// advance time in a simple loop (host would use frame delta)
	for _ in 0..60 {
		song_time += 1.0 / 60.0;
		manager.poll_events(song_time, &ctx, &mut holder);
	}
}
```

Quick poll-only example (when coroutines are started elsewhere):

```rust
use tracks_rs::animation::coroutine_manager::CoroutineManager;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::animation::tracks_holder::TracksHolder;

fn main() {
	let ctx = BaseProviderContext::new();
	let mut holder = TracksHolder::new();
	let mut manager = CoroutineManager::default();

	// Each frame, advance song time and poll events
	let song_time = 0.0f32;
	manager.poll_events(song_time, &ctx, &mut holder);
}
```

See `src/animation/coroutine_manager.rs` for the implementation and unit tests.

---