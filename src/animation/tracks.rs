use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Instant};

use crate::values::value::BaseValue;

use super::{
    game_object::GameObject,
    property::{PathProperty, PathPropertyGlobal, ValuePropertyGlobal},
};

pub type TrackGlobal = Rc<RefCell<Track>>;

pub struct Track {
    pub properties: HashMap<String, ValuePropertyGlobal>,
    pub path_properties: HashMap<String, PathPropertyGlobal>,

    // hashset but must be insertion ordered
    pub game_objects: Vec<GameObject>,

    pub last_updated: Instant,
}

impl Track {
    pub fn register_property(&mut self, id: String, property: ValuePropertyGlobal) {
        self.properties.insert(id, property);
    }

    pub fn register_path_property(&mut self, id: String, property: PathPropertyGlobal) {
        self.path_properties.insert(id, property);
    }

    pub fn register_game_object(&mut self, game_object: GameObject) {
        if self.game_objects.contains(&game_object) {
            return;
        }

        self.game_objects.push(game_object);
    }

    pub fn get_property(&self, id: &str) -> Option<&ValuePropertyGlobal> {
        self.properties.get(id)
    }
    pub fn get_path_property(&self, id: &str) -> Option<&PathPropertyGlobal> {
        self.path_properties.get(id)
    }

    pub fn remove_game_object(&mut self, game_object: &GameObject) {
        self.game_objects.retain(|go| go != game_object);
    }

    pub fn mark_updated(&mut self) {
        self.last_updated = Instant::now();
    }
}

impl Default for Track {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            path_properties: Default::default(),
            game_objects: Default::default(),
            last_updated: Instant::now(),
        }
    }
}
