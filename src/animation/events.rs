use itertools::Itertools;

use crate::easings::functions::Functions;
use crate::point_definition::{BasePointDefinition, BasePointDefinitionGlobal, PointDefinition};
use crate::values::base_provider_context::BaseProviderContext;

use super::property::{PathProperty, PathPropertyGlobal, ValuePropertyGlobal};
use super::tracks::{Track, TrackGlobal};

use std::collections::LinkedList;
use std::rc::Rc;
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
    pub track: TrackGlobal,
}

pub enum EventType {
    AnimateTrack(ValuePropertyGlobal, Rc<BasePointDefinitionGlobal>),
    AssignPathAnimation(PathPropertyGlobal, Option<BasePointDefinitionGlobal>),
}
