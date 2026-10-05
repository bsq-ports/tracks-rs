use crate::{
    animation::{
        track::{PathPropertyHandle, ValuePropertyHandle},
        tracks_holder::TrackKey,
    },
    easings::functions::Functions,
    point_definition::base_point_definition::{self},
    time_types::{BpmTime, SongTime},
};

#[derive(Debug, Clone)]
pub struct EventData {
    /// Duration in beats, as written in the beatmap. Converted to seconds with the BPM when the event starts.
    pub raw_duration: BpmTime,
    pub easing: Functions,
    pub repeat: u32,
    /// When the event starts on the song clock.
    pub start_song_time: SongTime,

    pub property: EventType,
    pub track_key: TrackKey,
    pub point_data: Option<base_point_definition::BasePointDefinition>,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum EventType {
    AnimateTrack(ValuePropertyHandle),
    AssignPathAnimation(PathPropertyHandle),
}
