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

Minimal example — queue an animate-track event and poll until completion (requires the `json` feature for JSON parsing helpers):

```rust
use glam::Vec3;
use serde_json::json;
use tracks_rs::animation::coroutine_manager::CoroutineManager;
use tracks_rs::animation::events::{EventData, EventType};
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::animation::track::{ValuePropertyHandle, PathPropertyHandle, PropertyNames};
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::easings::functions::Functions;
use tracks_rs::point_definition::vector3_point_definition::Vector3PointDefinition;

fn main() {
	let mut ctx = BaseProviderContext::new();
	let mut holder = TracksHolder::new();
	let mut manager = CoroutineManager::default();

	// create and register a track
	let mut track = tracks_rs::animation::track::Track::default();
	track.name = "queued_track".to_string();
	let key = holder.add_track(track);

	// build a simple two-point Vec3 definition: [x, y, z, time]
	let def_json = json!([[0.0, 0.0, 0.0, 0.0], [1.0, 1.0, 1.0, 1.0]]);
	let vec3_def = Vector3PointDefinition::parse(def_json, &mut ctx);
	let base_def = tracks_rs::point_definition::BasePointDefinition::from(vec3_def);

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

	// --- Read ValueProperty (final stored value) ---
	let stored = holder.get_track(key).expect("track present");

	// Use `PropertyNames` for canonical properties
	let prop = &stored.properties.position;
	if let Some(value) = prop.get_value() {
		println!("position property value = {:?}", value);
	} else {
		println!("position property has no value");
	}

	// --- Read PathProperty (interpolated path data) ---
	let path_prop = &stored.path_properties.position;

	if let Some(v) = path_prop.interpolate(0.5, &ctx) {
		println!("interpolated path value = {:?}", v);
	} else {
		println!("no path data available");
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

## TimelineCoroutineManager

`CoroutineManager` only moves forward: when a later event overwrites a property, the earlier state is gone, so you cannot seek backwards. `TimelineCoroutineManager` is the seekable alternative. You register **all** events up front, then ask for the state at **any** song time, in any order.

How it works:

- Events are grouped into one timeline per `(track, property)` and sorted by start time.
- Starting an event overrides the previous one on the same property, so the state at time `t` only depends on the last event with `start <= t`. A path property also uses the event before it, because that is the path it blends from.
- Nothing is replayed and no state is kept between calls, so seeking costs the same as playing.
- Evaluation matches `CoroutineManager`: `AnimateTrack` repeats `repeat + 1` times and then holds the final value, `AssignPathAnimation` blends from the previous path over its duration, and an event with no point data clears the property.
- If two events on the same property start at the same time, the one added later wins.
- Events can be added and removed at any time, in any order (see below).

Typical host usage:

- Build the manager once from the map's events: `TimelineCoroutineManager::from_events(bpm, events)` (or `add_event` one at a time).
- On every frame or seek, call `apply(song_time, &ctx, &mut holder)`. It writes the state at that time into every property that has events. Properties with no active event at that time are reset to `None`.
- To read a single property without touching the tracks, use `value_at` or `path_at`.

```rust
use tracks_rs::animation::events::{EventData, EventType};
use tracks_rs::animation::timeline_coroutine_manager::TimelineCoroutineManager;
use tracks_rs::animation::track::{Track, ValuePropertyHandle};
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::easings::functions::Functions;
use tracks_rs::time_types::{BpmTime, SongTime};

fn main() {
	let ctx = BaseProviderContext::new();
	let mut holder = TracksHolder::new();

	let mut track = Track::default();
	track.name = "my_track".to_string();
	let key = holder.add_track(track);

	let dissolve = ValuePropertyHandle::new("dissolve");
	let event = |start: f64, point_data| EventData {
		raw_duration: BpmTime::new(1.0), // beats
		easing: Functions::EaseLinear,
		repeat: 0,
		start_song_time: SongTime::new(start), // seconds
		property: EventType::AnimateTrack(dissolve.clone()),
		track_key: key,
		point_data: Some(point_data),
	};

	// `ramp_0_to_10` and `ramp_100_to_200` are `BasePointDefinition`s (see "Point Definitions").
	// The second event starts at 0.5s and overrides the first one.
	let bpm = 60.0;
	let timeline = TimelineCoroutineManager::from_events(
		bpm,
		[event(0.0, ramp_0_to_10), event(0.5, ramp_100_to_200)],
	);

	// seek anywhere, in any order
	timeline.apply(SongTime::new(1.0), &ctx, &mut holder); // second event is active
	timeline.apply(SongTime::new(0.25), &ctx, &mut holder); // back to the first event

	// read a value without writing to the tracks
	let value = timeline.value_at(SongTime::new(0.25), key, &dissolve, &ctx);
	println!("dissolve at 0.25s = {:?}", value);
}
```

Adding and removing events:

- `add_event` returns an `EventId`. Pass it to `remove_event(id)` to take that event out again (it returns `false` if the event is already gone). The event that was active before it becomes active again.
- `remove_events_between(track_key, start, end)` removes every event of a track, on any property, that starts in `start..=end` and returns how many it removed.
- `clear()` removes everything.
- Nothing is written to the tracks until the next `apply`. If every event of a property was removed one by one, that `apply` resets the property to `None`. After `clear()` the manager no longer knows those properties, so `apply` leaves them as they are.

Notes:

- `apply` only writes properties that have events. Anything else on the tracks is left alone.
- The BPM is fixed when events are added, so a map with BPM changes needs the durations converted beforehand.
- `path_at` returns a `PathSnapshot` (previous path, current path and eased blend time). Sample it at an object's lifetime with `PathSnapshot::interpolate`, the same as `PathProperty::interpolate`.

### From C / C++ (`ffi` feature)

The same API is exported through the generated `shared/bindings.h`:

| Function | Purpose |
| --- | --- |
| `create_timeline_coroutine_manager` / `destroy_timeline_coroutine_manager` | Create and free the manager. |
| `timeline_add_event(manager, bpm, event_data)` | Add an event (from `event_data_to_rust`). Returns its `CEventId` (`{ track_key, id }`; the id is `UINT64_MAX` on a null pointer). The event is cloned, so you keep ownership of `event_data`. |
| `timeline_remove_event(manager, id)` | Remove an event by its `CEventId`. Returns false if it is already gone. |
| `timeline_remove_events_between(manager, track_key, start, end)` | Remove every event of a track, on any property, starting in `start..=end`. Returns the count. |
| `timeline_clear(manager)` | Remove every event. |
| `timeline_apply(manager, song_time, context, tracks_holder)` | Write the state at `song_time` into the tracks. Any time, any order. |
| `timeline_value_at(manager, song_time, track_key, property, context)` | Value of an animated property, as a nullable value. |
| `timeline_path_interpolate(manager, song_time, track_key, property, path_time, context)` | Sample a path property at an object's lifetime `path_time`. |
| `timeline_path_blend(manager, song_time, track_key, property)` | Blend state of a path property (`exists`, `has_prev_point`, `has_point`, `interpolate_time`). |

`property` is a `CEventType`, the same struct used when building events. Its type must match the query (`AnimateTrack` for `timeline_value_at`, `AssignPathAnimation` for the path queries), otherwise the empty result is returned.

```cpp
auto* timeline = create_timeline_coroutine_manager();

// add every event of the map up front
for (const CEventData& e : events) {
    EventData* event = event_data_to_rust(&e);
    timeline_add_event(timeline, bpm, event);
    event_data_dispose(event);
}

// each frame, or when the user seeks
timeline_apply(timeline, songTime, context, tracksHolder);

destroy_timeline_coroutine_manager(timeline);
```

See `src/animation/timeline_coroutine_manager.rs` for the implementation and unit tests. One of them checks that `apply` gives the same results as stepping a `CoroutineManager`.

---
