use std::{fmt::Display, rc::Rc, str::FromStr};

use crate::{
    animation::property::{PathProperty, ValueProperty},
    ffi::types::WrapBaseValueType,
};

use super::game_object::GameObject;

// Define constants for property names
pub const POSITION: &str = "position";
pub const ROTATION: &str = "rotation";
pub const SCALE: &str = "scale";
pub const LOCAL_ROTATION: &str = "local_rotation";
pub const LOCAL_POSITION: &str = "local_position";
pub const DEFINITE_POSITION: &str = "definite_position";
pub const DISSOLVE: &str = "dissolve";
pub const DISSOLVE_ARROW: &str = "dissolve_arrow";
pub const TIME: &str = "time";
pub const CUTTABLE: &str = "cuttable";
pub const COLOR: &str = "color";
pub const ATTENTUATION: &str = "attentuation";
pub const FOG_OFFSET: &str = "fog_offset";
pub const HEIGHT_FOG_START_Y: &str = "height_fog_start_y";
pub const HEIGHT_FOG_HEIGHT: &str = "height_fog_height";

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ValuePropertyHandle {
    ByName(String),
    ById(PropertyNames),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    Rotation,
    Scale,
    LocalRotation,
    LocalPosition,
    DefinitePosition,
    Dissolve,
    DissolveArrow,
    Time,
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
#[derive()]
pub struct PathPropertiesMap {
    pub path_properties: ahash::AHashMap<String, PathProperty>,

    pub position: PathProperty,
    pub rotation: PathProperty,
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
    pub rotation: ValueProperty,
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
pub trait GameObjectCallback = Fn(GameObject, bool);

/// A Track represents a collection of properties and path properties associated with game objects.
/// It allows registering, retrieving, and managing properties and game objects.
#[derive()]
pub struct Track {
    pub properties: PropertiesMap,
    pub path_properties: PathPropertiesMap,

    pub name: String,

    // hashset but must be insertion ordered
    pub game_objects: Vec<GameObject>,
    pub game_object_callbacks: Vec<Rc<dyn GameObjectCallback>>,
}

impl Track {
    pub fn register_property(&mut self, id: String, property: ValueProperty) {
        self.properties.insert(id, property);
    }

    pub fn register_path_property(&mut self, id: String, property: PathProperty) {
        self.path_properties.insert(id, property);
    }

    pub fn register_game_object(&mut self, game_object: GameObject) {
        if self.game_objects.contains(&game_object) {
            return;
        }

        self.game_objects.push(game_object);

        self.game_object_callbacks
            .iter()
            .for_each(|callback| callback(game_object, true));
    }

    pub fn register_game_object_callback<F>(&mut self, callback: Rc<F>)
    where
        F: GameObjectCallback + 'static,
    {
        self.game_object_callbacks.push(callback);
    }

    // pub fn get_property(&self, id: &str) -> Option<&ValueProperty> {
    //     self.properties.get(id)
    // }
    // pub fn get_path_property(&self, id: &str) -> Option<&PathProperty> {
    //     self.path_properties.get(id)
    // }

    // pub fn get_property_mut(&mut self, id: &str) -> Option<&mut ValueProperty> {
    //     self.properties.get_mut(id)
    // }

    // pub fn get_path_property_mut(&mut self, id: &str) -> Option<&mut PathProperty> {
    //     self.path_properties.get_mut(id)
    // }

    pub fn remove_game_object(&mut self, game_object: &GameObject) {
        self.game_objects.retain(|go| go != game_object);

        self.game_object_callbacks
            .iter()
            .for_each(|callback| callback(*game_object, false));
    }

    pub fn remove_game_object_callback<F>(&mut self, callback: Rc<F>)
    where
        F: GameObjectCallback + 'static,
    {
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
            rotation: ValueProperty::empty(WrapBaseValueType::Quat),
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
            rotation: PathProperty::empty(WrapBaseValueType::Quat),
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

impl Default for Track {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            path_properties: Default::default(),
            game_objects: Default::default(),
            name: "".to_string(),
            game_object_callbacks: Vec::new(),
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
            PropertyNames::Rotation => Some(&self.rotation),
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
            PropertyNames::Rotation => Some(&mut self.rotation),
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
            PropertyNames::Rotation => Some(&self.rotation),
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
            PropertyNames::Rotation => Some(&mut self.rotation),
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
            POSITION => Ok(PropertyNames::Position),
            ROTATION => Ok(PropertyNames::Rotation),
            SCALE => Ok(PropertyNames::Scale),
            LOCAL_ROTATION => Ok(PropertyNames::LocalRotation),
            LOCAL_POSITION => Ok(PropertyNames::LocalPosition),
            DEFINITE_POSITION => Ok(PropertyNames::DefinitePosition),
            DISSOLVE => Ok(PropertyNames::Dissolve),
            DISSOLVE_ARROW => Ok(PropertyNames::DissolveArrow),
            TIME => Ok(PropertyNames::Time),
            CUTTABLE => Ok(PropertyNames::Cuttable),
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
            PropertyNames::Rotation => write!(f, "{ROTATION}"),
            PropertyNames::Scale => write!(f, "{SCALE}"),
            PropertyNames::LocalRotation => write!(f, "{LOCAL_ROTATION}"),
            PropertyNames::LocalPosition => write!(f, "{LOCAL_POSITION}"),
            PropertyNames::DefinitePosition => write!(f, "{DEFINITE_POSITION}"),
            PropertyNames::Dissolve => write!(f, "{DISSOLVE}"),
            PropertyNames::DissolveArrow => write!(f, "{DISSOLVE_ARROW}"),
            PropertyNames::Time => write!(f, "{TIME}"),
            PropertyNames::Cuttable => write!(f, "{CUTTABLE}"),
            PropertyNames::Color => write!(f, "{COLOR}"),
            PropertyNames::Attentuation => write!(f, "{ATTENTUATION}"),
            PropertyNames::FogOffset => write!(f, "{FOG_OFFSET}"),
            PropertyNames::HeightFogStartY => write!(f, "{HEIGHT_FOG_START_Y}"),
            PropertyNames::HeightFogHeight => write!(f, "{HEIGHT_FOG_HEIGHT}"),
            PropertyNames::UnknownPropertyName => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::value::BaseValue;
    use glam::{Quat, Vec3, Vec4};

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
}
