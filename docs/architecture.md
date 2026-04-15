# Tracks & Points — architecture (Heck spec aligned)

This document describes how `tracks-rs` maps to the Heck "Tracks and Points" specification: the roles of tracks, point definitions, point data, modifiers, and the `BaseProviderContext` (bases). Use this as a reference when adding or changing parsing/sampling logic.

## Core concepts

- `Track`
  - A named collection of objects (notes, walls, prefabs, etc.). Objects opt into tracks via a `"track": "Name"` entry in their `customData` and may belong to multiple tracks.
  - Events (e.g., `AnimateTrack`, `AssignPathAnimation`) target tracks and affect every object on the track.

- `Point Definition`
  - Describes how a property changes over the course of an animation. It is a list of points; each point is property-specific values followed by a time value (0..1), with optional easing and optional spline.
  - Points must be ordered by ascending time. Each point must contain exactly the number of components the property expects (e.g., `color` needs RGBA + time = 5 numbers).
  - Single-point shorthand: a single point may be specified without an outer array and is treated as time=0.

- `Point Data`
  - The concrete storage for a `PointDefinition` (the time/value tuples sampled at runtime).

- `Modifier`
  - Operations appended to points that perform arithmetic, combine inputs, or reference external base values. Supported component-wise ops include: `opNone`, `opAdd`, `opSub`, `opMul`, `opDiv`.
  - Modifiers can be chained; evaluation is left-to-right (explicit ordering via arrays).

- `BaseProviderContext` ("Bases")
  - A runtime context of named, typed values provided by the engine/host (score, colors, transforms, song time, etc.). These are NOT point data — they are external inputs that point definitions and modifiers may reference using the `base` syntax (e.g., `"baseHeadPosition"`).
  - Bases are evaluated lazily when read and support:
    - Swizzling: `.xyzw` selectors (e.g., `baseHeadPosition.z`, `baseHeadPosition.zyx`).
    - Smoothing: `.sN` syntax to smooth values across frames (e.g., `.s10` or `.s0_4`).
    - Component mixing and arithmetic via modifiers (e.g., `["baseNote0Color", [0.4, 0.4, 0.4, 1, "opMul"]]`).

## Tracks & events (how they behave)

- `AnimateTrack` — animate properties of all objects on the specified track(s) over an event duration. The event's `property` is a `PointDefinition` followed over `duration` using optional `easing` and `repeat`.
- `AssignPathAnimation` — assigns a path animation (per-object lifetime sampling) to track objects; `duration` and `easing` transition between path animations.
- Events targeting non-existent tracks will error; tracks must contain at least one object.
- Multiple `AnimateTrack` events can animate different properties on the same object concurrently. Animating the same property with multiple overlapping events is treated as overwriting unless using separate tracks that combine via modifiers.

## Point definition rules (summary)

- Pattern: `[data..., time, optional easing, optional spline]`.
- `time` is a float in [0,1]. Points must be ordered by time.
- `easing` may be an easing name (e.g., `easeLinear`, `easeStep`, or easings from easings.net).
- `spline` (e.g., `splineCatmullRom`) affects movement between points (positions/rotations only currently).
- Out-of-range sampling clamps to first/last points.

## Modifiers, bases, swizzling, smoothing

- Modifiers evaluate component-wise and can be nested. Example operations: add, multiply, divide.
- Bases let animations reference live engine state (song time, player transforms, colors, score values).
- Swizzling (`.xyzw`) extracts/reorders components; it can change dimensionality (e.g., pick `.x` to produce a single-value point from a vec3 base).
- Smoothing (`.sN`) provides temporal smoothing. Use `_` for decimals (e.g., `.s0_4`).

## Runtime sampling flow

1. Host/engine updates `BaseProviderContext` with current named base values as needed.
2. Tracks exist when one or more objects include the track name in their `customData`.
3. An event (`AnimateTrack`/`AssignPathAnimation`) targeting a track provides `PointDefinition`s for properties.
4. When sampling at a given time (global or per-object lifetime):
   - `PointDefinition` chooses the points to interpolate between and applies easing/spline rules.
   - Any `Modifier` or value provider invoked during sampling may read from `BaseProviderContext` (with swizzle/smooth applied).
   - Modifier chains combine and transform sampled values to produce the final value.
5. The final value is applied to the object by the consumer (engine/FFI).

## ASCII diagram

```
+----------------------+        +--------------------+
| BaseProviderContext  |------->| PointDefinition    |
| (base values)        |        | (sampling rules)   |
+----------------------+        +--------------------+
  |                             |
  |                             v
  |                       +-----------------+
  |                       |   PointData     |
  |                       | (time, value...)|
  |                       +-----------------+
  v                             |
+----------------------+              v
|     Modifiers        |<------------- 
| (chain transforms)   |
+----------------------+
  |
  v
+----------------------+
|       Track          |
|  (applies value)     |
+----------------------+
  |
  v
+----------------------+
|      Consumer        |
| (engine / FFI)      |
+----------------------+

Examples: float, vec3, quaternion
```

## Implementation pointers for contributors

- Validate point shapes early (throw/return an error on incorrect component counts).
- Keep `PointDefinition` parsing and sampling pure; side effects should live in the consumer/host code.
- Implement modifiers as small composable operations that accept a `BaseProviderContext` reference when they need bases.
- When exposing bases to FFI consumers, prefer explicit getters and stable, minimal layouts.

---

Code references:
- `BaseProviderContext`: [src/base_provider_context.rs](src/base_provider_context.rs)
- Point definitions/implementations: [src/point_definition](src/point_definition)
- Point data: [src/point_data](src/point_data)

If you want, I can now add a table linking each `base*` name to the exact field and line in `src/base_provider_context.rs`.
