use std::{fmt::Display, rc::Rc, str::FromStr};

use super::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
};

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

#[repr(u32)]
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
}

#[derive(Default, Clone)]
pub struct PathPropertiesMap<'a> {
    pub path_properties: ahash::AHashMap<String, PathProperty<'a>>,

    pub position: PathProperty<'a>,
    pub rotation: PathProperty<'a>,
    pub scale: PathProperty<'a>,
    pub local_rotation: PathProperty<'a>,
    pub local_position: PathProperty<'a>,
    pub definite_position: PathProperty<'a>,
    pub dissolve: PathProperty<'a>,
    pub dissolve_arrow: PathProperty<'a>,
    pub cuttable: PathProperty<'a>,
    pub color: PathProperty<'a>,
}

#[derive(Default, Clone)]
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

pub trait GameObjectCallback = Fn(GameObject, bool);

#[derive(Clone)]
pub struct Track<'a> {
    pub properties: PropertiesMap,
    pub path_properties: PathPropertiesMap<'a>,

    pub name: String,

    // hashset but must be insertion ordered
    pub game_objects: Vec<GameObject>,
    pub game_object_callbacks: Vec<Rc<dyn GameObjectCallback>>,
}

impl<'a> Track<'a> {
    pub fn register_property(&mut self, id: String, property: ValueProperty) {
        self.properties.insert(id, property);
    }

    pub fn register_path_property(&mut self, id: String, property: PathProperty<'a>) {
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

    pub fn get_property(&self, id: &str) -> Option<&ValueProperty> {
        self.properties.get(id)
    }
    pub fn get_mut_property(&mut self, id: &str) -> Option<&mut ValueProperty> {
        self.properties.get_mut(id)
    }
    pub fn get_path_property(&self, id: &str) -> Option<&PathProperty> {
        self.path_properties.get(id)
    }

    pub fn get_mut_path_property(&mut self, id: &str) -> Option<&mut PathProperty<'a>> {
        self.path_properties.get_mut(id)
    }

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
}

impl Default for Track<'_> {
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
}

impl<'a> PathPropertiesMap<'a> {
    pub fn insert(&mut self, id: String, property: PathProperty<'a>) {
        match self.get_mut(&id) {
            Some(prop) => *prop = property,
            None => {
                self.path_properties.insert(id, property);
            }
        }
    }

    pub fn get_property_by_name(&self, name: PropertyNames) -> Option<&PathProperty<'a>> {
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

    pub fn get_property_by_name_mut(
        &mut self,
        name: PropertyNames,
    ) -> Option<&mut PathProperty<'a>> {
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

    pub fn get(&self, id: &str) -> Option<&PathProperty<'a>> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name(name),
            _ => self.path_properties.get(id),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut PathProperty<'a>> {
        match PropertyNames::from_str(id) {
            Ok(name) => self.get_property_by_name_mut(name),
            _ => self.path_properties.get_mut(id),
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
            _ => Err(()),
        }
    }
}

impl Display for PropertyNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyNames::Position => write!(f, "{}", POSITION),
            PropertyNames::Rotation => write!(f, "{}", ROTATION),
            PropertyNames::Scale => write!(f, "{}", SCALE),
            PropertyNames::LocalRotation => write!(f, "{}", LOCAL_ROTATION),
            PropertyNames::LocalPosition => write!(f, "{}", LOCAL_POSITION),
            PropertyNames::DefinitePosition => write!(f, "{}", DEFINITE_POSITION),
            PropertyNames::Dissolve => write!(f, "{}", DISSOLVE),
            PropertyNames::DissolveArrow => write!(f, "{}", DISSOLVE_ARROW),
            PropertyNames::Time => write!(f, "{}", TIME),
            PropertyNames::Cuttable => write!(f, "{}", CUTTABLE),
            PropertyNames::Color => write!(f, "{}", COLOR),
            PropertyNames::Attentuation => write!(f, "{}", ATTENTUATION),
            PropertyNames::FogOffset => write!(f, "{}", FOG_OFFSET),
            PropertyNames::HeightFogStartY => write!(f, "{}", HEIGHT_FOG_START_Y),
            PropertyNames::HeightFogHeight => write!(f, "{}", HEIGHT_FOG_HEIGHT),
        }
    }
}
