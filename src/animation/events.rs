use crate::easings::functions::Functions;
use crate::point_definition::BasePointDefinitionGlobal;

use super::property::{PathPropertyGlobal, ValuePropertyGlobal};
use super::tracks::TrackGlobal;

pub struct EventData {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub property: EventType,
    pub track: TrackGlobal,
}

pub enum EventType {
    AnimateTrack(ValuePropertyGlobal, BasePointDefinitionGlobal),
    AssignPathAnimation(PathPropertyGlobal, Option<BasePointDefinitionGlobal>),
}
