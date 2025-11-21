use crate::{
    animation::{
        track::{PathPropertyHandle, ValuePropertyHandle},
        tracks_holder::TrackKey,
    },
    easings::functions::Functions,
    point_definition::base_point_definition::{self},
};

pub struct EventData {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub property: EventType,
    pub track_key: TrackKey,
    pub point_data: Option<base_point_definition::BasePointDefinition>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EventType {
    AnimateTrack(ValuePropertyHandle),
    AssignPathAnimation(PathPropertyHandle),
}
