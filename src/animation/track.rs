use std::{fmt::Display, rc::Rc, str::FromStr};

use crate::{
    animation::property::{PathProperty, ValueProperty},
    base_value::WrapBaseValueType,
};

use super::game_object::GameObject;

// Define constants for property names
pub const POSITION: &str = "position";
pub const OFFSET_POSITION: &str = "offsetPosition";
pub const ROTATION: &str = "rotation";
pub const OFFSET_ROTATION: &str = "offsetWorldRotation";
pub const SCALE: &str = "scale";
pub const LOCAL_ROTATION: &str = "localRotation";
pub const LOCAL_POSITION: &str = "localPosition";
pub const DEFINITE_POSITION: &str = "definitePosition";
pub const DISSOLVE: &str = "dissolve";
pub const DISSOLVE_ARROW: &str = "dissolveArrow";
pub const TIME: &str = "time";

// v3
pub const INTERACTABLE: &str = "interactable";

pub const COLOR: &str = "color";
pub const ATTENTUATION: &str = "attentuation";
pub const FOG_OFFSET: &str = "fogOffset";
pub const HEIGHT_FOG_START_Y: &str = "heightFogStartY";
pub const HEIGHT_FOG_HEIGHT: &str = "heightFogHeight";

// v2 keys
pub const V2_POSITION: &str = "_position";
pub const V2_LOCAL_POSITION: &str = "_localPosition";
pub const V2_ROTATION: &str = "_rotation";
pub const V2_LOCAL_ROTATION: &str = "_localRotation";
pub const V2_SCALE: &str = "_scale";
pub const V2_DEFINITE_POSITION: &str = "_definitePosition";
pub const V2_DISSOLVE: &str = "_dissolve";
pub const V2_DISSOLVE_ARROW: &str = "_dissolveArrow";
pub const V2_TIME: &str = "_time";
pub const V2_CUTTABLE: &str = "_cuttable";
pub const V2_COLOR: &str = "_color";
pub const V2_ATTENTUATION: &str = "_attenuation";
pub const V2_FOG_OFFSET: &str = "_fogOffset";
pub const V2_HEIGHT_FOG_START_Y: &str = "_heightFogStartY";
pub const V2_HEIGHT_FOG_HEIGHT: &str = "_heightFogHeight";

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum ValuePropertyHandle {
    ByName(String),
    ById(PropertyNames),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum PathPropertyHandle {
    ByName(String),
    ById(PropertyNames),
}

impl ValuePropertyHandle {
    pub fn new(id: &str) -> Self {
        match PropertyNames::from_str(id) {
            Ok(name) => ValuePropertyHandle::ById(name),
            _ => ValuePropertyHandle::ByName(id.to_string()),
        }
    }
}

impl PathPropertyHandle {
    pub fn new(id: &str) -> Self {
        match PropertyNames::from_str(id) {
            Ok(name) => PathPropertyHandle::ById(name),
            _ => PathPropertyHandle::ByName(id.to_string()),
        }
    }
}

/// An enumeration of common property names used in Tracks.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PropertyNames {
    Position,
    // v2 aliased to position
    OffsetPosition,
    Rotation,
    OffsetRotation,
    Scale,
    LocalRotation,
    LocalPosition,
    DefinitePosition,
    Dissolve,
    DissolveArrow,
    Time,
    // in v2 -> _interactable/V2_CUTTABLE
    // in v3 -> interactable/INTERACTABLE
    Cuttable,
    Color,
    Attentuation,
    FogOffset,
    HeightFogStartY,
    HeightFogHeight,
    UnknownPropertyName,
}

/// A PathPropertiesMap holds a collection of PathProperties associated with a Track.
/// Fast access to common path properties is provided via dedicated fields.
#[derive(Clone)]
pub struct PathPropertiesMap {
    pub path_properties: ahash::AHashMap<String, PathProperty>,

    pub position: PathProperty,
    pub offset_position: PathProperty,
    pub rotation: PathProperty,
    pub offset_rotation: PathProperty,
    pub scale: PathProperty,
    pub local_rotation: PathProperty,
    pub local_position: PathProperty,
    pub definite_position: PathProperty,
    pub dissolve: PathProperty,
    pub dissolve_arrow: PathProperty,
    pub cuttable: PathProperty,
    pub color: PathProperty,
}

/// A PropertiesMap holds a collection of ValueProperties associated with a Track.
/// Fast access to common properties is provided via dedicated fields.
#[derive(Clone)]
pub struct PropertiesMap {
    pub properties: ahash::AHashMap<String, ValueProperty>,

    // hard defined properties

    // Noodle
    pub position: ValueProperty,
    pub offset_position: ValueProperty,
    pub rotation: ValueProperty,
    pub offset_rotation: ValueProperty,
    pub scale: ValueProperty,
    pub local_rotation: ValueProperty,
    pub local_position: ValueProperty,
    pub dissolve: ValueProperty,
    pub dissolve_arrow: ValueProperty,
    pub time: ValueProperty,
    pub cuttable: ValueProperty,

    // Chroma
    pub color: ValueProperty,
    pub attentuation: ValueProperty,       // PropertyType::linear
    pub fog_offset: ValueProperty,         // PropertyType::linear
    pub height_fog_start_y: ValueProperty, // PropertyType::linear
    pub height_fog_height: ValueProperty,  // PropertyType::linear
}

/// A GameObjectCallback is a function that gets called when a game object is added or removed from a Track.
/// bool is true if added, false if removed.
pub trait GameObjectCallback: Fn(GameObject, bool) {}
// TODO: Use trait aliases when they are stabilized in Rust. For now, we can implement the trait for any function that matches the signature.

impl<F: Fn(GameObject, bool)> GameObjectCallback for F {}

/// A Track represents a collection of properties and path properties associated with game objects.
/// It allows registering, retrieving, and managing properties and game objects.
#[derive(Clone, Default)]
pub struct Track {
    pub properties: PropertiesMap,
    pub path_properties: PathPropertiesMap,

    pub name: String,

    // hashset but must be insertion ordered
    game_objects: Vec<GameObject>,
    game_object_callbacks: Vec<Rc<dyn GameObjectCallback>>,
}

impl Track {
    pub fn register_property(&mut self, id: String, property: ValueProperty) {
        self.properties.insert(id, property);
    }

    pub fn register_path_property(&mut self, id: String, property: PathProperty) {
        self.path_properties.insert(id, property);
    }

    pub fn register_game_object(&mut self, game_object: GameObject) {
        if !self.game_objects.contains(&game_object) {
            self.game_objects.push(game_object);
        }

        self.game_object_callbacks
            .iter()
            .for_each(|callback| callback(game_object, true));
    }

    pub fn register_game_object_callback(&mut self, callback: Rc<dyn GameObjectCallback>) {
        self.game_object_callbacks.push(callback);
    }

    pub fn get_game_objects(&self) -> &Vec<GameObject> {
        &self.game_objects
    }

    pub fn remove_game_object(&mut self, game_object: &GameObject) {
        if let Some(position) = self.game_objects.iter().position(|go| go == game_object) {
            self.game_objects.remove(position);
        };

        self.game_object_callbacks
            .iter()
            .for_each(|callback| callback(*game_object, false));
    }

    pub fn remove_game_object_callback(&mut self, callback: Rc<dyn GameObjectCallback>) {
        let callback_ref: Rc<dyn GameObjectCallback> = callback;
        self.game_object_callbacks
            .retain(|cb| !Rc::ptr_eq(cb, &callback_ref));
    }

    pub fn reset(&mut self) {
        self.properties = PropertiesMap::default();
        self.path_properties = PathPropertiesMap::default();
        self.game_objects.clear();
        self.game_object_callbacks.clear();
    }
}

impl Default for PropertiesMap {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            position: ValueProperty::empty(WrapBaseValueType::Vec3),
            offset_position: ValueProperty::empty(WrapBaseValueType::Vec3),
            rotation: ValueProperty::empty(WrapBaseValueType::Quat),
            offset_rotation: ValueProperty::empty(WrapBaseValueType::Quat),
            scale: ValueProperty::empty(WrapBaseValueType::Vec3),
            local_rotation: ValueProperty::empty(WrapBaseValueType::Quat),
            local_position: ValueProperty::empty(WrapBaseValueType::Vec3),
            dissolve: ValueProperty::empty(WrapBaseValueType::Float),
            dissolve_arrow: ValueProperty::empty(WrapBaseValueType::Float),
            time: ValueProperty::empty(WrapBaseValueType::Float),
            cuttable: ValueProperty::empty(WrapBaseValueType::Float),
            color: ValueProperty::empty(WrapBaseValueType::Vec4),
            attentuation: ValueProperty::empty(WrapBaseValueType::Float),

            fog_offset: ValueProperty::empty(WrapBaseValueType::Float),
            height_fog_start_y: ValueProperty::empty(WrapBaseValueType::Float),
            height_fog_height: ValueProperty::empty(WrapBaseValueType::Float),
        }
    }
}

impl Default for PathPropertiesMap {
    fn default() -> Self {
        Self {
            path_properties: Default::default(),
            position: PathProperty::empty(WrapBaseValueType::Vec3),
            offset_position: PathProperty::empty(WrapBaseValueType::Vec3),
            rotation: PathProperty::empty(WrapBaseValueType::Quat),
            offset_rotation: PathProperty::empty(WrapBaseValueType::Quat),
            scale: PathProperty::empty(WrapBaseValueType::Vec3),
            local_rotation: PathProperty::empty(WrapBaseValueType::Quat),
            local_position: PathProperty::empty(WrapBaseValueType::Vec3),
            definite_position: PathProperty::empty(WrapBaseValueType::Vec3),
            dissolve: PathProperty::empty(WrapBaseValueType::Float),
            dissolve_arrow: PathProperty::empty(WrapBaseValueType::Float),
            cuttable: PathProperty::empty(WrapBaseValueType::Float),
            color: PathProperty::empty(WrapBaseValueType::Vec4),
        }
    }
}

impl PropertiesMap {
    pub fn insert(&mut self, id: String, property: ValueProperty) {
        match self.get_mut(&id) {
            Some(prop) => *prop = property,
            None => {
                self.properties.insert(id, property);
            }
        }
    }

    pub fn get_property_by_name(&self, name: PropertyNames) -> Option<&ValueProperty> {
        match name {
            PropertyNames::Position => Some(&self.position),
            PropertyNames::OffsetPosition => Some(&self.offset_position),
            PropertyNames::Rotation => Some(&self.rotation),
            PropertyNames::OffsetRotation => Some(&self.offset_rotation),
            PropertyNames::Scale => Some(&self.scale),
            PropertyNames::LocalRotation => Some(&self.local_rotation),
            PropertyNames::LocalPosition => Some(&self.local_position),
            PropertyNames::Dissolve => Some(&self.dissolve),
            PropertyNames::DissolveArrow => Some(&self.dissolve_arrow),
            PropertyNames::Time => Some(&self.time),
            PropertyNames::Cuttable => Some(&self.cuttable),
            PropertyNames::Color => Some(&self.color),
            PropertyNames::Attentuation => Some(&self.attentuation),
            PropertyNames::FogOffset => Some(&self.fog_offset),
            PropertyNames::HeightFogStartY => Some(&self.height_fog_start_y),
            PropertyNames::HeightFogHeight => Some(&self.height_fog_height),
            _ => None,
        }
    }

    pub fn get_property_by_name_mut(&mut self, name: PropertyNames) -> Option<&mut ValueProperty> {
        match name {
            PropertyNames::Position => Some(&mut self.position),
            PropertyNames::OffsetPosition => Some(&mut self.offset_position),
            PropertyNames::Rotation => Some(&mut self.rotation),
            PropertyNames::OffsetRotation => Some(&mut self.offset_rotation),
            PropertyNames::Scale => Some(&mut self.scale),
            PropertyNames::LocalRotation => Some(&mut self.local_rotation),
            PropertyNames::LocalPosition => Some(&mut self.local_position),
            PropertyNames::Dissolve => Some(&mut self.dissolve),
            PropertyNames::DissolveArrow => Some(&mut self.dissolve_arrow),
            PropertyNames::Time => Some(&mut self.time),
            PropertyNames::Cuttable => Some(&mut self.cuttable),
            PropertyNames::Color => Some(&mut self.color),
            PropertyNames::Attentuation => Some(&mut self.attentuation),
            PropertyNames::FogOffset => Some(&mut self.fog_offset),
            PropertyNames::HeightFogStartY => Some(&mut self.height_fog_start_y),
            PropertyNames::HeightFogHeight => Some(&mut self.height_fog_height),
            _ => None,
        }
    }

    pub fn get(&self, id: &str) -> Option<&ValueProperty> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name(name),
            _ => self.properties.get(id),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ValueProperty> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name_mut(name),
            _ => self.properties.get_mut(id),
        }
    }

    // faster access via handle
    pub fn get_by_handle(&self, handle: &ValuePropertyHandle) -> Option<&ValueProperty> {
        match handle {
            ValuePropertyHandle::ByName(id) => self.properties.get(id),
            ValuePropertyHandle::ById(name) => self.get_property_by_name(*name),
        }
    }

    pub fn get_by_handle_mut(
        &mut self,
        handle: &ValuePropertyHandle,
    ) -> Option<&mut ValueProperty> {
        match handle {
            ValuePropertyHandle::ByName(id) => self.properties.get_mut(id),
            ValuePropertyHandle::ById(name) => self.get_property_by_name_mut(*name),
        }
    }
}

impl PathPropertiesMap {
    pub fn insert(&mut self, id: String, property: PathProperty) {
        match self.get_mut(&id) {
            Some(prop) => *prop = property,
            None => {
                self.path_properties.insert(id, property);
            }
        }
    }

    pub fn get_property_by_name(&self, name: PropertyNames) -> Option<&PathProperty> {
        match name {
            PropertyNames::Position => Some(&self.position),
            PropertyNames::OffsetPosition => Some(&self.offset_position),
            PropertyNames::Rotation => Some(&self.rotation),
            PropertyNames::OffsetRotation => Some(&self.offset_rotation),
            PropertyNames::Scale => Some(&self.scale),
            PropertyNames::LocalRotation => Some(&self.local_rotation),
            PropertyNames::LocalPosition => Some(&self.local_position),
            PropertyNames::DefinitePosition => Some(&self.definite_position),
            PropertyNames::Dissolve => Some(&self.dissolve),
            PropertyNames::DissolveArrow => Some(&self.dissolve_arrow),
            PropertyNames::Cuttable => Some(&self.cuttable),
            PropertyNames::Color => Some(&self.color),

            _ => None,
        }
    }

    pub fn get_property_by_name_mut(&mut self, name: PropertyNames) -> Option<&mut PathProperty> {
        match name {
            PropertyNames::Position => Some(&mut self.position),
            PropertyNames::OffsetPosition => Some(&mut self.offset_position),
            PropertyNames::Rotation => Some(&mut self.rotation),
            PropertyNames::OffsetRotation => Some(&mut self.offset_rotation),
            PropertyNames::Scale => Some(&mut self.scale),
            PropertyNames::LocalRotation => Some(&mut self.local_rotation),
            PropertyNames::LocalPosition => Some(&mut self.local_position),
            PropertyNames::DefinitePosition => Some(&mut self.definite_position),
            PropertyNames::Dissolve => Some(&mut self.dissolve),
            PropertyNames::DissolveArrow => Some(&mut self.dissolve_arrow),
            PropertyNames::Cuttable => Some(&mut self.cuttable),
            PropertyNames::Color => Some(&mut self.color),
            _ => None,
        }
    }

    pub fn get(&self, id: &str) -> Option<&PathProperty> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name(name),
            _ => self.path_properties.get(id),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut PathProperty> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name_mut(name),
            _ => self.path_properties.get_mut(id),
        }
    }

    // faster access via handle
    pub fn get_by_handle(&self, handle: &PathPropertyHandle) -> Option<&PathProperty> {
        match handle {
            PathPropertyHandle::ByName(id) => self.path_properties.get(id),
            PathPropertyHandle::ById(name) => self.get_property_by_name(*name),
        }
    }

    pub fn get_by_handle_mut(&mut self, handle: &PathPropertyHandle) -> Option<&mut PathProperty> {
        match handle {
            PathPropertyHandle::ByName(id) => self.path_properties.get_mut(id),
            PathPropertyHandle::ById(name) => self.get_property_by_name_mut(*name),
        }
    }
}

impl FromStr for PropertyNames {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            // v3
            POSITION => Ok(PropertyNames::Position),
            OFFSET_POSITION => Ok(PropertyNames::OffsetPosition),
            ROTATION => Ok(PropertyNames::Rotation),
            OFFSET_ROTATION => Ok(PropertyNames::OffsetRotation),
            SCALE => Ok(PropertyNames::Scale),
            LOCAL_ROTATION => Ok(PropertyNames::LocalRotation),
            LOCAL_POSITION => Ok(PropertyNames::LocalPosition),
            DEFINITE_POSITION => Ok(PropertyNames::DefinitePosition),
            DISSOLVE => Ok(PropertyNames::Dissolve),
            DISSOLVE_ARROW => Ok(PropertyNames::DissolveArrow),
            TIME => Ok(PropertyNames::Time),
            // in v3 -> interactable
            INTERACTABLE => Ok(PropertyNames::Cuttable),

            COLOR => Ok(PropertyNames::Color),
            ATTENTUATION => Ok(PropertyNames::Attentuation),
            FOG_OFFSET => Ok(PropertyNames::FogOffset),
            HEIGHT_FOG_START_Y => Ok(PropertyNames::HeightFogStartY),
            HEIGHT_FOG_HEIGHT => Ok(PropertyNames::HeightFogHeight),

            V2_POSITION => Ok(PropertyNames::Position),
            V2_LOCAL_POSITION => Ok(PropertyNames::LocalPosition),
            V2_ROTATION => Ok(PropertyNames::Rotation),
            V2_LOCAL_ROTATION => Ok(PropertyNames::LocalRotation),
            V2_SCALE => Ok(PropertyNames::Scale),
            V2_DEFINITE_POSITION => Ok(PropertyNames::DefinitePosition),
            V2_DISSOLVE => Ok(PropertyNames::Dissolve),
            V2_DISSOLVE_ARROW => Ok(PropertyNames::DissolveArrow),
            V2_TIME => Ok(PropertyNames::Time),
            V2_CUTTABLE => Ok(PropertyNames::Cuttable),
            V2_COLOR => Ok(PropertyNames::Color),
            V2_ATTENTUATION => Ok(PropertyNames::Attentuation),
            V2_FOG_OFFSET => Ok(PropertyNames::FogOffset),
            V2_HEIGHT_FOG_START_Y => Ok(PropertyNames::HeightFogStartY),
            V2_HEIGHT_FOG_HEIGHT => Ok(PropertyNames::HeightFogHeight),

            _ => Err(()),
        }
    }
}

impl Display for PropertyNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyNames::Position => write!(f, "{POSITION}"),
            PropertyNames::OffsetPosition => write!(f, "{OFFSET_POSITION}"),
            PropertyNames::Rotation => write!(f, "{ROTATION}"),
            PropertyNames::OffsetRotation => write!(f, "{OFFSET_ROTATION}"),
            PropertyNames::Scale => write!(f, "{SCALE}"),
            PropertyNames::LocalRotation => write!(f, "{LOCAL_ROTATION}"),
            PropertyNames::LocalPosition => write!(f, "{LOCAL_POSITION}"),
            PropertyNames::DefinitePosition => write!(f, "{DEFINITE_POSITION}"),
            PropertyNames::Dissolve => write!(f, "{DISSOLVE}"),
            PropertyNames::DissolveArrow => write!(f, "{DISSOLVE_ARROW}"),
            PropertyNames::Time => write!(f, "{TIME}"),
            PropertyNames::Cuttable => write!(f, "{V2_CUTTABLE}"),
            PropertyNames::Color => write!(f, "{COLOR}"),
            PropertyNames::Attentuation => write!(f, "{ATTENTUATION}"),
            PropertyNames::FogOffset => write!(f, "{FOG_OFFSET}"),
            PropertyNames::HeightFogStartY => write!(f, "{HEIGHT_FOG_START_Y}"),
            PropertyNames::HeightFogHeight => write!(f, "{HEIGHT_FOG_HEIGHT}"),
            PropertyNames::UnknownPropertyName => write!(f, "Unknown"),
        }
    }
}

#[cfg(feature = "json")]
mod serde {
    use std::str::FromStr;

    use ::serde::de::Error as DeError;
    use ahash::HashMap;
    use serde_json::Value as JsonValue;

    use crate::{
        animation::{
            property::PathProperty,
            track::{PathPropertiesMap, PropertyNames},
        },
        base_provider_context::BaseProviderContext,
        base_value::WrapBaseValueType,
        point_definition::{
            PointDefinitionLike, base_point_definition::BasePointDefinition,
            basic_point_definition::BasicPointDefinition,
            quaternion_point_definition::QuaternionPointDefinition,
            vector3_point_definition::Vector3PointDefinition,
        },
    };

    impl PathPropertiesMap {
        /// Deserialize path definitions while using the caller's provider context.
        pub fn deserialize(
            properties: JsonValue,
            context: &mut BaseProviderContext,
            named_point_defs: HashMap<String, BasePointDefinition>,
        ) -> Result<Self, serde_json::Error> {
            let JsonValue::Object(object) = properties else {
                return Err(serde_json::Error::custom(
                    "path properties must be a JSON object",
                ));
            };

            let mut result = PathPropertiesMap::default();
            for (name, value) in object {
                let property = match value {
                    JsonValue::Null => {
                        // skip null values
                        continue;
                    }
                    JsonValue::Array(arr) if arr.is_empty() => {
                        // skip empty arrays
                        continue;
                    }
                    JsonValue::String(s) if let Some(definition) = named_point_defs.get(&s) => {
                        let ty = definition.value_type();
                        PathProperty::new(Some(definition.clone()), ty)
                    }
                    JsonValue::String(s) => {
                        return Err(serde_json::Error::custom(format!(
                            "named point definition '{s}' not found"
                        )));
                    }
                    JsonValue::Array(array) => {
                        let ty = property_type(&name, &array).map_err(serde_json::Error::custom)?;
                        let definition = parse_definition(JsonValue::Array(array), ty, context)
                            .map_err(serde_json::Error::custom)?;
                        PathProperty::new(Some(definition), ty)
                    }
                    _ => {
                        return Err(serde_json::Error::custom(format!(
                            "path property '{name}' must be an array or a named point definition"
                        )));
                    }
                };

                result.insert(name, property);
            }

            Ok(result)
        }
    }

    fn property_type(name: &str, points: &[JsonValue]) -> Result<WrapBaseValueType, String> {
        let known_type = match PropertyNames::from_str(name) {
            Ok(
                PropertyNames::Position
                | PropertyNames::OffsetPosition
                | PropertyNames::Scale
                | PropertyNames::LocalPosition
                | PropertyNames::DefinitePosition,
            ) => Some(WrapBaseValueType::Vec3),
            Ok(
                PropertyNames::Rotation
                | PropertyNames::OffsetRotation
                | PropertyNames::LocalRotation,
            ) => Some(WrapBaseValueType::Quat),
            Ok(
                PropertyNames::Dissolve | PropertyNames::DissolveArrow | PropertyNames::Cuttable,
            ) => Some(WrapBaseValueType::Float),
            Ok(PropertyNames::Color) => Some(WrapBaseValueType::Vec4),
            _ => None,
        };
        known_type.map_or_else(|| infer_type(points), Ok)
    }

    fn infer_type(points: &[JsonValue]) -> Result<WrapBaseValueType, String> {
        let first = points
            .first()
            .ok_or_else(|| "path property cannot be an empty array".to_owned())?;
        let components = first
            .as_array()
            .map(Vec::as_slice)
            .unwrap_or(std::slice::from_ref(first));
        let numeric_count = components.iter().filter(|v| v.is_number()).count();

        match numeric_count {
            1 | 2 => Ok(WrapBaseValueType::Float),
            4 => Ok(WrapBaseValueType::Vec3),
            5 => Ok(WrapBaseValueType::Vec4),
            count => Err(format!(
                "cannot infer the value type for a custom path property with {count} numeric components"
            )),
        }
    }

    fn parse_definition(
        value: JsonValue,
        ty: WrapBaseValueType,
        context: &mut BaseProviderContext,
    ) -> Result<BasePointDefinition, String> {
        if value.as_array().is_none_or(Vec::is_empty) {
            return Err("path property cannot be an empty array".to_owned());
        }

        let definition = match ty {
            WrapBaseValueType::Float => BasicPointDefinition::<f32>::parse(value, context).into(),
            WrapBaseValueType::Vec3 => Vector3PointDefinition::parse(value, context).into(),
            WrapBaseValueType::Vec4 => {
                BasicPointDefinition::<glam::Vec4>::parse(value, context).into()
            }
            WrapBaseValueType::Quat => QuaternionPointDefinition::parse(value, context).into(),
            WrapBaseValueType::Unknown => {
                return Err("path property value type cannot be unknown".to_owned());
            }
        };
        Ok(definition)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    use crate::base_value::BaseValue;
    use glam::{Quat, Vec3, Vec4};

    #[cfg(feature = "json")]
    mod path_properties_json {
        use super::super::*;
        use crate::{
            base_provider_context::BaseProviderContext,
            base_value::WrapBaseValueType,
            point_definition::{
                PointDefinitionLike, base_point_definition::BasePointDefinition,
                vector3_point_definition::Vector3PointDefinition,
            },
        };
        use ahash::HashMap;
        use serde_json::json;

        fn parse(
            value: serde_json::Value,
            named: HashMap<String, BasePointDefinition>,
        ) -> Result<PathPropertiesMap, serde_json::Error> {
            let mut ctx = BaseProviderContext::new();
            PathPropertiesMap::deserialize(value, &mut ctx, named)
        }

        fn count(def: &Option<BasePointDefinition>) -> usize {
            match def.as_ref().expect("definition should be set") {
                BasePointDefinition::Float(d) => d.get_count(),
                BasePointDefinition::Vector3(d) => d.get_count(),
                BasePointDefinition::Vector4(d) => d.get_count(),
                BasePointDefinition::Quaternion(d) => d.get_count(),
            }
        }

        /// Heck docs: `"offsetPosition": [[0, 40, 0, 0], [0, 0, 0, 0.2]]`
        #[test]
        fn heck_offset_position_example() {
            let map = parse(
                json!({ "offsetPosition": [[0, 40, 0, 0], [0, 0, 0, 0.2]] }),
                HashMap::default(),
            )
            .unwrap();

            let prop = map.get("offsetPosition").unwrap();
            assert_eq!(prop.get_type(), WrapBaseValueType::Vec3);
            assert_eq!(count(&prop.point), 2);
            assert!(std::ptr::eq(prop, &map.offset_position));
        }

        #[test]
        fn known_property_types() {
            let map = parse(
                json!({
                    "rotation": [[0, 0, 0, 0], [10, 20, 30, 1]],
                    "dissolve": [[0, 0], [1, 1]],
                    "color": [[1, 0, 0, 1, 0], [0, 1, 0, 1, 1]],
                    "scale": [[1, 1, 1, 0]],
                }),
                HashMap::default(),
            )
            .unwrap();

            assert_eq!(map.rotation.get_type(), WrapBaseValueType::Quat);
            assert_eq!(map.dissolve.get_type(), WrapBaseValueType::Float);
            assert_eq!(map.color.get_type(), WrapBaseValueType::Vec4);
            assert_eq!(map.scale.get_type(), WrapBaseValueType::Vec3);
        }

        #[test]
        fn custom_property_type_is_inferred() {
            let map = parse(
                json!({
                    "customVec3": [[0, 1, 2, 0], [1, 2, 3, 1]],
                    "customFloat": [[0, 0], [1, 1]],
                }),
                HashMap::default(),
            )
            .unwrap();

            let v = map.get("customVec3").unwrap();
            assert_eq!(v.get_type(), WrapBaseValueType::Vec3);
            assert_eq!(count(&v.point), 2);
            assert_eq!(
                map.get("customFloat").unwrap().get_type(),
                WrapBaseValueType::Float
            );
        }

        #[test]
        fn null_and_empty_arrays_are_skipped() {
            let map = parse(
                json!({ "offsetPosition": null, "position": [], "customThing": null }),
                HashMap::default(),
            )
            .unwrap();

            assert!(map.offset_position.point.is_none());
            assert!(map.position.point.is_none());
            assert!(map.get("customThing").is_none());
        }

        #[test]
        fn named_point_definition_is_resolved() {
            let mut ctx = BaseProviderContext::new();
            let def: BasePointDefinition = Vector3PointDefinition::parse(
                json!([[0, 40, 0, 0], [0, 0, 0, 0.2]]),
                &mut ctx,
            )
            .into();
            let mut named = HashMap::default();
            named.insert("ZigZagPosition".to_owned(), def);

            let map = parse(json!({ "offsetPosition": "ZigZagPosition" }), named).unwrap();

            assert_eq!(map.offset_position.get_type(), WrapBaseValueType::Vec3);
            assert_eq!(count(&map.offset_position.point), 2);
        }

        #[test]
        fn unknown_named_point_definition_errors() {
            let res = parse(json!({ "offsetPosition": "Missing" }), HashMap::default());
            assert!(res.is_err());
        }

        #[test]
        fn invalid_inputs_error() {
            // not an object
            assert!(parse(json!([1, 2]), HashMap::default()).is_err());
            // property neither array nor string
            assert!(parse(json!({ "offsetPosition": 5 }), HashMap::default()).is_err());
            // custom property whose type can't be inferred
            assert!(parse(json!({ "custom": [[1, 2, 3]] }), HashMap::default()).is_err());
        }
    }

    #[test]
    fn test_null_values_return_none() {
        // default PropertiesMap initializes properties with None values
        let props = PropertiesMap::default();

        // ensure several default properties are None
        assert!(
            props.scale.get_value().is_none(),
            "scale should be None by default"
        );
        assert!(
            props.color.get_value().is_none(),
            "color should be None by default"
        );
        assert!(
            props.rotation.get_value().is_none(),
            "rotation should be None by default"
        );
        assert!(
            props.dissolve.get_value().is_none(),
            "dissolve should be None by default"
        );
    }

    #[test]
    fn test_set_and_get_float_vec3_vec4_quat() {
        let mut props = PropertiesMap::default();

        // Linear / float (dissolve)
        props.dissolve.set_value(Some(BaseValue::from(3.15_f32)));
        let f = props.dissolve.get_value().unwrap().as_float().unwrap();
        assert!((f - 3.15).abs() < 1e-6, "float value mismatch");

        // Vec3 (scale) - user requested one test must be scale
        let scale = Vec3::new(1.0, 2.0, 3.0);
        props.scale.set_value(Some(BaseValue::from(scale)));
        let got_scale = props.scale.get_value().unwrap().as_vec3().unwrap();
        assert_eq!(got_scale, scale, "scale Vec3 mismatch");

        // Vec4 (color)
        let color = Vec4::new(0.1, 0.2, 0.3, 0.4);
        props.color.set_value(Some(BaseValue::from(color)));
        let got_color = props.color.get_value().unwrap().as_vec4().unwrap();
        assert_eq!(got_color, color, "color Vec4 mismatch");

        // Quat (rotation)
        let quat = Quat::from_array([0.0, 0.0, 0.0, 1.0]);
        props.rotation.set_value(Some(BaseValue::from(quat)));
        let got_quat = props.rotation.get_value().unwrap().as_quat().unwrap();
        assert_eq!(got_quat, quat, "rotation Quat mismatch");
    }

    #[test]
    fn test_property_names_aliases_and_display() {
        // v1 -> enum -> display -> v1
        let pname = PropertyNames::from_str(POSITION).expect("POSITION should parse");
        assert_eq!(pname.to_string(), POSITION);

        // v2 alias -> enum -> display -> v1 canonical
        let pname_v2 = PropertyNames::from_str(V2_POSITION).expect("V2_POSITION should parse");
        assert_eq!(pname_v2.to_string(), POSITION);

        // another example: color
        let pcolor = PropertyNames::from_str(COLOR).expect("COLOR should parse");
        assert_eq!(pcolor.to_string(), COLOR);
        let pcolor_v2 = PropertyNames::from_str(V2_COLOR).expect("V2_COLOR should parse");
        assert_eq!(pcolor_v2.to_string(), COLOR);
    }

    #[test]
    fn test_insert_and_get_custom_property() {
        let mut props = PropertiesMap::default();

        // create a custom float property and insert it under a custom id
        let mut custom_prop = ValueProperty::empty(WrapBaseValueType::Float);
        custom_prop.set_value(Some(BaseValue::from(9.99_f32)));

        props.insert("custom_prop".to_string(), custom_prop);

        let got = props
            .get("custom_prop")
            .expect("custom_prop should be present")
            .get_value()
            .expect("custom_prop should have a value")
            .as_float()
            .expect("custom_prop should be a float");

        assert!((got - 9.99).abs() < 1e-6, "custom_prop float mismatch");
    }

    #[test]
    fn test_path_properties_v2_alias() {
        let path_props = PathPropertiesMap::default();

        // V2 alias should return the canonical path property (position)
        let p_by_v2 = path_props
            .get(V2_POSITION)
            .expect("V2_POSITION should map to a path property");
        let p_by_v3 = path_props
            .get(POSITION)
            .expect("POSITION should map to a path property");

        // pointers should be to the same underlying canonical field (addresses equal)
        assert!(
            std::ptr::eq(p_by_v3, p_by_v2),
            "V2_POSITION and POSITION should resolve to the same path property"
        );
    }

    #[test]
    fn test_last_updated() {
        let mut props = PropertiesMap::default();

        let time = props.dissolve.last_updated;
        assert!(
            time.elapsed().is_ok(),
            "last_updated should be a valid SystemTime"
        );
        // sleep to avoid potential flakiness of last_updated being the same as time (if set_value is called too quickly)
        sleep(Duration::from_millis(1));

        // Set dissolve to an initial value
        props.dissolve.set_value(Some(BaseValue::from(0.1_f32)));
        let first = props
            .dissolve
            .get_value()
            .expect("first value present")
            .as_float()
            .expect("first is float");
        assert!(
            props.dissolve.last_updated.duration_since(time).is_ok(),
            "last_updated should be a valid SystemTime after setting value"
        );
        assert!(
            props.dissolve.last_updated.duration_since(time).unwrap()
                > std::time::Duration::from_millis(0),
            "last_updated should be a valid SystemTime after setting value"
        );

        // Update dissolve to a new value
        props.dissolve.set_value(Some(BaseValue::from(0.2_f32)));
        let second = props
            .dissolve
            .get_value()
            .expect("second value present")
            .as_float()
            .expect("second is float");

        // Ensure the value actually changed (i.e., it was "updated")
        assert!(
            (second - first).abs() > 1e-6,
            "dissolve should have been updated to a new value"
        );
    }

    #[test]
    fn game_object_register_remove_and_callbacks() {
        use crate::animation::game_object::GameObject;
        use std::cell::RefCell;
        use std::ffi::c_void;
        use std::rc::Rc;

        let mut track = Track {
            name: "g_track".to_string(),
            ..Default::default()
        };

        // create a unique pointer for GameObject
        let p = Box::into_raw(Box::new(42)) as *const c_void;
        let go = GameObject::from(p);

        // no game objects initially
        assert!(!track.game_objects.contains(&go));

        // callback tracking (add/remove)
        let called = Rc::new(RefCell::new((false, false)));
        let called_cb = called.clone();
        let cb = Rc::new(move |_g: GameObject, added: bool| {
            let mut v = called_cb.borrow_mut();
            if added {
                v.0 = true;
            } else {
                v.1 = true;
            }
        });

        track.register_game_object_callback(cb.clone());

        // register the game object (should invoke callback with added=true)
        track.register_game_object(go);
        assert!(track.game_objects.contains(&go));
        assert!(called.borrow().0, "add callback should have fired");

        // registering again should not duplicate
        track.register_game_object(go);
        let count = track.game_objects.iter().filter(|x| **x == go).count();
        assert_eq!(count, 1, "duplicate game object should not be added");

        // remove it (should invoke callback with added=false)
        track.remove_game_object(&go);
        assert!(!track.game_objects.contains(&go));
        assert!(called.borrow().1, "remove callback should have fired");

        // free the boxed memory
        unsafe {
            drop(Box::from_raw(p as *mut i32));
        }
    }

    #[test]
    fn register_and_remove_game_object_callback_works() {
        use crate::animation::game_object::GameObject;
        use std::cell::RefCell;
        use std::ffi::c_void;
        use std::rc::Rc;

        let mut track = Track::default();

        // two distinct game objects (unique pointers)
        let p1 = Box::into_raw(Box::new(1)) as *const c_void;
        let p2 = Box::into_raw(Box::new(2)) as *const c_void;
        let go1 = GameObject::from(p1);
        let go2 = GameObject::from(p2);

        let cb1_called = Rc::new(RefCell::new(0usize));
        let cb2_called = Rc::new(RefCell::new(0usize));

        let cb1 = Rc::new({
            let c = cb1_called.clone();
            move |_g: GameObject, _added: bool| {
                *c.borrow_mut() += 1;
            }
        });

        let cb2 = Rc::new({
            let c = cb2_called.clone();
            move |_g: GameObject, _added: bool| {
                *c.borrow_mut() += 1;
            }
        });

        track.register_game_object_callback(cb1.clone());
        track.register_game_object_callback(cb2.clone());

        // add first object: both callbacks should fire once
        track.register_game_object(go1);
        assert_eq!(*cb1_called.borrow(), 1);
        assert_eq!(*cb2_called.borrow(), 1);

        // remove cb1
        track.remove_game_object_callback(cb1.clone());

        // add second object: only cb2 should fire
        track.register_game_object(go2);
        assert_eq!(*cb1_called.borrow(), 1, "cb1 should not have fired again");
        assert_eq!(*cb2_called.borrow(), 2, "cb2 should have fired twice");

        // cleanup leaked boxes
        unsafe {
            drop(Box::from_raw(p1 as *mut i32));
            drop(Box::from_raw(p2 as *mut i32));
        }
    }

    #[test]
    fn callbacks_and_gameobjects_preserve_order() {
        use crate::animation::game_object::GameObject;
        use std::cell::RefCell;
        use std::ffi::c_void;
        use std::rc::Rc;

        let mut track = Track::default();

        // prepare three unique game objects
        let p1 = Box::into_raw(Box::new(11)) as *const c_void;
        let p2 = Box::into_raw(Box::new(22)) as *const c_void;
        let p3 = Box::into_raw(Box::new(33)) as *const c_void;
        let go1 = GameObject::from(p1);
        let go2 = GameObject::from(p2);
        let go3 = GameObject::from(p3);

        // record order of callback invocations (by pointer value)
        let calls = Rc::new(RefCell::new(Vec::new()));

        let c1 = {
            let calls = calls.clone();
            Rc::new(move |g: GameObject, _added: bool| {
                calls.borrow_mut().push(g.ptr as usize);
            })
        };

        let c2 = {
            let calls = calls.clone();
            Rc::new(move |g: GameObject, _added: bool| {
                calls.borrow_mut().push(g.ptr as usize + 1000); // offset to distinguish
            })
        };

        // register callbacks in order c1 then c2
        track.register_game_object_callback(c1.clone());
        track.register_game_object_callback(c2.clone());

        // register game objects in order go1, go2, go3
        track.register_game_object(go1);
        track.register_game_object(go2);
        track.register_game_object(go3);

        // callbacks should have been called for each registration in registration order
        {
            let recorded = calls.borrow();
            // we expect 6 entries: for each gameobject, c1 then c2
            assert_eq!(recorded.len(), 6);
            // verify pattern: [p1, p1+1000, p2, p2+1000, p3, p3+1000]
            assert_eq!(recorded[0], p1 as usize);
            assert_eq!(recorded[1], p1 as usize + 1000);
            assert_eq!(recorded[2], p2 as usize);
            assert_eq!(recorded[3], p2 as usize + 1000);
            assert_eq!(recorded[4], p3 as usize);
            assert_eq!(recorded[5], p3 as usize + 1000);

            // verify track.game_objects preserves insertion order
            assert_eq!(track.game_objects.len(), 3);
            assert_eq!(track.game_objects[0], go1);
            assert_eq!(track.game_objects[1], go2);
            assert_eq!(track.game_objects[2], go3);
        }

        // remove middle object and ensure callbacks called (with removed=false)
        calls.borrow_mut().clear();
        track.remove_game_object(&go2);
        // removal should have called callbacks once per registered callback
        assert_eq!(calls.borrow().len(), 2);
        // after removal, remaining game_objects should be go1, go3 (order preserved)
        assert_eq!(track.game_objects.len(), 2);
        assert_eq!(track.game_objects[0], go1);
        assert_eq!(track.game_objects[1], go3);

        // cleanup boxes
        unsafe {
            drop(Box::from_raw(p1 as *mut i32));
            drop(Box::from_raw(p2 as *mut i32));
            drop(Box::from_raw(p3 as *mut i32));
        }
    }

    #[test]
    fn register_remove_multiple_times_invokes_callbacks_each_time() {
        use crate::animation::game_object::GameObject;
        use std::cell::RefCell;
        use std::ffi::c_void;
        use std::rc::Rc;

        let mut track = Track::default();

        let p = Box::into_raw(Box::new(55)) as *const c_void;
        let go = GameObject::from(p);

        let calls = Rc::new(RefCell::new(0usize));
        let calls_clone = calls.clone();
        let cb = Rc::new(move |_g: GameObject, _added: bool| {
            *calls_clone.borrow_mut() += 1;
        });

        track.register_game_object_callback(cb.clone());

        // alternate register/remove three times; reset counter each iteration and assert per-iteration
        for _ in 1..=3 {
            // reset
            *calls.borrow_mut() = 0;

            track.register_game_object(go);
            assert_eq!(
                *calls.borrow(),
                1,
                "register should have invoked callback once per registration"
            );

            track.remove_game_object(&go);
            assert_eq!(
                *calls.borrow(),
                2,
                "remove should have invoked callback once per removal"
            );
        }

        unsafe {
            drop(Box::from_raw(p as *mut i32));
        }
    }
}
