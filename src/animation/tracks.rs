use std::{collections::HashMap, time::Instant};

use super::{
    game_object::GameObject,
    property::{PathProperty, ValueProperty},
};

pub struct Track {
    pub properties: HashMap<String, ValueProperty>,
    pub path_properties: HashMap<String, PathProperty>,

    // hashset but must be insertion ordered
    pub game_objects: Vec<GameObject>,

    pub last_updated: Instant,
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
    }

    pub fn get_property(&self, id: &str) -> Option<&ValueProperty> {
        self.properties.get(id)
    }
    pub fn get_path_property(&self, id: &str) -> Option<&PathProperty> {
        self.path_properties.get(id)
    }

    pub fn remove_game_object(&mut self, game_object: &GameObject) {
        self.game_objects.retain(|go| go != game_object);
    }

    pub fn mark_updated(&mut self) {
        self.last_updated = Instant::now();
    }
}
