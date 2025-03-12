use itertools::Itertools;

use crate::easings::functions::Functions;
use crate::point_definition::{BasePointDefinition, PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;

use super::property::PathProperty;
use super::{property::ValueProperty, tracks::Track};
use std::collections::LinkedList;
use std::sync::Arc;
use std::time::Duration;
use std::vec;

pub struct EventData {
    pub raw_duration: f32,
    pub easing: Functions,
    pub repeat: u32,
    // song time or beatmap time?
    pub start_time: f32,

    pub property: EventType,
    pub track: Track,
}

pub enum EventType {
    AnimateTrack(ValueProperty, BasePointDefinition),
    AssignPathAnimation(PathProperty, Option<BasePointDefinition>),
}
