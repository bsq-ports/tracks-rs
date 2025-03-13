use crate::{
    animation::{coroutine_manager::CoroutineManager, tracks::Track},
    point_definition::BasePointDefinition,
    values::base_provider_context::BaseProviderContext,
};

pub struct TracksContext<'a> {
    // TODO: Removable tracks/
    pub tracks: Vec<Track<'a>>,
    // TODO: Removable point definitions?
    pub point_definitions: Vec<BasePointDefinition>,
    pub coroutine_manager: CoroutineManager<'a>,
    pub base_providers: BaseProviderContext,
}

impl<'a> TracksContext<'a> {
    pub fn add_track(&mut self, track: Track<'a>) {
        self.tracks.push(track);
    }

    pub fn add_point_definition(&mut self, point_definition: BasePointDefinition) {
        self.point_definitions.push(point_definition);
    }

    pub fn get_track(&mut self, index: usize) -> Option<&mut Track<'a>> {
        self.tracks.get_mut(index)
    }

    pub fn get_track_by_name(&mut self, name: &str) -> Option<&mut Track<'a>> {
        self.tracks.iter_mut().find(|track| track.name == name)
    }

    pub fn get_base_provider_context(&self) -> &BaseProviderContext {
        &self.base_providers
    }
}