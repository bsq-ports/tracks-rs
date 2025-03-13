use crate::{easings::functions::Functions, point_definition::BasePointDefinition};

use super::{property::{PathProperty, ValueProperty}, tracks::Track};

pub struct EventData<'a> {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub property: EventType<'a>,
    pub track: &'a mut Track<'a>,
    pub point_data: Option<&'a BasePointDefinition>,
}

pub enum EventType<'a> {
    AnimateTrack(ValueProperty),
    AssignPathAnimation(PathProperty<'a>),
}
