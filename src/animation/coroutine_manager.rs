use std::ops::ControlFlow;

use log::debug;

use crate::{
    animation::{
        event_timing::EventTiming,
        track::Track,
        tracks_holder::{TrackKey, TracksHolder},
    },
    base_provider_context::BaseProviderContext,
    point_definition::{
        PointDefinitionLike,
        base_point_definition::{self},
    },
    time_types::SongTime,
};

use super::{
    events::{EventData, EventType},
    property::{PathProperty, ValueProperty},
};

/// Drives time-based track events (`AnimateTrack` and `AssignPathAnimation`).
///
/// # Coroutines
/// A coroutine is a `CoroutineTask`: the state of one event that is still running.
/// It holds the target track and property, the start time, the duration (beats converted to
/// song time using the BPM), the easing, the repeat count and the point definition. Each poll
/// runs one step, which returns [`ControlFlow::Continue`] (keep running) or
/// [`ControlFlow::Break`] (finished; remove it). This mirrors the Unity coroutines
/// used by the original C# implementation, but here the caller drives them with song time.
///
/// # Starting and overriding events
/// [`Self::start_event_coroutine`] first cancels any running coroutine with the same track
/// *and* event type. Each `(track, property)` pair has at most one active coroutine, so a
/// new event takes over a property that is still animating. Events on other tracks, or on
/// other properties of the same track, are unaffected.
///
/// The new event is not queued if:
/// - it has no point data, so the target property is cleared (`set_null`),
/// - it has zero duration or has already fully elapsed (repeats included), so the final value is applied,
/// - it is a single static point with no base provider, so that value is applied once.
///
/// Otherwise its first frame runs straight away, through the same step as every later poll,
/// and it is queued unless that step already finished it.
///
/// # Animating properties (`AnimateTrack`)
/// Each poll computes `elapsed / duration`, clamps it to `[0, 1]`, applies the easing,
/// interpolates the point definition, and writes the result to the track's `ValueProperty`.
/// When an iteration ends and `repeat > 0`, the start time moves forward by one duration and
/// the animation runs again. A large time step can finish several iterations in one poll.
/// Point definitions with base providers are never finished early, because their output
/// can still change after the last point is reached.
///
/// # Animating path properties (`AssignPathAnimation`)
/// When the event starts, the track's `PathProperty` keeps its current path as the previous
/// path and takes the event's point definition as the new path. Objects sample the path at
/// their own lifetime. The coroutine only advances the eased `interpolate_time`, which
/// controls the blend from the previous path to the new path. When the duration ends, the
/// path property is finished: the previous path is dropped and only the new path is used.
///
/// Call [`Self::poll_events`] once per frame with the current song time.
#[derive(Clone)]
pub struct CoroutineManager {
    coroutines: Vec<CoroutineTask>,
}

/// Represents a single coroutine task for an event.
#[derive(Clone)]
struct CoroutineTask {
    event_type: EventType,
    repeat: u32,
    /// Whether the point definition has a base provider, which affects whether we can skip interpolation when finished
    /// this is here to avoid repeatedly calling has_base_provider on the point definition during interpolation, which can be expensive for complex definitions with many modifiers
    has_base_provider: bool,
    timing: EventTiming,
    track_key: TrackKey,
    /// The `AnimateTrack` points. `None` for path events, whose points live in the path property.
    point_definition: Option<base_point_definition::BasePointDefinition>,
}

impl Default for CoroutineManager {
    fn default() -> Self {
        CoroutineManager {
            coroutines: Vec::with_capacity(1000),
        }
    }
}

impl EventType {
    /// Clears the track property or path property targeted by this event type.
    pub(crate) fn set_null(&self, track: &mut Track) {
        match self {
            EventType::AnimateTrack(property_handle) => {
                let property = track
                    .properties
                    .get_by_handle_mut(property_handle)
                    .expect("Property not found");
                property.set_value(None);
            }
            EventType::AssignPathAnimation(path_property_handle) => {
                let path_property = track
                    .path_properties
                    .get_by_handle_mut(path_property_handle)
                    .expect("Path property not found");
                path_property.init(None)
            }
        }
    }
}

impl CoroutineManager {
    /// Starts a new event coroutine, cancelling any existing coroutines for the same event type on the same track.
    pub fn start_event_coroutine(
        &mut self,
        bpm: f32,
        song_time: SongTime,
        provider_context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
        event_group_data: EventData,
    ) {
        let duration_song_time = event_group_data.raw_duration.to_song_time(bpm as f64);

        // cancel any existing coroutines for the same event type
        // that are on the same track
        // there's only ever one per track per event type
        // so we find the first and remove it
        if let Some(pos) = self.coroutines.iter().position(|x| {
            x.track_key == event_group_data.track_key && x.event_type == event_group_data.property
        }) {
            // order does not matter for coroutine execution semantics, so use swap_remove
            // to avoid O(n) shifts on cancellation-heavy paths
            self.coroutines.swap_remove(pos);
        }

        let Some(mut task) = Self::start_task(
            song_time,
            duration_song_time,
            event_group_data,
            provider_context,
            tracks_holder,
        ) else {
            debug!("CoroutineTask has 0 duration or no points, skipping");
            return;
        };

        // the first frame runs exactly like every later poll
        if Self::step(&mut task, song_time, provider_context, tracks_holder).is_continue() {
            self.coroutines.push(task);
        }
    }

    /// Applies the parts of an event that happen immediately, and returns the task to run if it still needs to.
    ///
    /// Returns `None` when there is nothing left to animate:
    /// - no point data: the property is cleared,
    /// - zero duration or already fully elapsed (repeats included): the final value is applied, or the path is finished,
    /// - a single static point: its value is applied once.
    fn start_task(
        current_song_time: SongTime,
        duration_song_time: SongTime,
        data: EventData,
        provider_context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) -> Option<CoroutineTask> {
        let timing = EventTiming {
            start_song_time: data.start_song_time,
            duration_song_time,
            easing: data.easing,
        };
        let already_elapsed = duration_song_time == SongTime::ZERO
            || data.start_song_time + duration_song_time * (data.repeat as f64 + 1.0)
                < current_song_time;

        let track = tracks_holder
            .get_track_mut(data.track_key)
            .expect("Track not found for CoroutineTask");
        let Some(point_data) = data.point_data else {
            data.property.set_null(track);
            return None;
        };

        // only `AnimateTrack` steps read this, and it walks every point
        let mut has_base_provider = false;
        let point_definition = match &data.property {
            EventType::AnimateTrack(property_handle) => {
                has_base_provider = point_data.has_base_provider();
                if already_elapsed || (point_data.get_count() <= 1 && !has_base_provider) {
                    let property = track
                        .properties
                        .get_by_handle_mut(property_handle)
                        .expect("Property not found");
                    set_property_value(&point_data, property, 1.0, provider_context);
                    return None;
                }
                Some(point_data)
            }
            EventType::AssignPathAnimation(path_property_handle) => {
                let path_property = track
                    .path_properties
                    .get_by_handle_mut(path_property_handle)
                    .expect("Path property not found");
                path_property.init(Some(point_data));
                if already_elapsed {
                    path_property.finish();
                    return None;
                }
                None
            }
        };

        Some(CoroutineTask {
            event_type: data.property,
            repeat: data.repeat,
            has_base_provider,
            timing,
            track_key: data.track_key,
            point_definition,
        })
    }

    /// Advances all active coroutines to `song_time`, removing any that have finished.
    pub fn poll_events(
        &mut self,
        song_time: SongTime,
        context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) {
        // Poll in-place and remove completed coroutines with swap_remove to avoid
        // compaction costs from retain_mut when many entries complete each frame.
        let mut i = 0;
        while i < self.coroutines.len() {
            if Self::step(&mut self.coroutines[i], song_time, context, tracks_holder).is_continue()
            {
                i += 1;
            } else {
                self.coroutines.swap_remove(i);
            }
        }
    }

    /// Runs one frame of a coroutine: updates its property, and moves on to the next repeat when an iteration ends.
    /// Returns `Break` when the task is done and can be dropped.
    fn step(
        task: &mut CoroutineTask,
        song_time: SongTime,
        context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) -> ControlFlow<()> {
        let track = tracks_holder
            .get_track_mut(task.track_key)
            .expect("Track not found for CoroutineTask");

        match &task.event_type {
            EventType::AnimateTrack(value_property_handle) => {
                let Some(point_def) = &task.point_definition else {
                    debug!("No point definition for AnimateTrack event, skipping");
                    return ControlFlow::Break(());
                };
                let value_property = track
                    .properties
                    .get_by_handle_mut(value_property_handle)
                    .expect("Property not found");

                let mut run = |timing: &EventTiming| {
                    animate_track(
                        point_def,
                        value_property,
                        timing,
                        song_time,
                        task.has_base_provider,
                        context,
                    )
                };

                let mut flow = run(&task.timing);

                // each repeat restarts the iteration one duration later
                while flow.is_break() && task.repeat > 0 {
                    task.repeat -= 1;
                    task.timing.start_song_time += task.timing.duration_song_time;
                    flow = run(&task.timing);
                }

                flow
            }
            EventType::AssignPathAnimation(path_property_handle) => {
                let path_property = track
                    .path_properties
                    .get_by_handle_mut(path_property_handle)
                    .expect("Path property not found");

                assign_path_animation(path_property, &task.timing, song_time)
            }
        }
    }
}

/// Interpolates `points` at the eased progress of the event and writes the result to `property`.
/// Returns `Break` once the duration has elapsed (or early if the last point is reached and not `non_lazy`).
fn animate_track(
    points: &base_point_definition::BasePointDefinition,
    property: &mut ValueProperty,
    timing: &EventTiming,
    current_song_time: SongTime,
    non_lazy: bool,
    context: &BaseProviderContext,
) -> ControlFlow<()> {
    let on_last = set_property_value(
        points,
        property,
        timing.progress(current_song_time),
        context,
    );
    let reached_end = !non_lazy && on_last;

    if timing.has_finished(current_song_time) || reached_end {
        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

/// Updates the path property's eased interpolation time for the current song time.
/// Returns `Break` and finishes the path property once the duration has elapsed.
fn assign_path_animation(
    interpolation: &mut PathProperty,
    timing: &EventTiming,
    song_time: SongTime,
) -> ControlFlow<()> {
    interpolation.interpolate_time = timing.progress(song_time);

    if !timing.has_finished(song_time) {
        return ControlFlow::Continue(());
    }

    interpolation.finish();
    ControlFlow::Break(())
}

/// Sets the value of a property based on the points defined in the BasePointDefinition.
/// Returns true if the property was set to the last point's value. aka finished
fn set_property_value(
    points: &base_point_definition::BasePointDefinition,
    property: &mut ValueProperty,
    time: f32,
    context: &BaseProviderContext,
) -> bool {
    let (value, finished) = points.interpolate(time, context);

    if Some(value) == property.get_value() {
        return finished;
    }

    property.set_value(Some(value));
    finished
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::events::{EventData, EventType};
    use crate::animation::track::PathPropertyHandle;
    use crate::animation::track::Track;
    use crate::animation::track::ValuePropertyHandle;
    use crate::animation::tracks_holder::TracksHolder;
    use crate::base_provider_context::BaseProviderContext;
    use crate::base_value::WrapBaseValueType;
    use crate::easings::functions::Functions;
    use crate::modifiers::ModifierValues;
    use crate::point_data::basic_point_data::BasicPointData;
    use crate::point_definition::base_point_definition::BasePointDefinition;
    use crate::point_definition::basic_point_definition::BasicPointDefinition;
    use crate::point_definition::vector3_point_definition;
    use crate::time_types::BpmTime;
    use glam::Vec3;
    use glam::Vec4;

    type Vector3PointData = BasicPointData<Vec3>;

    type Vector3PointDefinition = vector3_point_definition::Vector3PointDefinition;
    type Vector4PointDefinition = BasicPointDefinition<Vec4>;

    #[test]
    fn tracks_holder_add_get() {
        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "track_a".to_string();
        let key = holder.add_track(t);

        let got = holder.get_track(key).expect("track should exist");
        assert_eq!(got.name, "track_a");

        let by_name = holder
            .get_track_by_name("track_a")
            .expect("by_name should work");
        assert_eq!(by_name.name, "track_a");
    }

    #[test]
    #[should_panic]
    fn tracks_holder_duplicate_panics() {
        let mut holder = TracksHolder::new();
        let mut t1 = Track::default();
        t1.name = "dup".to_string();
        let t2 = Track::default();
        // two distinct values with same name
        let mut t2 = t2;
        t2.name = "dup".to_string();
        holder.add_track(t1);
        // adding another with same name should panic
        holder.add_track(t2);
    }

    #[test]
    fn coroutine_start_and_poll_sets_property() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "c_track".to_string();
        let key = holder.add_track(t);

        // construct a simple float point definition with two points (0 -> 10 over time 0..1)
        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd)),
        };

        // bpm 60 => duration = 1.0 for raw_duration 1.0
        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // poll at halfway through duration (0.5) - should set dissolve ~5.0
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);

        let track = holder.get_track(key).unwrap();
        let val = track.properties.dissolve.get_value().expect("value set");
        let f = val.as_float().unwrap();
        assert!((f - 5.0).abs() < 1e-3, "expected ~5.0 got {}", f);
    }

    #[test]
    fn cancel_previous_coroutine_on_same_track_only() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();

        // track A
        let mut ta = Track::default();
        ta.name = "track_a".to_string();
        let key_a = holder.add_track(ta);

        // track B
        let mut tb = Track::default();
        tb.name = "track_b".to_string();
        let key_b = holder.add_track(tb);

        // initial coroutine on track A (should be cancelled later)
        let pd_a1 = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev_a1 = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key_a,
            point_data: Some(BasePointDefinition::Float(pd_a1)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev_a1);

        // start a coroutine on track B same property - should NOT be cancelled by later A
        let pd_b = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(5.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(15.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev_b = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key_b,
            point_data: Some(BasePointDefinition::Float(pd_b)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev_b);

        // start a different-property coroutine on track A - should NOT cancel dissolve on A
        // use color (vec4)
        let pd_color = Vector4PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(0.0, 0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec4::new(4.0, 4.0, 4.0, 4.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev_a_color = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("color")),
            track_key: key_a,
            point_data: Some(BasePointDefinition::Vector4(pd_color)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev_a_color);

        // Now start a NEW coroutine on track A for same property (dissolve) which should cancel the first one
        let pd_a2 = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(20.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev_a2 = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key_a,
            point_data: Some(BasePointDefinition::Float(pd_a2)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev_a2);

        // poll halfway through (0.5)
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);

        // track A dissolve should reflect pd_a2 (0->20 => 10 at t=0.5)
        let ta = holder.get_track(key_a).unwrap();
        let da = ta
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (da - 10.0).abs() < 1e-3,
            "track A dissolve expected ~10 got {}",
            da
        );

        // track B dissolve should reflect its own pd_b (5->15 => 10 at t=0.5)
        let tb = holder.get_track(key_b).unwrap();
        let db = tb
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (db - 10.0).abs() < 1e-3,
            "track B dissolve expected ~10 got {}",
            db
        );

        // track A color should reflect pd_color (0->4 => 2.0 per component at t=0.5)
        let ta_color = ta.properties.color.get_value().unwrap().as_vec4().unwrap();
        assert!(
            (ta_color.x - 2.0).abs() < 1e-3,
            "track A color.x expected ~2 got {}",
            ta_color.x
        );
    }

    #[test]
    fn zero_duration_event_sets_final_value() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "z_track".to_string();
        let key = holder.add_track(t);

        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        // raw_duration 0 -> duration calculation leads to 0 and should immediately set final value
        let ev = EventData {
            raw_duration: BpmTime::new(0.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // 0-duration events should not leave coroutines enqueued
        assert!(
            cm.coroutines.is_empty(),
            "0-duration coroutine should not be retained"
        );

        let track = holder.get_track(key).unwrap();
        let v = track
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        // should be final value 10.0
        assert!(
            (v - 10.0).abs() < 1e-6,
            "expected final value 10.0, got {}",
            v
        );
    }

    #[test]
    fn zero_duration_assign_path_interpolate_not_none() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "path_track".to_string();
        let key = holder.add_track(t);

        // Vec3 point definition (0 -> 3 over time 0..1)
        let pd = Vector3PointDefinition::new(vec![
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(0.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AssignPathAnimation(PathPropertyHandle::new("definitePosition")),
            track_key: key,
            point_data: Some(BasePointDefinition::Vector3(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // 0-duration events should not leave coroutines enqueued
        assert!(
            cm.coroutines.is_empty(),
            "0-duration coroutine should not be retained"
        );

        {
            let track = holder.get_track(key).unwrap();
            // Interpolating the definitePosition path property should NOT return None
            let res = track
                .path_properties
                .definite_position
                .interpolate(0.0, &ctx);
            assert!(
                res.is_some(),
                "interpolate should return Some for zero-duration assign path"
            );

            // value should equal the final point (0.0, 0.0, 0.0)
            let v = res.unwrap().as_vec3().unwrap();
            assert_eq!(v, Vec3::new(0.0, 0.0, 0.0));
        }

        cm.poll_events(SongTime::new(20.0), &ctx, &mut holder);

        {
            let track = holder.get_track(key).unwrap();
            let res_end = track
                .path_properties
                .definite_position
                .interpolate(1.0, &ctx);
            assert!(
                res_end.is_some(),
                "interpolate should return Some for zero-duration assign path at end"
            );
            let v_end = res_end.unwrap().as_vec3().unwrap();
            assert_eq!(v_end, Vec3::new(3.0, 3.0, 3.0));
        }
    }

    #[test]
    fn zero_duration_assign_path_finishes_immediately() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "path_track_immediate".to_string();
        let key = holder.add_track(t);

        // Vec3 point definition (0 -> 3 over time 0..1)
        let pd = Vector3PointDefinition::new(vec![
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(0.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AssignPathAnimation(PathPropertyHandle::new("definitePosition")),
            track_key: key,
            point_data: Some(BasePointDefinition::Vector3(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // zero-duration events should not leave coroutines enqueued
        assert!(
            cm.coroutines.is_empty(),
            "0-duration coroutine should not be retained"
        );

        // The path property should already be finished and return the final value immediately
        let track = holder.get_track(key).unwrap();
        let res = track
            .path_properties
            .definite_position
            .interpolate(1.0, &ctx);
        assert!(
            res.is_some(),
            "interpolate should return Some after zero-duration assign"
        );
        let v = res.unwrap().as_vec3().unwrap();
        assert_eq!(v, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn path_animation_progress_and_persistence() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "path_track_progress".to_string();
        let key = holder.add_track(t);

        // Vec3 point definition (0 -> 3 over time 0..1)
        let pd = Vector3PointDefinition::new(vec![
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AssignPathAnimation(PathPropertyHandle::new("definitePosition")),
            track_key: key,
            point_data: Some(BasePointDefinition::Vector3(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // halfway through duration (0.5) -> should be approximately halfway along the path
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let interp_time = track.path_properties.definite_position.interpolate_time;
        let res_mid = track
            .path_properties
            .definite_position
            .interpolate(interp_time, &ctx);
        assert!(
            res_mid.is_some(),
            "interpolate should return Some during animation"
        );
        let v_mid = res_mid.unwrap().as_vec3().unwrap();
        assert!(
            (v_mid.x - 1.5).abs() < 1e-3,
            "expected ~1.5 got {}",
            v_mid.x
        );

        // after duration (slightly after 1.0) the animation should finish and value be final
        cm.poll_events(SongTime::new(1.1), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res_final = track
            .path_properties
            .definite_position
            .interpolate(1.0, &ctx);
        assert!(
            res_final.is_some(),
            "interpolate should return Some after finish"
        );
        let v_final = res_final.unwrap().as_vec3().unwrap();
        assert_eq!(v_final, Vec3::new(3.0, 3.0, 3.0));

        // much later, value should persist
        cm.poll_events(SongTime::new(5.0), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res_later = track
            .path_properties
            .definite_position
            .interpolate(1.0, &ctx);
        assert!(res_later.is_some(), "interpolate should return Some later");
        let v_later = res_later.unwrap().as_vec3().unwrap();
        assert_eq!(v_later, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn animate_repeat_runs_multiple_times() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "animate_repeat".to_string();
        let key = holder.add_track(t);

        // float point definition (0 -> 10 over 0..1)
        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        // repeat = 2 -> should run 3 times total
        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 2,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // midpoint of first iteration
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);
        let v1 = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v1 - 5.0).abs() < 1e-3,
            "expected ~5.0 during first iteration, got {}",
            v1
        );

        // ensure first iteration finishes and schedules restart
        cm.poll_events(SongTime::new(1.01), &ctx, &mut holder);
        // midpoint of second iteration
        cm.poll_events(SongTime::new(1.5), &ctx, &mut holder);
        let v2 = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v2 - 5.0).abs() < 1e-3,
            "expected ~5.0 during second iteration, got {}",
            v2
        );

        // midpoint of third iteration
        cm.poll_events(SongTime::new(2.5), &ctx, &mut holder);
        let v3 = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v3 - 5.0).abs() < 1e-3,
            "expected ~5.0 during third iteration, got {}",
            v3
        );

        // after all repeats complete
        cm.poll_events(SongTime::new(3.1), &ctx, &mut holder);
        let v_final = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v_final - 10.0).abs() < 1e-6,
            "expected final 10.0 got {}",
            v_final
        );
        assert!(cm.coroutines.is_empty(), "expected no coroutines left");
    }

    #[test]
    fn path_repeat_runs_multiple_times() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "path_repeat".to_string();
        let key = holder.add_track(t);

        // Vec3 point definition (0 -> 3 over 0..1)
        let pd = Vector3PointDefinition::new(vec![
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(0.0, 0.0, 0.0)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            Vector3PointData::new(
                ModifierValues::Static(Vec3::new(3.0, 3.0, 3.0)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        // repeat = 2 -> should run 3 times total
        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 2,
            start_song_time: SongTime::new(0.0),
            property: EventType::AssignPathAnimation(PathPropertyHandle::new("definitePosition")),
            track_key: key,
            point_data: Some(BasePointDefinition::Vector3(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // midpoint first
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res1 = track
            .path_properties
            .definite_position
            .interpolate(0.5, &ctx)
            .unwrap()
            .as_vec3()
            .unwrap();
        assert!((res1.x - 1.5).abs() < 1e-3, "expected ~1.5 got {}", res1.x);

        // midpoint second
        cm.poll_events(SongTime::new(1.5), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res2 = track
            .path_properties
            .definite_position
            .interpolate(0.5, &ctx)
            .unwrap()
            .as_vec3()
            .unwrap();
        assert!((res2.x - 1.5).abs() < 1e-3, "expected ~1.5 got {}", res2.x);

        // midpoint third
        cm.poll_events(SongTime::new(2.5), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res3 = track
            .path_properties
            .definite_position
            .interpolate(0.5, &ctx)
            .unwrap()
            .as_vec3()
            .unwrap();
        assert!((res3.x - 1.5).abs() < 1e-3, "expected ~1.5 got {}", res3.x);

        // after completion final value
        cm.poll_events(SongTime::new(3.1), &ctx, &mut holder);
        let track = holder.get_track(key).unwrap();
        let res_final = track
            .path_properties
            .definite_position
            .interpolate(1.0, &ctx)
            .unwrap()
            .as_vec3()
            .unwrap();
        assert_eq!(res_final, Vec3::new(3.0, 3.0, 3.0));
        assert!(cm.coroutines.is_empty(), "expected no coroutines left");
    }

    #[test]
    fn missing_point_data_sets_property_to_none() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "n_track".to_string();
        let key = holder.add_track(t);

        // Event with no point_data should call set_null and leave property None
        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: None,
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        let track = holder.get_track(key).unwrap();
        assert!(
            track.properties.dissolve.get_value().is_none(),
            "dissolve should be None"
        );
    }

    #[test]
    fn late_start_mid_repeat_runs_the_current_iteration() {
        let ctx = BaseProviderContext::new();
        let mut holder = TracksHolder::new();
        let mut track = Track::default();
        track.name = "late".to_string();
        let key = holder.add_track(track);

        // runs 0..1, 1..2 and 2..3, but is only started at 1.5, halfway through the second iteration
        let event = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 2,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(BasicPointDefinition::new(vec![
                BasicPointData::new(
                    ModifierValues::Static(0.0),
                    0.0,
                    false,
                    vec![],
                    Functions::EaseLinear,
                ),
                BasicPointData::new(
                    ModifierValues::Static(10.0),
                    1.0,
                    false,
                    vec![],
                    Functions::EaseLinear,
                ),
            ]))),
        };

        let mut cm = CoroutineManager::default();
        cm.start_event_coroutine(60.0, SongTime::new(1.5), &ctx, &mut holder, event);

        let dissolve = |holder: &TracksHolder| {
            holder
                .get_track(key)
                .unwrap()
                .properties
                .dissolve
                .get_value()
                .unwrap()
                .as_float()
                .unwrap()
        };

        assert!(
            (dissolve(&holder) - 5.0).abs() < 1e-4,
            "got {}",
            dissolve(&holder)
        );

        // a quarter into the third iteration
        cm.poll_events(SongTime::new(2.25), &ctx, &mut holder);
        assert!(
            (dissolve(&holder) - 2.5).abs() < 1e-4,
            "got {}",
            dissolve(&holder)
        );

        cm.poll_events(SongTime::new(3.5), &ctx, &mut holder);
        assert_eq!(dissolve(&holder), 10.0);
        assert!(cm.coroutines.is_empty(), "all repeats should have finished");
    }

    #[test]
    fn repeat_event_restarts_once() {
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "r_track".to_string();
        let key = holder.add_track(t);

        // repeat = 1 -> should run twice
        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 1,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd)),
        };

        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // halfway through first iteration
        cm.poll_events(SongTime::new(0.5), &ctx, &mut holder);
        let v1 = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v1 - 5.0).abs() < 1e-3,
            "expected ~5.0 during first iteration, got {}",
            v1
        );

        // after first completes (slightly after 1.0) it should restart for second iteration
        cm.poll_events(SongTime::new(1.01), &ctx, &mut holder);

        // during second iteration at 1.5 (0.5 into second), value should again be ~5.0
        cm.poll_events(SongTime::new(1.5), &ctx, &mut holder);
        let v2 = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v2 - 5.0).abs() < 1e-2,
            "expected ~5.0 during second iteration, got {}",
            v2
        );
    }

    #[test]
    fn event_duration_scales_with_bpm_and_value_persists_after_expiry() {
        let mut cm = CoroutineManager::default();
        // use bpm=120 so duration = (60 * raw_duration) / bpm = 0.5 when raw_duration=1.0
        let bpm = 120.0;
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "persist_track".to_string();
        let key = holder.add_track(t);

        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd)),
        };

        // Start at song_time = 0.0
        cm.start_event_coroutine(bpm, SongTime::new(0.0), &ctx, &mut holder, ev);

        // Half of duration (duration = 0.5) occurs at song_time = 0.25
        cm.poll_events(SongTime::new(0.25), &ctx, &mut holder);
        let v_half = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        // Expect around 5.0
        assert!((v_half - 5.0).abs() < 1e-3, "expected ~5.0 got {}", v_half);

        // After duration (0.5), at song_time = 0.6 the coroutine should finish and value be final
        cm.poll_events(SongTime::new(0.6), &ctx, &mut holder);
        let v_final = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v_final - 10.0).abs() < 1e-6,
            "expected final 10.0 got {}",
            v_final
        );

        // No coroutines should remain for this manager (event expired)
        assert!(cm.coroutines.is_empty(), "expected no coroutines left");

        // Poll much later and ensure value remains the same
        cm.poll_events(SongTime::new(2.0), &ctx, &mut holder);
        let v_later = holder
            .get_track(key)
            .unwrap()
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!(
            (v_later - 10.0).abs() < 1e-6,
            "value should persist at final value"
        );
    }

    #[test]
    fn animate_track_matches_csharp_coroutine_semantics() {
        use crate::point_data::basic_point_data::BasicPointData;
        use crate::point_definition::basic_point_definition::BasicPointDefinition;

        // prepare points 0 -> 10 over 0..1
        let pd = BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(0.0),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(10.0),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]);

        // common parameters
        let duration = 1.0_f32;
        let easing = Functions::EaseLinear;
        let non_lazy = true;
        let repeat = 1; // run twice total

        // create a CoroutineManager and a TracksHolder with a track that has a `dissolve` property
        let mut cm = CoroutineManager::default();
        let ctx = BaseProviderContext::new();

        let mut holder = TracksHolder::new();
        let mut t = Track::default();
        t.name = "coro_track".to_string();
        let key = holder.add_track(t);

        // event definition (raw_duration 1.0 -> duration = 1.0 when bpm=60)
        let ev = EventData {
            raw_duration: BpmTime::new(1.0),
            easing,
            repeat,
            start_song_time: SongTime::new(0.0),
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(BasePointDefinition::Float(pd.clone())),
        };

        // Start the coroutine (bpm=60 -> duration_song_time = 1.0)
        cm.start_event_coroutine(60.0, SongTime::new(0.0), &ctx, &mut holder, ev);

        // C#-side simulation property
        let mut prop_csharp = ValueProperty::empty(WrapBaseValueType::Float);

        // C# simulation state
        let mut cs_repeat = repeat as i32;
        let mut cs_start = 0.0_f32;
        let mut cs_skip = false;

        // We'll step song_time forward in small increments and compare property values after each frame
        let mut song_time = 0.0_f32;
        let dt = 0.1_f32;
        let mut iter = 0;

        loop {
            iter += 1;
            assert!(iter < 500, "test timed out");

            // --- C# step: simulate full coroutine loop (may iterate multiple times in same tick) ---
            let mut cs_done = false;
            loop {
                let elapsed = song_time - cs_start;
                if !cs_skip {
                    let normalized = (elapsed / duration).min(1.0);
                    let time = easing.interpolate(normalized);
                    let on_last = {
                        let (value, finished) = pd.interpolate(time, &BaseProviderContext::new());
                        prop_csharp.set_value(Some(crate::base_value::BaseValue::Float(value)));
                        finished
                    };
                    cs_skip = !non_lazy && on_last;
                }

                if elapsed < duration {
                    if cs_repeat <= 0 && cs_skip {
                        cs_done = true;
                    }
                    break;
                } else {
                    cs_repeat -= 1;
                    cs_start += duration;
                    cs_skip = false;
                    if cs_repeat < 0 {
                        cs_done = true;
                        break;
                    }
                    // otherwise, continue the loop and run next iteration in same tick
                    continue;
                }
            }

            // --- Rust step via CoroutineManager ---
            cm.poll_events(SongTime::from(song_time), &ctx, &mut holder);

            // Read the property's value from the track
            let track = holder.get_track(key).unwrap();
            let v_r = track
                .properties
                .dissolve
                .get_value()
                .map(|v| v.as_float().unwrap());

            let v_cs = prop_csharp.get_value().map(|v| v.as_float().unwrap());

            assert_eq!(v_cs, v_r, "property mismatch at song_time {}", song_time);

            let r_done = cm.coroutines.is_empty();

            if cs_done && r_done {
                break;
            }

            song_time += dt;
        }
    }
}
