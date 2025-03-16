use std::time::Instant;

use super::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
};

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

#[derive(Clone)]
pub struct Track<'a> {
    pub properties: PropertiesMap,
    pub path_properties: PathPropertiesMap<'a>,

    pub name: String,

    // hashset but must be insertion ordered
    pub game_objects: Vec<GameObject>,

    pub last_updated: Instant,
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
    }

    pub fn mark_updated(&mut self) {
        self.last_updated = Instant::now();
    }
}

impl Default for Track<'_> {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            path_properties: Default::default(),
            game_objects: Default::default(),
            last_updated: Instant::now(),
            name: "".to_string(),
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

    pub fn get(&self, id: &str) -> Option<&ValueProperty> {
        match id {
            "position" => Some(&self.position),
            "rotation" => Some(&self.rotation),
            "scale" => Some(&self.scale),
            "local_rotation" => Some(&self.local_rotation),
            "local_position" => Some(&self.local_position),
            "dissolve" => Some(&self.dissolve),
            "dissolve_arrow" => Some(&self.dissolve_arrow),
            "time" => Some(&self.time),
            "cuttable" => Some(&self.cuttable),
            "color" => Some(&self.color),
            "attentuation" => Some(&self.attentuation),
            "fog_offset" => Some(&self.fog_offset),
            "height_fog_start_y" => Some(&self.height_fog_start_y),
            "height_fog_height" => Some(&self.height_fog_height),
            _ => self.properties.get(id),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut ValueProperty> {
        match id {
            "position" => Some(&mut self.position),
            "rotation" => Some(&mut self.rotation),
            "scale" => Some(&mut self.scale),
            "local_rotation" => Some(&mut self.local_rotation),
            "local_position" => Some(&mut self.local_position),
            "dissolve" => Some(&mut self.dissolve),
            "dissolve_arrow" => Some(&mut self.dissolve_arrow),
            "time" => Some(&mut self.time),
            "cuttable" => Some(&mut self.cuttable),
            "color" => Some(&mut self.color),
            "attentuation" => Some(&mut self.attentuation),
            "fog_offset" => Some(&mut self.fog_offset),
            "height_fog_start_y" => Some(&mut self.height_fog_start_y),
            "height_fog_height" => Some(&mut self.height_fog_height),
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

    pub fn get(&self, id: &str) -> Option<&PathProperty<'a>> {
        match id {
            "position" => Some(&self.position),
            "rotation" => Some(&self.rotation),
            "scale" => Some(&self.scale),
            "local_rotation" => Some(&self.local_rotation),
            "local_position" => Some(&self.local_position),
            "definite_position" => Some(&self.definite_position),
            "dissolve" => Some(&self.dissolve),
            "dissolve_arrow" => Some(&self.dissolve_arrow),
            "cuttable" => Some(&self.cuttable),
            "color" => Some(&self.color),
            _ => self.path_properties.get(id),
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut PathProperty<'a>> {
        match id {
            "position" => Some(&mut self.position),
            "rotation" => Some(&mut self.rotation),
            "scale" => Some(&mut self.scale),
            "local_rotation" => Some(&mut self.local_rotation),
            "local_position" => Some(&mut self.local_position),
            "definite_position" => Some(&mut self.definite_position),
            "dissolve" => Some(&mut self.dissolve),
            "dissolve_arrow" => Some(&mut self.dissolve_arrow),
            "cuttable" => Some(&mut self.cuttable),
            "color" => Some(&mut self.color),
            _ => self.path_properties.get_mut(id),
        }
    }
}
