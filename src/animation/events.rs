use crate::{
    easings::functions::Functions,
    point_definition::base_point_definition::{self},
};

use super::{
    property::{PathProperty, ValueProperty},
    tracks::Track,
};

pub struct EventData<'a> {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub property: EventType<'a>,
    pub track: &'a mut Track<'a>,
    pub point_data: Option<&'a base_point_definition::BasePointDefinition>,
}

#[derive(Debug)]
pub enum EventType<'a> {
    AnimateTrack(&'a mut ValueProperty),
    AssignPathAnimation(&'a mut PathProperty<'a>),
}

impl PartialEq for EventType<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EventType::AnimateTrack(a), EventType::AnimateTrack(b)) => std::ptr::eq(a, b),
            (EventType::AssignPathAnimation(a), EventType::AssignPathAnimation(b)) => {
                std::ptr::eq(a, b)
            }
            _ => false,
        }
    }
}
