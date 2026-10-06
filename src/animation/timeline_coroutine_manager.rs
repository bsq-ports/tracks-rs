use std::collections::HashMap;

use crate::{
    animation::{
        events::{EventData, EventType},
        track::{PathPropertyHandle, Track, ValuePropertyHandle},
        tracks_holder::{TrackKey, TracksHolder},
    },
    base_provider_context::BaseProviderContext,
    base_value::BaseValue,
    easings::functions::Functions,
    point_definition::{PointDefinitionLike, base_point_definition::BasePointDefinition},
    time_types::SongTime,
};

/// Evaluates track events at any song time, instead of stepping forward like [`super::coroutine_manager::CoroutineManager`].
///
/// All events are registered up front. They are grouped into one timeline per
/// `(track, property)` and sorted by start time. Starting an event overrides the previous
/// one on the same property, so the state at song time `x` depends only on the **last event
/// with `start <= x`**. A path property also needs the event before it, because that is the
/// path it blends from. Nothing has to be replayed, so you can seek forwards or backwards.
///
/// - [`Self::value_at`] and [`Self::path_at`] are read-only queries.
/// - [`Self::apply`] writes the snapshot at `x` into a [`TracksHolder`]. Properties with no
///   active event at `x` are reset to `None`.
///
/// The evaluation rules match `CoroutineManager`:
/// - `AnimateTrack` repeats `repeat + 1` times and then holds the final value.
/// - `AssignPathAnimation` blends from the previous path to the new one over the duration, then finishes.
/// - Events with no point data clear the property.
#[derive(Clone, Default)]
pub struct TimelineCoroutineManager {
    tracks: HashMap<TrackKey, TrackTimelines>,
}

/// The timelines of one track. Value and path properties behave differently, so they have
/// separate event types and maps, and lookups can borrow the handle instead of cloning it
/// (custom property handles own a `String`).
#[derive(Clone, Default)]
struct TrackTimelines {
    properties: HashMap<ValuePropertyHandle, Timeline<ValueEvent>>,
    path_properties: HashMap<PathPropertyHandle, Timeline<PathEvent>>,
}

/// An event that can be placed on a [`Timeline`].
trait TimedEvent {
    fn start_song_time(&self) -> SongTime;
}

/// The events that target one property on one track, sorted by start time.
#[derive(Clone)]
struct Timeline<E: TimedEvent> {
    events: Vec<E>,
}

impl<E: TimedEvent> Default for Timeline<E> {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}

/// An `AnimateTrack` event.
#[derive(Clone)]
struct ValueEvent {
    start_song_time: SongTime,
    duration_song_time: SongTime,
    repeat: u32,
    easing: Functions,
    point_data: Option<BasePointDefinition>,
}

/// An `AssignPathAnimation` event. It has no `repeat`: path animations finish after one iteration.
#[derive(Clone)]
struct PathEvent {
    start_song_time: SongTime,
    duration_song_time: SongTime,
    easing: Functions,
    point_data: Option<BasePointDefinition>,
}

impl TimedEvent for ValueEvent {
    fn start_song_time(&self) -> SongTime {
        self.start_song_time
    }
}

impl TimedEvent for PathEvent {
    fn start_song_time(&self) -> SongTime {
        self.start_song_time
    }
}

/// The state of a path property at a given song time.
#[derive(Debug, Clone, Copy)]
pub struct PathSnapshot<'a> {
    /// The path being blended from. `None` once the blend has finished.
    pub prev_point: Option<&'a BasePointDefinition>,
    pub point: Option<&'a BasePointDefinition>,
    /// Eased blend from `prev_point` to `point`.
    pub interpolate_time: f32,
}

impl PathSnapshot<'_> {
    /// Samples the path at `time` (an object's lifetime). Works the same as `PathProperty::interpolate`.
    pub fn interpolate(&self, time: f32, context: &BaseProviderContext) -> Option<BaseValue> {
        let point = self.point?.interpolate(time, context).0;
        match self.prev_point {
            Some(prev) => {
                let prev = prev.interpolate(time, context).0;
                Some(prev.lerp_clamped(point, self.interpolate_time))
            }
            None => Some(point),
        }
    }
}

impl ValueEvent {
    /// Eased progress through the current repeat iteration.
    /// Holds at the end once every iteration has elapsed.
    fn animate_time(&self, song_time: SongTime) -> f32 {
        let duration = self.duration_song_time;
        if duration <= SongTime::ZERO {
            return 1.0;
        }

        let elapsed = song_time - self.start_song_time;
        if elapsed >= duration * (self.repeat as f64 + 1.0) {
            return 1.0;
        }

        // each repeat restarts at the iteration boundary
        let local = elapsed - duration * (elapsed / duration).floor();
        self.easing
            .interpolate((local / duration).clamp(0.0, 1.0) as f32)
    }
}

impl PathEvent {
    /// Eased blend time, or `None` once the animation has finished.
    fn blend_time(&self, song_time: SongTime) -> Option<f32> {
        let duration = self.duration_song_time;
        let elapsed = song_time - self.start_song_time;
        if duration <= SongTime::ZERO || elapsed >= duration {
            return None;
        }

        Some(self.easing.interpolate((elapsed / duration) as f32))
    }
}

impl<E: TimedEvent> Timeline<E> {
    /// Inserts `event` keeping the timeline sorted by start time.
    /// Inserting after equal start times keeps the later-added event active,
    /// and is cheap when events arrive already sorted.
    fn insert(&mut self, event: E) {
        let start = event.start_song_time();
        let position = self
            .events
            .partition_point(|e| e.start_song_time() <= start);
        self.events.insert(position, event);
    }

    /// Index of the last event with `start <= song_time`.
    fn active_index(&self, song_time: SongTime) -> Option<usize> {
        // binary search
        self.events
            .partition_point(|e| e.start_song_time() <= song_time)
            .checked_sub(1)
    }
}

impl Timeline<ValueEvent> {
    /// Value of the active event at `song_time`, or `None` if no event is active or it has no point data.
    fn value_at(&self, song_time: SongTime, context: &BaseProviderContext) -> Option<BaseValue> {
        let event = &self.events[self.active_index(song_time)?];
        let points = event.point_data.as_ref()?;
        Some(points.interpolate(event.animate_time(song_time), context).0)
    }
}

impl Timeline<PathEvent> {
    /// A path property also needs the event before the active one: that is the path it blends from.
    fn path_at(&self, song_time: SongTime) -> PathSnapshot<'_> {
        let Some(index) = self.active_index(song_time) else {
            return PathSnapshot {
                prev_point: None,
                point: None,
                interpolate_time: 0.0,
            };
        };

        let event = &self.events[index];
        let prev_point = index
            .checked_sub(1)
            .and_then(|i| self.events[i].point_data.as_ref());

        let Some(point) = event.point_data.as_ref() else {
            // a null event never starts a blend, so the previous path is kept but unused
            return PathSnapshot {
                prev_point,
                point: None,
                interpolate_time: 0.0,
            };
        };

        match event.blend_time(song_time) {
            Some(interpolate_time) => PathSnapshot {
                prev_point,
                point: Some(point),
                interpolate_time,
            },
            None => PathSnapshot {
                prev_point: None,
                point: Some(point),
                interpolate_time: 1.0,
            },
        }
    }
}

impl TrackTimelines {
    /// Writes the state at `song_time` into every property of `track` that has events.
    fn apply(&self, song_time: SongTime, context: &BaseProviderContext, track: &mut Track) {
        for (handle, timeline) in &self.properties {
            track
                .properties
                .get_by_handle_mut(handle)
                .expect("Property not found")
                .set_value(timeline.value_at(song_time, context));
        }

        for (handle, timeline) in &self.path_properties {
            let path_property = track
                .path_properties
                .get_by_handle_mut(handle)
                .expect("Path property not found");

            let snapshot = timeline.path_at(song_time);
            // clones up to two point definitions per call. if this shows up in profiles,
            // storing them as `Rc<BasePointDefinition>` in `PathProperty` would make this a refcount bump
            path_property.prev_point = snapshot.prev_point.cloned();
            path_property.point = snapshot.point.cloned();
            path_property.interpolate_time = snapshot.interpolate_time;
        }
    }
}

impl TimelineCoroutineManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a manager from every event in the map.
    pub fn from_events(bpm: f32, events: impl IntoIterator<Item = EventData>) -> Self {
        let mut manager = Self::new();
        for event in events {
            manager.add_event(bpm, event);
        }
        manager
    }

    /// Adds an event to its `(track, property)` timeline, keeping the timeline sorted by start time.
    /// When two events start at the same time, the one added later wins.
    pub fn add_event(&mut self, bpm: f32, event: EventData) {
        let track = self.tracks.entry(event.track_key).or_default();
        let start_song_time = event.start_song_time;
        let duration_song_time = event.raw_duration.to_song_time(bpm as f64);

        match event.property {
            EventType::AnimateTrack(handle) => {
                track.properties.entry(handle).or_default().insert(ValueEvent {
                    start_song_time,
                    duration_song_time,
                    repeat: event.repeat,
                    easing: event.easing,
                    point_data: event.point_data,
                })
            }
            EventType::AssignPathAnimation(handle) => track
                .path_properties
                .entry(handle)
                .or_default()
                .insert(PathEvent {
                    start_song_time,
                    duration_song_time,
                    easing: event.easing,
                    point_data: event.point_data,
                }),
        }
    }

    /// Value of an animated property at `song_time`.
    /// Returns `None` if no event is active or the active event has no point data.
    pub fn value_at(
        &self,
        song_time: SongTime,
        track_key: TrackKey,
        handle: &ValuePropertyHandle,
        context: &BaseProviderContext,
    ) -> Option<BaseValue> {
        self.tracks
            .get(&track_key)?
            .properties
            .get(handle)?
            .value_at(song_time, context)
    }

    /// Path state at `song_time`. Returns `None` if no event ever targets this path property.
    pub fn path_at(
        &self,
        song_time: SongTime,
        track_key: TrackKey,
        handle: &PathPropertyHandle,
    ) -> Option<PathSnapshot<'_>> {
        self.tracks
            .get(&track_key)?
            .path_properties
            .get(handle)
            .map(|timeline| timeline.path_at(song_time))
    }

    /// Writes the snapshot at `song_time` into every property that has events.
    /// Can be called with any time, in any order.
    pub fn apply(
        &self,
        song_time: SongTime,
        context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) {
        for (track_key, timelines) in &self.tracks {
            let track = tracks_holder
                .get_track_mut(*track_key)
                .expect("Track not found for replay timeline");
            timelines.apply(song_time, context, track);
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::{Vec3, Vec4};

    use super::*;
    use crate::animation::coroutine_manager::CoroutineManager;
    use crate::animation::track::{PathPropertyHandle, Track, ValuePropertyHandle};
    use crate::modifiers::ModifierValues;
    use crate::point_data::basic_point_data::BasicPointData;
    use crate::point_definition::basic_point_definition::BasicPointDefinition;
    use crate::point_definition::vector3_point_definition::Vector3PointDefinition;

    fn float_points(from: f32, to: f32) -> BasePointDefinition {
        BasePointDefinition::Float(BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(from),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(to),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]))
    }

    fn color_points(from: f32, to: f32) -> BasePointDefinition {
        BasePointDefinition::Vector4(BasicPointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec4::splat(from)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec4::splat(to)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]))
    }

    fn path_points(from: f32, to: f32) -> BasePointDefinition {
        BasePointDefinition::Vector3(Vector3PointDefinition::new(vec![
            BasicPointData::new(
                ModifierValues::Static(Vec3::splat(from)),
                0.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
            BasicPointData::new(
                ModifierValues::Static(Vec3::splat(to)),
                1.0,
                false,
                vec![],
                Functions::EaseLinear,
            ),
        ]))
    }

    fn event(
        track_key: TrackKey,
        property: EventType,
        start: f32,
        duration: f32,
        repeat: u32,
        point_data: Option<BasePointDefinition>,
    ) -> EventData {
        EventData {
            raw_duration: duration.into(),
            easing: Functions::EaseLinear,
            repeat,
            start_song_time: start.into(),
            property,
            track_key,
            point_data,
        }
    }

    fn dissolve() -> EventType {
        EventType::AnimateTrack(ValuePropertyHandle::new("dissolve"))
    }
    fn color() -> EventType {
        EventType::AnimateTrack(ValuePropertyHandle::new("color"))
    }
    fn definite_position() -> EventType {
        EventType::AssignPathAnimation(PathPropertyHandle::new("definitePosition"))
    }

    fn holder_with_tracks() -> (TracksHolder, TrackKey, TrackKey) {
        let mut holder = TracksHolder::new();
        let mut a = Track::default();
        a.name = "a".to_string();
        let mut b = Track::default();
        b.name = "b".to_string();
        let a = holder.add_track(a);
        let b = holder.add_track(b);
        (holder, a, b)
    }

    /// Overrides, repeats, zero duration, null events, several tracks and properties.
    /// Events are sorted by start time, as the live manager would receive them.
    fn scenario(a: TrackKey, b: TrackKey) -> Vec<EventData> {
        let mut events = vec![
            event(a, dissolve(), 0.0, 1.0, 0, Some(float_points(0.0, 10.0))),
            event(b, dissolve(), 0.0, 1.0, 2, Some(float_points(5.0, 15.0))),
            event(
                a,
                definite_position(),
                0.0,
                1.0,
                0,
                Some(path_points(0.0, 3.0)),
            ),
            event(a, color(), 0.2, 2.0, 0, Some(color_points(0.0, 4.0))),
            event(a, dissolve(), 0.5, 1.0, 1, Some(float_points(0.0, 20.0))),
            event(
                a,
                definite_position(),
                1.5,
                1.0,
                0,
                Some(path_points(10.0, 20.0)),
            ),
            event(
                a,
                definite_position(),
                2.0,
                0.0,
                0,
                Some(path_points(-1.0, 1.0)),
            ),
            event(
                a,
                definite_position(),
                2.6,
                1.0,
                0,
                Some(path_points(5.0, 6.0)),
            ),
            event(a, dissolve(), 3.0, 1.0, 0, None),
            event(a, definite_position(), 3.3, 1.0, 0, None),
            event(a, dissolve(), 3.5, 0.0, 0, Some(float_points(0.0, 7.0))),
        ];
        events.sort_by(|l, r| l.start_song_time.total_cmp(&r.start_song_time));
        events
    }

    fn float_of(v: Option<BaseValue>) -> Option<f32> {
        v.map(|v| v.as_float().unwrap())
    }

    fn assert_close(l: Option<BaseValue>, r: Option<BaseValue>, what: &str, t: SongTime) {
        match (l, r) {
            (None, None) => {}
            (Some(l), Some(r)) => {
                let diff = (l.as_slice_raw().iter())
                    .zip(r.as_slice_raw().iter())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0, f32::max);
                assert!(diff < 1e-3, "{what} mismatch at {t}: {l:?} vs {r:?}");
            }
            _ => panic!("{what} mismatch at {t}: {l:?} vs {r:?}"),
        }
    }

    #[test]
    fn matches_live_coroutine_manager() {
        let ctx = BaseProviderContext::new();
        let (mut live_holder, a, b) = holder_with_tracks();
        let mut replay_holder = live_holder.clone();

        let events = scenario(a, b);
        let replay = TimelineCoroutineManager::from_events(60.0, events.clone());
        let mut live = CoroutineManager::default();
        let mut pending = events.into_iter().peekable();

        for i in 0..100 {
            let t = SongTime::new(i as f64 * 0.05);
            while let Some(e) = pending.next_if(|e| e.start_song_time <= t) {
                live.start_event_coroutine(60.0, t, &ctx, &mut live_holder, e);
            }
            live.poll_events(t, &ctx, &mut live_holder);
            replay.apply(t, &ctx, &mut replay_holder);

            for key in [a, b] {
                let l = live_holder.get_track(key).unwrap();
                let r = replay_holder.get_track(key).unwrap();
                assert_close(
                    l.properties.dissolve.get_value(),
                    r.properties.dissolve.get_value(),
                    "dissolve",
                    t,
                );
                assert_close(
                    l.properties.color.get_value(),
                    r.properties.color.get_value(),
                    "color",
                    t,
                );

                let l_path = &l.path_properties.definite_position;
                let r_path = &r.path_properties.definite_position;
                assert_eq!(
                    l_path.prev_point.is_some(),
                    r_path.prev_point.is_some(),
                    "path prev at {t}"
                );
                assert_eq!(
                    l_path.point.is_some(),
                    r_path.point.is_some(),
                    "path point at {t}"
                );
                for path_time in [0.0, 0.25, 0.5, 1.0] {
                    assert_close(
                        l_path.interpolate(path_time, &ctx),
                        r_path.interpolate(path_time, &ctx),
                        "path",
                        t,
                    );
                }
            }

            // the read-only queries agree with what apply wrote
            let r = replay_holder.get_track(a).unwrap();
            assert_close(
                replay.value_at(t, a, &ValuePropertyHandle::new("dissolve"), &ctx),
                r.properties.dissolve.get_value(),
                "value_at",
                t,
            );
            let snapshot = replay
                .path_at(t, a, &PathPropertyHandle::new("definitePosition"))
                .unwrap();
            assert_close(
                snapshot.interpolate(0.5, &ctx),
                r.path_properties.definite_position.interpolate(0.5, &ctx),
                "path_at",
                t,
            );
        }
    }

    #[test]
    fn seeking_matches_fresh_evaluation() {
        let ctx = BaseProviderContext::new();
        let (holder, a, b) = holder_with_tracks();
        let replay = TimelineCoroutineManager::from_events(60.0, scenario(a, b));
        let mut seek_holder = holder.clone();

        for t in [2.5, 0.5, 1.75, 0.1, 3.4, 2.5].map(SongTime::new) {
            replay.apply(t, &ctx, &mut seek_holder);

            let mut fresh_holder = holder.clone();
            TimelineCoroutineManager::from_events(60.0, scenario(a, b)).apply(
                t,
                &ctx,
                &mut fresh_holder,
            );

            for key in [a, b] {
                let s = seek_holder.get_track(key).unwrap();
                let f = fresh_holder.get_track(key).unwrap();
                assert_close(
                    s.properties.dissolve.get_value(),
                    f.properties.dissolve.get_value(),
                    "dissolve",
                    t,
                );
                assert_close(
                    s.properties.color.get_value(),
                    f.properties.color.get_value(),
                    "color",
                    t,
                );
                for path_time in [0.0, 0.5, 1.0] {
                    assert_close(
                        s.path_properties
                            .definite_position
                            .interpolate(path_time, &ctx),
                        f.path_properties
                            .definite_position
                            .interpolate(path_time, &ctx),
                        "path",
                        t,
                    );
                }
            }
        }
    }

    #[test]
    fn before_first_event_is_none_and_final_value_persists() {
        let ctx = BaseProviderContext::new();
        let (_, a, _) = holder_with_tracks();
        let replay = TimelineCoroutineManager::from_events(
            60.0,
            [event(
                a,
                dissolve(),
                1.0,
                1.0,
                1,
                Some(float_points(0.0, 10.0)),
            )],
        );
        let handle = ValuePropertyHandle::new("dissolve");

        assert_eq!(
            float_of(replay.value_at(SongTime::new(0.5), a, &handle, &ctx)),
            None
        );
        assert_eq!(
            float_of(replay.value_at(SongTime::new(1.5), a, &handle, &ctx)),
            Some(5.0)
        );
        // second repeat iteration
        assert_eq!(
            float_of(replay.value_at(SongTime::new(2.25), a, &handle, &ctx)),
            Some(2.5)
        );
        assert_eq!(
            float_of(replay.value_at(SongTime::new(3.0), a, &handle, &ctx)),
            Some(10.0)
        );
        assert_eq!(
            float_of(replay.value_at(SongTime::new(100.0), a, &handle, &ctx)),
            Some(10.0)
        );
    }

    #[test]
    fn later_added_event_wins_ties() {
        let ctx = BaseProviderContext::new();
        let (_, a, _) = holder_with_tracks();
        let replay = TimelineCoroutineManager::from_events(
            60.0,
            [
                event(a, dissolve(), 1.0, 0.0, 0, Some(float_points(0.0, 1.0))),
                event(a, dissolve(), 1.0, 0.0, 0, Some(float_points(0.0, 2.0))),
            ],
        );
        let value = replay.value_at(
            SongTime::new(1.0),
            a,
            &ValuePropertyHandle::new("dissolve"),
            &ctx,
        );
        assert_eq!(float_of(value), Some(2.0));
    }
}
