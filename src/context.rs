use core::panic;
use std::{cell::RefCell, rc::Rc};

use crate::{
    animation::{coroutine_manager::CoroutineManager, tracks::Track},
    base_provider_context::BaseProviderContext,
    ffi::types::WrapBaseValueType,
    point_definition::{PointDefinition, base_point_definition::BasePointDefinition},
};

pub struct TracksContext<'a> {
    // we use an Rc here so vec reallocs don't break the track pointers
    // though we could also use a linkedlist

    // TODO: Removable tracks
    tracks: Vec<Rc<RefCell<Track<'a>>>>,
    // TODO: Removable point definitions?
    point_definitions: ahash::AHashMap<(String, WrapBaseValueType), Rc<BasePointDefinition>>,
    pub coroutine_manager: CoroutineManager<'a>,
    pub base_providers: BaseProviderContext,
}

impl<'a> TracksContext<'a> {
    pub fn add_track(&mut self, track: Rc<RefCell<Track<'a>>>) {
        if self
            .tracks
            .iter()
            .any(|t| t.borrow().name == track.borrow().name)
        {
            // If the track already exists, we can just return it
            // This avoids unnecessary duplication of tracks
            panic!("Track with name '{}' already exists.", track.borrow().name);
        }

        self.tracks.push(track);
    }

    pub fn add_point_definition(&mut self, id: String, point_definition: Rc<BasePointDefinition>) {
        if self
            .point_definitions
            .contains_key(&(id.clone(), point_definition.get_type()))
        {
            // If the point definition already exists, we can just return it
            // This avoids unnecessary duplication of point definitions
            panic!(
                "Point definition with id '{}' and type '{:?}' already exists.",
                id,
                point_definition.get_type()
            );
        }

        let ty = point_definition.get_type();
        self.point_definitions.insert((id, ty), point_definition);
    }

    pub fn get_point_definition(
        &self,
        name: &str,
        typ: WrapBaseValueType,
    ) -> Option<Rc<BasePointDefinition>> {
        self.point_definitions
            .get(&(name.to_string(), typ))
            .cloned()
    }

    pub fn get_track(&mut self, index: usize) -> Option<Rc<RefCell<Track<'a>>>> {
        self.tracks.get_mut(index).cloned()
    }

    pub fn get_track_by_name(&mut self, name: &str) -> Option<Rc<RefCell<Track<'a>>>> {
        self.tracks
            .iter_mut()
            .find(|track: &&mut Rc<RefCell<Track<'a>>>| track.borrow().name == name)
            .cloned()
    }

    pub fn get_base_provider_context(&self) -> &BaseProviderContext {
        &self.base_providers
    }
    pub fn get_mut_base_provider_context(&mut self) -> &mut BaseProviderContext {
        &mut self.base_providers
    }
}

impl<'a> Default for TracksContext<'a> {
    fn default() -> Self {
        TracksContext {
            tracks: Vec::new(),
            point_definitions: Default::default(),
            coroutine_manager: CoroutineManager::default(),
            base_providers: BaseProviderContext::new(),
        }
    }
}
