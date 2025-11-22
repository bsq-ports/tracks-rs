use log::debug;

use crate::{
    animation::{
        track::Track,
        tracks_holder::{TrackKey, TracksHolder},
    },
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    point_definition::{
        PointDefinition,
        base_point_definition::{self},
    },
};

use super::{
    events::{EventData, EventType},
    property::{PathProperty, ValueProperty},
};

#[derive(Default)]
pub struct CoroutineManager {
    coroutines: Vec<CoroutineTask>,
}

/// Represents a single coroutine task for an event.
struct CoroutineTask {
    event_type: EventType,
    repeat: u32,
    duration_song_time: f32,
    easing: Functions,
    start_song_time: f32,
    track_key: TrackKey,
    point_definition: Option<base_point_definition::BasePointDefinition>,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum CoroutineResult {
    Yield,
    Break,
}

impl EventType {
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
    pub fn start_event_coroutine(
        &mut self,
        bpm: f32,
        song_time: f32,
        provider_context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
        event_group_data: EventData,
    ) {
        let duration = (60.0 * event_group_data.raw_duration) / bpm;

        let start_song_time = event_group_data.start_song_time;
        let easing = event_group_data.easing;
        let repeat = event_group_data.repeat;

        // cancel any existing coroutines for the same event type
        self.coroutines
            .retain(|c| c.event_type != event_group_data.property);

        let value = Self::make_event_task(
            song_time,
            duration,
            start_song_time,
            easing,
            repeat,
            event_group_data,
            provider_context,
            tracks_holder,
        );
        let Some(value) = value else {
            debug!("CoroutineTask has 0 duration or no points, skipping");
            return;
        };

        self.coroutines.push(value);

        // let event_tasks = event_group_data
        //     .coroutine_infos
        //     .into_iter()
        //     .filter_map(|data| {
        //         Self::enqueue_event(
        //             song_time, duration, start_time, easing, repeat, data, context,
        //         )
        //     })
        //     .collect_vec();

        // self.coroutines.extend(event_tasks);
    }

    fn make_event_task(
        song_time: f32,
        duration: f32,
        start_song_time: f32,

        easing: Functions,
        repeat: u32,
        data: EventData,
        provider_context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) -> Option<CoroutineTask> {
        let mut repeat = repeat;
        let no_duration =
            duration == 0.0 || start_song_time + (duration * (repeat as f32 + 1.0)) < song_time;
        let mut property = data.property;
        let track_key = data.track_key;

        // use an optional point data to move it into the coroutine task
        let mut point_data = data.point_data;
        let track = tracks_holder
            .get_track_mut(track_key)
            .expect("Track not found for CoroutineTask");
        if point_data.is_none() {
            property.set_null(track);
            return None;
        };

        match &mut property {
            EventType::AnimateTrack(property_handle) => {
                let property = track
                    .properties
                    .get_by_handle_mut(property_handle)
                    .expect("Property not found");

                let point_data = point_data.as_ref().unwrap();

                let has_base = point_data.has_base_provider();
                if no_duration || (point_data.get_points().len() <= 1 && !has_base) {
                    set_property_value(point_data, property, 1.0, provider_context);
                    return None;
                }

                let result = animate_track(
                    point_data,
                    property,
                    duration,
                    start_song_time,
                    song_time,
                    easing,
                    has_base,
                    provider_context,
                );
                if result == CoroutineResult::Break {
                    repeat = repeat.saturating_sub(1);
                }

                if repeat == 0 && result == CoroutineResult::Break {
                    return None;
                }
            }
            EventType::AssignPathAnimation(path_property_handle) => {
                let path_property = track
                    .path_properties
                    .get_by_handle_mut(path_property_handle)
                    .expect("Path property not found");

                path_property.init(point_data.take());

                if no_duration {
                    path_property.finish();
                    return None;
                }
                let res =
                    assign_path_animation(path_property, duration, start_song_time, easing, song_time);
                if res == CoroutineResult::Break {
                    return None;
                }
            }
        };
        Some(CoroutineTask {
            easing,
            track_key,
            event_type: property,

            point_definition: point_data,

            repeat,
            duration_song_time: duration,
            start_song_time,
        })
    }

    pub fn poll_events(
        &mut self,
        song_time: f32,
        context: &BaseProviderContext,
        tracks_holder: &mut TracksHolder,
    ) {
        // Yield and remove coroutines that are finished
        self.coroutines.retain_mut(|event| {
            Self::poll_event(song_time, context, event, tracks_holder) == CoroutineResult::Yield
        });
    }

    fn poll_event(
        song_time: f32,
        context: &BaseProviderContext,
        event_data: &mut CoroutineTask,
        tracks_holder: &mut TracksHolder,
    ) -> CoroutineResult {
        let duration = event_data.duration_song_time;
        let track = tracks_holder
            .get_track_mut(event_data.track_key)
            .expect("Track not found for CoroutineTask");

        match &mut event_data.event_type {
            EventType::AnimateTrack(value_property_handle) => {
                let point_def = match &event_data.point_definition {
                    Some(def) => def,
                    None => {
                        debug!("No point definition for AnimateTrack event, skipping");
                        return CoroutineResult::Break;
                    }
                };
                let has_base = point_def.has_base_provider();
                let value_property = track
                    .properties
                    .get_by_handle_mut(value_property_handle)
                    .expect("Property not found");

                let mut result = animate_track(
                    point_def,
                    value_property,
                    duration,
                    event_data.start_song_time,
                    song_time,
                    event_data.easing,
                    has_base,
                    context,
                );

                // when we repeat, we restart state
                if result == CoroutineResult::Break && event_data.repeat > 0 {
                    event_data.repeat = event_data.repeat.saturating_sub(1);
                    event_data.start_song_time += duration;
                    result = CoroutineResult::Yield;
                }

                result
            }
            EventType::AssignPathAnimation(path_property_handle) => {
                let path_property = track
                    .path_properties
                    .get_by_handle_mut(path_property_handle)
                    .expect("Path property not found");

                assign_path_animation(
                    path_property,
                    duration,
                    event_data.start_song_time,
                    event_data.easing,
                    song_time,
                )
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn animate_track(
    points: &base_point_definition::BasePointDefinition,
    property: &mut ValueProperty,
    duration: f32,
    start_time: f32,
    song_time: f32,
    easing: Functions,
    non_lazy: bool,
    context: &BaseProviderContext,
) -> CoroutineResult {
    let elapsed_time = song_time - start_time;

    let normalized_time = (elapsed_time / duration).min(1.0);
    let time = easing.interpolate(normalized_time);
    let on_last = set_property_value(points, property, time, context);
    let skip = !non_lazy && on_last;

    if elapsed_time < duration && !skip {
        return CoroutineResult::Yield;
    }

    CoroutineResult::Break
}

fn assign_path_animation(
    interpolation: &mut PathProperty,
    duration: f32,
    start_time: f32,
    easing: Functions,
    song_time: f32,
) -> CoroutineResult {
    let elapsed_time = song_time - start_time;
    let normalized_time = (elapsed_time / duration).min(1.0);
    interpolation.time = easing.interpolate(normalized_time);

    if elapsed_time < duration {
        return CoroutineResult::Yield;
    }

    interpolation.finish();
    CoroutineResult::Break
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
    use crate::animation::track::Track;
    use crate::animation::track::ValuePropertyHandle;
    use crate::animation::tracks_holder::TracksHolder;
    use crate::base_provider_context::BaseProviderContext;
    use crate::easings::functions::Functions;
    use crate::modifiers::float_modifier::FloatValues;
    use crate::point_data::PointData;
    use crate::point_data::float_point_data::FloatPointData;
    use crate::point_definition::float_point_definition::FloatPointDefinition;

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
        let pd = FloatPointDefinition::new(vec![
            PointData::Float(FloatPointData::new(
                FloatValues::Static(0.0),
                0.0,
                vec![],
                Functions::EaseLinear,
            )),
            PointData::Float(FloatPointData::new(
                FloatValues::Static(10.0),
                1.0,
                vec![],
                Functions::EaseLinear,
            )),
        ]);

        let ev = EventData {
            raw_duration: 1.0,
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: 0.0,
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some(pd.into()),
        };

        // bpm 60 => duration = 1.0 for raw_duration 1.0
        cm.start_event_coroutine(60.0, 0.0, &ctx, &mut holder, ev);

        // poll at halfway through duration (0.5) - should set dissolve ~5.0
        cm.poll_events(0.5, &ctx, &mut holder);

        let track = holder.get_track(key).unwrap();
        let val = track.properties.dissolve.get_value().expect("value set");
        let f = val.as_float().unwrap();
        assert!((f - 5.0).abs() < 1e-3, "expected ~5.0 got {}", f);
    }
}
