use crate::{
    animation::{
        track::{PathPropertyHandle, ValuePropertyHandle},
        tracks_holder::TrackKey,
    },
    easings::functions::Functions,
    point_definition::base_point_definition::{self},
    time_types::{BpmTime, SongTime},
};

/// Event type name (`"t"`) of an `AnimateTrack` event.
pub const ANIMATE_TRACK: &str = "AnimateTrack";
/// Event type name (`"t"`) of an `AssignPathAnimation` event.
pub const ASSIGN_PATH_ANIMATION: &str = "AssignPathAnimation";

/// Event field: time in beats.
pub const EVENT_TIME: &str = "b";
/// Event field: event type name.
pub const EVENT_TYPE: &str = "t";
/// Event field: object holding the event's data.
pub const EVENT_DATA: &str = "d";

/// Event data field: track name or array of track names.
pub const DATA_TRACK: &str = "track";
/// Event data field: duration in beats (default 0).
pub const DATA_DURATION: &str = "duration";
/// Event data field: easing name (default `easeLinear`).
pub const DATA_EASING: &str = "easing";
/// Event data field: number of times the event repeats (default 0).
pub const DATA_REPEAT: &str = "repeat";

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

#[cfg(feature = "json")]
mod json {
    use std::str::FromStr;

    use ::serde::{Deserialize, de::Error as DeError};
    use ahash::HashMap;
    use serde_json::{Map, Value as JsonValue};
    use smallvec::SmallVec;

    use super::{ANIMATE_TRACK, ASSIGN_PATH_ANIMATION, EventData, EventType};
    use crate::{
        animation::{
            track::{PathPropertyHandle, ValuePropertyHandle},
            tracks_holder::TracksHolder,
        },
        base_provider_context::BaseProviderContext,
        base_value::WrapBaseValueType,
        easings::functions::Functions,
        point_definition::base_point_definition::BasePointDefinition,
        time_types::BpmTime,
    };

    fn err<T: std::fmt::Display>(msg: T) -> serde_json::Error {
        serde_json::Error::custom(msg)
    }

    /// Wire format of an event. Field names mirror [`super::EVENT_TIME`], [`super::EVENT_TYPE`] and [`super::EVENT_DATA`].
    #[derive(::serde::Deserialize)]
    struct RawEvent {
        /// Time in beats.
        b: BpmTime,
        /// Event type name, e.g. [`super::ANIMATE_TRACK`].
        t: String,
        /// Event data.
        d: RawEventData,
    }

    /// Wire format of an event's `d` object.
    #[derive(::serde::Deserialize)]
    struct RawEventData {
        /// [`super::DATA_TRACK`]: a track name or an array of names.
        track: RawTracks,
        /// [`super::DATA_DURATION`]: length in beats, defaults to 0.
        duration: Option<BpmTime>,
        /// [`super::DATA_EASING`]: easing function name, defaults to `easeLinear`.
        easing: Option<String>,
        /// [`super::DATA_REPEAT`]: how many extra times the event repeats, defaults to 0.
        repeat: Option<u32>,
        /// Every other key is a property name (e.g. `offsetPosition`) whose value
        /// is the point definition (inline array, named definition or `null`).
        #[serde(flatten)]
        properties: Map<String, JsonValue>,
    }

    #[derive(::serde::Deserialize)]
    #[serde(untagged)]
    enum RawTracks {
        One(String),
        Many(SmallVec<[String; 2]>),
    }

    impl RawTracks {
        fn into_vec(self) -> SmallVec<[String; 2]> {
            match self {
                RawTracks::One(name) => smallvec::smallvec![name],
                RawTracks::Many(names) => names.into(),
            }
        }
    }

    impl EventData {
        /// Deserialize a V3 Heck event (`{"b": .., "t": "AnimateTrack" | "AssignPathAnimation", "d": {..}}`).
        ///
        /// Takes one event object (not an array of events). The event may target several
        /// tracks and animate several properties, so one [`EventData`] is produced per
        /// (track, property) pair.
        ///
        /// * `bpm` converts the event's beat time `b` to song time.
        /// * `named_point_defs` resolves properties written as a point definition name.
        /// * Every referenced track must already exist in `tracks_holder`.
        pub fn deserialize(
            event: &JsonValue,
            bpm: f32,
            tracks_holder: &mut TracksHolder,
            context: &mut BaseProviderContext,
            named_point_defs: &HashMap<String, JsonValue>,
        ) -> Result<Vec<EventData>, serde_json::Error> {
            let RawEvent {
                b: bpm_time,
                t: e_type,
                d: data,
            } = RawEvent::deserialize(event)?;

            let is_path = match e_type.as_str() {
                ANIMATE_TRACK => false,
                ASSIGN_PATH_ANIMATION => true,
                other => return Err(err(format!("unsupported event type '{other}'"))),
            };

            let track_names = data.track.into_vec();
            let raw_duration = data.duration.unwrap_or(BpmTime::ZERO);
            let repeat = data.repeat.unwrap_or(0);
            let easing = match data.easing.as_deref() {
                None => Functions::EaseLinear,
                Some(name) => Functions::from_str(name)
                    .map_err(|()| err(format!("unknown easing '{name}'")))?,
            };

            let start_song_time = bpm_time.to_song_time(f64::from(bpm));

            // Every remaining key of "d" is a property to animate.
            let properties = data.properties;
            if properties.is_empty() {
                return Err(err("event has no property to animate"));
            }

            // One EventData per (track, property) pair.
            let mut events = Vec::with_capacity(track_names.len() * properties.len());
            for track_name in &track_names {
                let track_key = tracks_holder
                    .get_track_key(track_name)
                    .ok_or_else(|| err(format!("track '{track_name}' does not exist")))?;
                let track = tracks_holder
                    .get_track(track_key)
                    .ok_or_else(|| err(format!("track '{track_name}' does not exist")))?;

                for (name, value) in &properties {
                    // The type is dictated by the property if the track already has it.
                    let existing_type = if is_path {
                        track.path_properties.get(name).map(|p| p.get_type())
                    } else {
                        track.properties.get(name).map(|p| p.get_type())
                    }
                    .filter(|ty| *ty != WrapBaseValueType::Unknown);

                    // `null` erases the value; anything else needs the property's registered type.
                    let point_data = match value {
                        JsonValue::Null => None,
                        value @ _ => {
                            let ty = existing_type.ok_or_else(|| {
                                err(format!(
                                    "property '{name}' is not registered on track '{track_name}'"
                                ))
                            })?;
                            let point_definition = BasePointDefinition::deserialize(
                                value.clone(),
                                ty,
                                context,
                                named_point_defs,
                            )?;
                            Some(point_definition)
                        }
                    };

                    let property = if is_path {
                        EventType::AssignPathAnimation(PathPropertyHandle::new(name))
                    } else {
                        EventType::AnimateTrack(ValuePropertyHandle::new(name))
                    };

                    events.push(EventData {
                        raw_duration,
                        easing,
                        repeat,
                        start_song_time,
                        property,
                        track_key,
                        point_data,
                    });
                }
            }

            Ok(events)
        }
    }
}

#[cfg(all(test, feature = "json"))]
mod json_tests {
    use ahash::HashMap;
    use serde_json::json;

    use super::*;
    use crate::{
        animation::{track::Track, tracks_holder::TracksHolder},
        base_provider_context::BaseProviderContext,
        base_value::WrapBaseValueType,
        point_definition::PointDefinitionLike,
    };

    fn holder(names: &[&str]) -> TracksHolder {
        let mut holder = TracksHolder::new();
        for name in names {
            let mut track = Track::default();
            track.name = (*name).to_owned();
            holder.add_track(track);
        }
        holder
    }

    fn deserialize(
        event: serde_json::Value,
        holder: &mut TracksHolder,
        named: &HashMap<String, serde_json::Value>,
    ) -> Result<Vec<EventData>, serde_json::Error> {
        let mut ctx = BaseProviderContext::new();
        EventData::deserialize(&event, 60.0, holder, &mut ctx, named)
    }

    /// Heck docs: AnimateTrack zig-zagging `offsetPosition` over 10 beats.
    #[test]
    fn animate_track_example() {
        let mut holder = holder(&["ZigZagTrack"]);
        let events = deserialize(
            json!({
                "b": 3,
                "t": "AnimateTrack",
                "d": {
                    "track": "ZigZagTrack",
                    "duration": 10,
                    "easing": "easeInOutQuad",
                    "offsetPosition": [[0, 0, 0, 0], [5, 0, 0, 0.25], [-5, 0, 0, 0.75], [0, 0, 0, 1]]
                }
            }),
            &mut holder,
            &HashMap::default(),
        )
        .unwrap();

        assert_eq!(events.len(), 1);
        let ev = &events[0];
        assert_eq!(ev.raw_duration, BpmTime::new(10.0));
        assert_eq!(ev.easing, Functions::EaseInOutQuad);
        assert_eq!(ev.repeat, 0);
        assert_eq!(ev.start_song_time, SongTime::new(3.0)); // 60 bpm
        assert_eq!(ev.track_key, holder.get_track_key("ZigZagTrack").unwrap());
        assert_eq!(
            ev.property,
            EventType::AnimateTrack(ValuePropertyHandle::new("offsetPosition"))
        );
        let point = ev.point_data.as_ref().unwrap();
        assert_eq!(point.get_type(), WrapBaseValueType::Vec3);
    }

    /// Heck docs: AssignPathAnimation dropping notes from 40 units up.
    #[test]
    fn assign_path_animation_example() {
        let mut holder = holder(&["DropNotes"]);
        let events = deserialize(
            json!({
                "b": 0,
                "t": "AssignPathAnimation",
                "d": {
                    "track": "DropNotes",
                    "offsetPosition": [[0, 40, 0, 0], [0, 0, 0, 0.2]]
                }
            }),
            &mut holder,
            &HashMap::default(),
        )
        .unwrap();

        assert_eq!(events.len(), 1);
        let ev = &events[0];
        assert_eq!(ev.raw_duration, BpmTime::ZERO);
        assert_eq!(ev.easing, Functions::EaseLinear);
        assert_eq!(
            ev.property,
            EventType::AssignPathAnimation(PathPropertyHandle::new("offsetPosition"))
        );
        assert!(ev.point_data.is_some());
    }

    #[test]
    fn multiple_tracks_and_properties_fan_out() {
        let mut holder = holder(&["A", "B"]);
        let events = deserialize(
            json!({
                "b": 1, "t": "AnimateTrack",
                "d": {
                    "track": ["A", "B"],
                    "dissolve": [[0, 0], [1, 1]],
                    "scale": [[1, 1, 1, 0]]
                }
            }),
            &mut holder,
            &HashMap::default(),
        )
        .unwrap();

        assert_eq!(events.len(), 4);
        for track in ["A", "B"] {
            let key = holder.get_track_key(track).unwrap();
            for prop in ["dissolve", "scale"] {
                assert!(events.iter().any(|e| e.track_key == key
                    && e.property == EventType::AnimateTrack(ValuePropertyHandle::new(prop))));
            }
        }
    }

    #[test]
    fn repeat_and_color_type() {
        let mut holder = holder(&["A"]);
        let events = deserialize(
            json!({
                "b": 1, "t": "AnimateTrack",
                "d": { "track": "A", "repeat": 2, "color": [[1, 0, 0, 1, 0], [0, 1, 0, 1, 1]] }
            }),
            &mut holder,
            &HashMap::default(),
        )
        .unwrap();

        let ev = &events[0];
        assert_eq!(ev.repeat, 2);
        assert_eq!(
            ev.point_data.as_ref().unwrap().get_type(),
            WrapBaseValueType::Vec4
        );
    }

    #[test]
    fn null_property_erases() {
        let mut holder = holder(&["T"]);
        let events = deserialize(
            json!({"b": 0, "t": "AnimateTrack", "d": {"track": "T", "dissolve": null}}),
            &mut holder,
            &HashMap::default(),
        )
        .unwrap();
        assert!(events[0].point_data.is_none());
    }

    #[test]
    fn custom_property_requires_registration() {
        use crate::animation::property::ValueProperty;

        let mut holder = holder(&["T"]);
        let event = json!({"b": 0, "t": "AnimateTrack", "d": {"track": "T", "custom": [[0, 1, 2, 0], [1, 2, 3, 1]]}});
        let named = HashMap::default();

        assert!(deserialize(event.clone(), &mut holder, &named).is_err());

        let key = holder.get_track_key("T").unwrap();
        holder.get_track_mut(key).unwrap().register_property(
            "custom".to_owned(),
            ValueProperty::new(None, WrapBaseValueType::Vec3),
        );
        let events = deserialize(event, &mut holder, &named).unwrap();
        assert_eq!(
            events[0].point_data.as_ref().unwrap().get_type(),
            WrapBaseValueType::Vec3
        );
    }

    #[test]
    fn named_definition_resolves() {
        let mut holder = holder(&["T"]);
        let mut named = HashMap::default();
        named.insert(
            "ZigZagPosition".to_owned(),
            json!([[0, 0, 0, 0], [1, 0, 0, 1]]),
        );
        let events = deserialize(
            json!({"b": 0, "t": "AnimateTrack", "d": {"track": "T", "offsetPosition": "ZigZagPosition"}}),
            &mut holder,
            &named,
        )
        .unwrap();
        assert!(events[0].point_data.is_some());
    }

    #[test]
    fn invalid_events_error() {
        let mut holder = holder(&["T"]);
        let named = HashMap::default();
        let mut run = |ev: serde_json::Value| deserialize(ev, &mut holder, &named).is_err();

        // missing track in the holder
        assert!(run(
            json!({"b": 0, "t": "AnimateTrack", "d": {"track": "Nope", "dissolve": [[0,0]]}})
        ));
        // unsupported event type
        assert!(run(
            json!({"b": 0, "t": "AnimateComponent", "d": {"track": "T"}})
        ));
        // missing fields
        assert!(run(json!({"t": "AnimateTrack", "d": {"track": "T"}})));
        assert!(run(json!({"b": 0, "t": "AnimateTrack"})));
        assert!(run(json!({"b": 0, "t": "AnimateTrack", "d": {}})));
        // bad easing / unknown named definition
        assert!(run(
            json!({"b": 0, "t": "AnimateTrack", "d": {"track": "T", "easing": "nope"}})
        ));
        assert!(run(
            json!({"b": 0, "t": "AnimateTrack", "d": {"track": "T", "dissolve": "Missing"}})
        ));
        // an array of events is not a single event block
        assert!(run(
            json!([{"b": 0, "t": "AnimateTrack", "d": {"track": "T", "dissolve": [[0,0]]}}])
        ));
        assert!(run(json!(42)));
    }
}
