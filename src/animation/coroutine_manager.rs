use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use itertools::Itertools;

use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    point_definition::{base_point_definition::{self, BasePointDefinition}, PointDefinition},
};

use super::{
    events::{EventData, EventType},
    property::{PathProperty, ValueProperty},
    tracks::Track,
};

#[derive(Default)]
pub struct CoroutineManager<'a> {
    coroutines: Vec<CoroutineTask<'a>>,
    _marker: std::marker::PhantomData<&'a ()>,
}


struct CoroutineTask<'a> {
    event_type: EventType<'a>,
    repeat: u32,
    duration_song_time: f32,
    easing: Functions,
    start_time: f32,
    track: &'a mut Track<'a>,
    point_defintion: &'a base_point_definition::BasePointDefinition,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum CoroutineResult {
    Yield,
    Break,
}

impl EventType<'_> {
    pub(crate) fn set_null(&mut self) {
        match self {
            EventType::AnimateTrack(property) => *property = None,
            EventType::AssignPathAnimation(path_property) => path_property.init(None),
        }
    }
}

impl<'a> CoroutineManager<'a> {
    pub fn start_event_coroutine(
        &mut self,
        bpm: f32,
        song_time: f32,
        context: &'a BaseProviderContext,
        event_group_data: EventData<'a>,
    ) {
        let duration = (60.0 * event_group_data.raw_duration) / bpm;

        let start_time = event_group_data.start_time;
        let easing = event_group_data.easing;
        let repeat = event_group_data.repeat;

        let value = Self::enqueue_event(
            song_time,
            duration,
            start_time,
            easing,
            repeat,
            event_group_data,
            context,
        );
        let Some(value) = value else {
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

    fn enqueue_event(
        song_time: f32,
        duration: f32,
        start_time: f32,

        easing: Functions,
        repeat: u32,
        data: EventData<'a>,
        context: &'a BaseProviderContext,
    ) -> Option<CoroutineTask<'a>> {
        let mut repeat = repeat;
        let no_duration =
            duration == 0.0 || start_time + (duration * (repeat as f32 + 1.0)) < song_time;
        let mut property = data.property;
        let mut track = data.track;

        let Some(point_data) = data.point_data else {
            track.mark_updated();
            property.set_null();
            return None;
        };

        let has_base = point_data.has_base_provider();
        match &mut property {
            EventType::AnimateTrack(property) => {
                if no_duration || (point_data.get_points().len() <= 1 && !has_base) {
                    set_property_value(&point_data, property, &mut track, 1.0, context);
                    return None;
                }

                let result = animate_track(
                    point_data, property, &mut track, duration, start_time, song_time, easing,
                    has_base, context,
                );
                if result == CoroutineResult::Break {
                    repeat = repeat.saturating_sub(1);
                }

                if repeat == 0 && result == CoroutineResult::Break {
                    return None;
                }
            }
            EventType::AssignPathAnimation(path_property) => {
                path_property.init(Some(point_data));

                if no_duration {
                    path_property.finish();
                    return None;
                }
                let res =
                    assign_path_animation(path_property, duration, start_time, easing, song_time);
                if res == CoroutineResult::Break {
                    return None;
                }
            }
        };
        Some(CoroutineTask {
            easing,
            track,
            event_type: property,

            repeat,
            duration_song_time: duration,
            start_time,
            point_defintion: point_data,
        })
    }

    pub fn poll_events(&mut self, song_time: f32, context: &BaseProviderContext) {
        // Yield and remove coroutines that are finished
        self.coroutines.retain_mut(|event| {
            Self::poll_event(song_time, context, event) == CoroutineResult::Yield
        });
    }

    fn poll_event(
        song_time: f32,
        context: &BaseProviderContext,
        event_data: &mut CoroutineTask,
    ) -> CoroutineResult {
        let duration = event_data.duration_song_time;
        let point_def = event_data.point_defintion;

        let has_base = point_def.has_base_provider();

        match &mut event_data.event_type {
            EventType::AnimateTrack(value_property) => {
                let mut result = animate_track(
                    point_def,
                    value_property,
                    event_data.track,
                    duration,
                    event_data.start_time,
                    song_time,
                    event_data.easing,
                    has_base,
                    context,
                );

                // when we repeat, we restart state
                if result == CoroutineResult::Break && event_data.repeat > 0 {
                    event_data.repeat = event_data.repeat.saturating_sub(1);
                    event_data.start_time += duration;
                    result = CoroutineResult::Yield;
                }

                result
            }
            EventType::AssignPathAnimation(path_property) => assign_path_animation(
                path_property,
                duration,
                event_data.start_time,
                event_data.easing,
                song_time,
            ),
        }
    }
}

fn animate_track(
    points: &base_point_definition::BasePointDefinition,
    property: &mut ValueProperty,
    track: &mut Track,
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
    let on_last = set_property_value(points, property, track, time, context);
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

fn set_property_value(
    points: &base_point_definition::BasePointDefinition,
    property: &mut ValueProperty,
    track: &mut Track,
    time: f32,
    context: &BaseProviderContext,
) -> bool {
    let (value, on_last) = points.interpolate(time, context);

    if Some(value) == *property {
        return on_last;
    }

    *property = Some(value);
    track.mark_updated();
    on_last
}
