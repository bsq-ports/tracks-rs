use core::panic;
use std::rc::Rc;

use crate::{
    animation::{coroutine_manager::CoroutineManager, tracks_holder::TracksHolder},
    base_provider_context::BaseProviderContext,
    ffi::types::WrapBaseValueType,
    point_definition::{PointDefinition, base_point_definition::BasePointDefinition},
};

/// Context that holds tracks, point definitions, and coroutine manager.
pub struct TracksContext {
    // we use an Rc here so vec reallocs don't break the track pointers
    // though we could also use a linkedlist

    // Using SlotMap as it provides stable keys and efficient storage
    // very fast lookups vs HashMap and avoids fragmentation issues of Vec
    pub tracks: TracksHolder,
    // TODO: Removable point definitions?
    point_definitions: ahash::AHashMap<(String, WrapBaseValueType), Rc<BasePointDefinition>>,
    pub coroutine_manager: CoroutineManager,
    pub base_providers: BaseProviderContext,
}

impl TracksContext {
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

    pub fn get_base_provider_context(&self) -> &BaseProviderContext {
        &self.base_providers
    }
    pub fn get_mut_base_provider_context(&mut self) -> &mut BaseProviderContext {
        &mut self.base_providers
    }
}

impl Default for TracksContext {
    fn default() -> Self {
        TracksContext {
            tracks: Default::default(),
            point_definitions: Default::default(),
            coroutine_manager: CoroutineManager::default(),
            base_providers: BaseProviderContext::new(),
        }
    }
}
