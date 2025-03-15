use std::{cell::RefCell, rc::Rc};

use crate::{
    animation::{coroutine_manager::CoroutineManager, tracks::Track},
    point_definition::BasePointDefinition,
    values::base_provider_context::BaseProviderContext,
};

pub struct TracksContext<'a> {
    // TODO: Removable tracks/
    pub tracks: Vec<Rc<RefCell<Track<'a>>>>,
    // TODO: Removable point definitions?
    pub point_definitions: Vec<Rc<BasePointDefinition>>,
    pub coroutine_manager: CoroutineManager<'a>,
    pub base_providers: BaseProviderContext,
}

impl<'a> TracksContext<'a> {
    pub fn add_track(&mut self, track: Rc<RefCell<Track<'a>>>) {
        self.tracks.push(track);
    }

    pub fn add_point_definition(&mut self, point_definition: Rc<BasePointDefinition>) {
        self.point_definitions.push(point_definition);
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
