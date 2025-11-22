use crate::{
    animation::{
        track::{PathPropertyHandle, ValuePropertyHandle},
        tracks_holder::TrackKey,
    },
    easings::functions::Functions,
    point_definition::base_point_definition::{self},
};

#[derive(Debug, Clone)]
pub struct EventData {
    /// duration in beatmap time
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    /// song adjusted time
    pub start_song_time: f32,

    pub property: EventType,
    pub track_key: TrackKey,
    pub point_data: Option<base_point_definition::BasePointDefinition>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum EventType {
    AnimateTrack(ValuePropertyHandle),
    AssignPathAnimation(PathPropertyHandle),
}
