use crate::easings::functions::Functions;
use crate::point_definition::{BasePointDefinition, PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;

use super::property::PathProperty;
use super::{property::ValueProperty, tracks::Track};
use std::sync::Arc;
use std::time::Duration;

pub struct CoroutineInfo {
    pub property: CoroutineProperty,
    pub track: Track,
}

pub enum CoroutineProperty {
    ValueProperty(ValueProperty, BasePointDefinition),
    PathProperty(PathProperty, Option<BasePointDefinition>),
}

impl CoroutineProperty {
    pub(crate) fn set_null(&mut self) {
        match self {
            CoroutineProperty::ValueProperty(property, _) => property.set_null(),
            CoroutineProperty::PathProperty(path_property, _) => path_property.init(None),
        }
    }

    pub fn get_point_definition(&self) -> Option<&BasePointDefinition> {
        match self {
            CoroutineProperty::ValueProperty(_, base_point_definition) => {
                Some(base_point_definition)
            }
            CoroutineProperty::PathProperty(_, base_point_definition) => {
                base_point_definition.as_ref()
            }
        }
    }
}
pub enum EventType {
    AnimateTrack(ValueProperty),
    AssignPathAnimation(PathProperty),
}

pub struct EventData {
    pub duration: f32,
    pub easing: Functions,
    pub repeat: i32,
    pub coroutine_infos: Vec<CoroutineInfo>,
    pub time: f32,
}

pub fn start_event_coroutine(
    event_data: EventData,
    bpm: f32,
    current_time: f32,
    context: &BaseProviderContext,
) {
    let duration = (60.0 * event_data.duration) / bpm;
    let easing = event_data.easing;
    let repeat = event_data.repeat;

    let no_duration =
        duration == 0.0 || event_data.time + (duration * (repeat as f32 + 1.0)) < current_time;

    for info in &event_data.coroutine_infos {
        if info.property.get_point_definition().is_none() {
            info.track.mark_updated();
            info.property.set_null();
            continue;
        }

        let has_base = info
            .property
            .get_point_definition()
            .is_some_and(|t| t.has_base_provider());

        let property = &info.property;

        let finished = match &mut info.property {
            CoroutineProperty::ValueProperty(property, point_data) => {
                if no_duration || (point_data.get_points().len() <= 1 && !has_base) {
                    set_property_value(point_data, property, &info.track, 1.0, context);
                    continue;
                }

                animate_track(
                    point_data,
                    property,
                    &info.track,
                    duration,
                    event_data.time,
                    easing,
                    repeat,
                    has_base,
                    context,
                );
            }
            CoroutineProperty::PathProperty(path_property, point_data) => {
                path_property.init(point_data);

                if no_duration {
                    path_property.finish();
                    continue;
                }
                assign_path_animation(path_property, duration, event_data.time, easing, context);
            }
        };
    }
}

async fn animate_track(
    points: &mut BasePointDefinition,
    property: &mut ValueProperty,
    track: &Track,
    duration: f32,
    start_time: f32,
    easing: Functions,
    mut repeat: i32,
    non_lazy: bool,
    context: &BaseProviderContext,
) {
    let mut skip = false;

    while repeat >= 0 {
        let elapsed_time = context.get_current_time() - start_time;

        if !skip {
            let normalized_time = (elapsed_time / duration).min(1.0);
            let time = easing.interpolate(normalized_time);
            let on_last = set_property_value(points, property, &track, time, context);
            skip = !non_lazy && on_last;
        }

        if elapsed_time < duration {
            if repeat <= 0 && skip {
                break;
            }
            // yield to caller
            tokio::time::sleep(Duration::from_millis(16)).await;
        } else {
            repeat -= 1;
            start_time += duration;
            skip = false;
        }
    }
}

fn set_property_value(
    points: &mut BasePointDefinition,
    property: &mut ValueProperty,
    track: &Track,
    time: f32,
    context: &BaseProviderContext,
) -> bool {
    let (value, on_last) = points.interpolate(time, context);

    if value == property.get_value() {
        return on_last;
    }

    property.update_value(value);
    track.mark_updated();
    return on_last;
}

async fn assign_path_animation(
    interpolation: &mut PathProperty,
    duration: f32,
    start_time: f32,
    easing: Functions,
    context: &BaseProviderContext,
) {
    loop {
        let elapsed_time = context.get_current_time() - start_time;
        let normalized_time = (elapsed_time / duration).min(1.0);
        interpolation.time = easing.interpolate(normalized_time);

        if elapsed_time >= duration {
            break;
        }

        tokio::time::sleep(Duration::from_millis(16)).await;
    }

    interpolation.finish();
}
