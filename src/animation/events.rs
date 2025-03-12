use crate::easings::functions::Functions;
use crate::point_definition::{BasePointDefinition, PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;

use super::property::PathProperty;
use super::{property::ValueProperty, tracks::Track};
use std::collections::LinkedList;
use std::sync::Arc;
use std::time::Duration;
use std::vec;

pub struct CoroutineManager {
    coroutines: Vec<EventTask>,
}

pub struct EventData {
    pub duration: f32,
    pub easing: Functions,
    pub repeat: i32,
    pub coroutine_infos: Vec<CoroutineInfo>,
    pub start_time: f32,
}

pub struct CoroutineInfo {
    pub property: CoroutineProperty,
    pub track: Track,
}

pub enum CoroutineProperty {
    AnimateTrack(ValueProperty, BasePointDefinition),
    AssignPathAnimation(PathProperty, Option<BasePointDefinition>),
}

struct EventTask {
    property: CoroutineProperty,
    repeat: i32,
    duration_song_time: f32,
    easing: Functions,
    start_time: f32,
    track: Track,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum CoroutineResult {
    Yield,
    Break,
}

impl CoroutineProperty {
    pub(crate) fn set_null(&mut self) {
        match self {
            CoroutineProperty::AnimateTrack(property, _) => property.set_null(),
            CoroutineProperty::AssignPathAnimation(path_property, _) => path_property.init(None),
        }
    }

    pub fn get_point_definition(&self) -> Option<&BasePointDefinition> {
        match self {
            CoroutineProperty::AnimateTrack(_, base_point_definition) => {
                Some(base_point_definition)
            }
            CoroutineProperty::AssignPathAnimation(_, base_point_definition) => {
                base_point_definition.as_ref()
            }
        }
    }
}

impl CoroutineManager {
    pub fn start_event_coroutine(
        &mut self,
        bpm: f32,
        song_time: f32,
        context: &BaseProviderContext,
        event_data: EventData,
    ) {
        let duration = (60.0 * event_data.duration) / bpm;


        let event_tasks = event_data.coroutine_infos.into_iter().map(|coroutine_info| EventTask{
            property: coroutine_info.property,
            track: coroutine_info.track,
            repeat: event_data.repeat,
            duration_song_time: duration,
            easing: event_data.easing,
            start_time: event_data.start_time,
        }).filter_map(|e| {
            self.poll_event(bpm, song_time, context, event_data)
        });

        let Some(event_data) = self.poll_event(bpm, song_time, context, event_data) else {
            return;
        };

        self.coroutines.push(event_data);
    }

    pub fn poll_events(mut self, bpm: f32, song_time: f32, context: &BaseProviderContext) {
        let coroutines = self.coroutines;
        self.coroutines = vec![];

        self.coroutines = coroutines
            .into_iter()
            .filter_map(|event| self.poll_event(bpm, song_time, context, event))
            .collect();
    }

    fn poll_event(
        &mut self,
        bpm: f32,
        song_time: f32,
        context: &BaseProviderContext,
        mut event_data: EventData,
    ) -> Option<EventData> {
        let duration = (60.0 * event_data.duration) / bpm;
        let easing = event_data.easing;

        let no_duration = duration == 0.0
            || event_data.start_time + (duration * (event_data.repeat as f32 + 1.0)) < song_time;

        let remaining_events: Vec<CoroutineInfo> = event_data
            .coroutine_infos
            .into_iter()
            .filter_map(|mut info| {
                if info.property.get_point_definition().is_none() {
                    info.track.mark_updated();
                    info.property.set_null();
                    return None;
                }

                let has_base = info
                    .property
                    .get_point_definition()
                    .is_some_and(|t| t.has_base_provider());

                match &mut info.property {
                    CoroutineProperty::AnimateTrack(property, point_data) => {
                        if no_duration || (point_data.get_points().len() <= 1 && !has_base) {
                            set_property_value(point_data, property, &mut info.track, 1.0, context);
                            return None;
                        }

                        

                        let result = animate_track(
                            point_data,
                            property,
                            &mut info.track,
                            duration,
                            event_data.start_time,
                            song_time,
                            easing,
                            has_base,
                            context,
                        );
                        if result == CoroutineResult::Break {
                            event_data.repeat -= 1;
                        }

                        (event_data.repeat > 0).then_some(info)
                    }
                    CoroutineProperty::AssignPathAnimation(path_property, point_data) => {
                        path_property.init(point_data.take());

                        if no_duration {
                            path_property.finish();
                            return None;
                        }
                        let cont = assign_path_animation(
                            path_property,
                            duration,
                            event_data.start_time,
                            easing,
                            song_time,
                        ) == CoroutineResult::Yield;
                        cont.then_some(info)
                    }
                }
            })
            .collect();

        if remaining_events.is_empty() {
            return None;
        }

        let new_event_data = EventData {
            coroutine_infos: remaining_events,
            ..event_data
        };

        Some(new_event_data)
    }
}

fn animate_track(
    points: &mut BasePointDefinition,
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
    points: &mut BasePointDefinition,
    property: &mut ValueProperty,
    track: &mut Track,
    time: f32,
    context: &BaseProviderContext,
) -> bool {
    let (value, on_last) = points.interpolate(time, context);

    if value == property.get_value() {
        return on_last;
    }

    property.update_value(value);
    track.mark_updated();
    on_last
}
