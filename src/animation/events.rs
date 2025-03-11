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

pub fn start_event_coroutine(event_data: EventData, bpm: f32, current_time: f32) {
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
                if no_duration
                    || (info.point_definition.point_count() <= 1
                        && !info.point_definition.has_base_provider())
                {
                    set_property_value(&info.point_definition, &info.property, &info.track, 1.0);
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
                )
            }
            BaseProperty::PathProperty(path_property) => {
                let interpolation = path_property.get_interpolation();
                interpolation.init(&info.point_definition);

                if no_duration {
                    interpolation.finish();
                    continue;
                }
                assign_path_animation(interpolation, duration, event_data.time, easing);
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
        let elapsed_time = get_current_time() - start_time;

        if !skip {
            let normalized_time = (elapsed_time / duration).min(1.0);
            let time = interpolate(normalized_time, &easing);
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

async fn assign_path_animation(
    interpolation: BasePointDefinition,
    duration: f32,
    start_time: f32,
    easing: Functions,
    context: &BaseProviderContext,
) {
    loop {
        let elapsed_time = get_current_time() - start_time;
        let normalized_time = (elapsed_time / duration).min(1.0);
        interpolation.set_time(interpolate(normalized_time, &easing));

        if elapsed_time >= duration {
            break;
        }

        tokio::time::sleep(Duration::from_millis(16)).await;
    }

    interpolation.finish();
}
