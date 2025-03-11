use crate::easings::functions::Functions;
use crate::point_definition::{BasePointDefinition, PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;

use super::property::{BaseProperty, PathProperty};
use super::{property::Property, tracks::Track};
use std::sync::Arc;
use std::time::Duration;

pub struct CoroutineInfo {
    pub point_definition: Option<BasePointDefinition>,
    pub property: BaseProperty,
    pub track: Track,
}

pub enum EventType {
    AnimateTrack(Property),
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

    for info in event_data.coroutine_infos {
        if info.point_definition.is_none() {
            info.track.mark_updated();
            info.property.set_null();
            continue;
        }

        match info.property {
            BaseProperty::Property(property) => {
                if let Some(point_def) = &info.point_definition
                    && (no_duration
                        || (point_def.get_points().len() <= 1 && !point_def.has_base_provider()))
                {
                    set_property_value(
                        &point_def,
                        &property,
                        &info.track,
                        1.0,
                        context,
                    );
                    continue;
                }

                animate_track(
                    info.point_definition,
                    info.property,
                    info.track,
                    duration,
                    event_data.time,
                    easing,
                    repeat,
                    info.point_definition.has_base_provider(),
                );
            }
            BaseProperty::PathProperty(path_property) => {
                path_property.init(info.point_definition);

                if no_duration {
                    path_property.finish();
                    continue;
                }
                assign_path_animation(path_property, duration, event_data.time, easing, context);
            }
        }
    }
}

async fn animate_track(
    points: BasePointDefinition,
    property: Property,
    track: Track,
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
            let on_last = set_property_value(&points, &property, &track, time);
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
    points: &BasePointDefinition,
    property: &Property,
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
