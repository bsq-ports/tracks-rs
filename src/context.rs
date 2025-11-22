use core::panic;
use std::rc::Rc;

use crate::{
    animation::{coroutine_manager::CoroutineManager, tracks_holder::TracksHolder},
    base_provider_context::BaseProviderContext,
    ffi::types::WrapBaseValueType,
    point_definition::{PointDefinition, base_point_definition::BasePointDefinition},
};

/// Context that holds tracks, point definitions, and coroutine manager.
#[derive(Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::events::{EventData, EventType};
    use crate::animation::track::ValuePropertyHandle;
    use crate::easings::functions::Functions;
    use crate::modifiers::float_modifier::FloatValues;
    use crate::modifiers::quaternion_modifier::QuaternionValues;
    use crate::modifiers::vector3_modifier::Vector3Values;
    use crate::modifiers::vector4_modifier::Vector4Values;
    use crate::point_data::PointData;
    use crate::point_data::float_point_data::FloatPointData;
    use crate::point_data::quaternion_point_data::QuaternionPointData;
    use crate::point_data::vector3_point_data::Vector3PointData;
    use crate::point_data::vector4_point_data::Vector4PointData;
    use crate::point_definition::float_point_definition::FloatPointDefinition;
    use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;
    use crate::point_definition::vector3_point_definition::Vector3PointDefinition;
    use crate::point_definition::vector4_point_definition::Vector4PointDefinition;
    use glam::{Quat, Vec3, Vec4};
    use std::rc::Rc;

    #[test]
    fn tracks_context_roundtrip_and_coroutine() {
        let mut ctx = TracksContext::default();

        // Float definition
        let fp = FloatPointDefinition::new(vec![
            PointData::Float(FloatPointData::new(
                FloatValues::Static(0.0),
                0.0,
                vec![],
                Functions::EaseLinear,
            )),
            PointData::Float(FloatPointData::new(
                FloatValues::Static(10.0),
                1.0,
                vec![],
                Functions::EaseLinear,
            )),
        ]);
        ctx.add_point_definition("pf".to_string(), Rc::new(BasePointDefinition::Float(fp)));

        // Vec3 definition
        let v3p = Vector3PointDefinition::new(vec![
            PointData::Vector3(Vector3PointData::new(
                Vector3Values::Static(Vec3::new(0.0, 0.0, 0.0)),
                false,
                0.0,
                vec![],
                Functions::EaseLinear,
            )),
            PointData::Vector3(Vector3PointData::new(
                Vector3Values::Static(Vec3::new(3.0, 3.0, 3.0)),
                false,
                1.0,
                vec![],
                Functions::EaseLinear,
            )),
        ]);
        ctx.add_point_definition(
            "pv3".to_string(),
            Rc::new(BasePointDefinition::Vector3(v3p)),
        );

        // Vec4 definition
        let v4p = Vector4PointDefinition::new(vec![
            PointData::Vector4(Vector4PointData::new(
                Vector4Values::Static(Vec4::new(0.0, 0.0, 0.0, 0.0)),
                false,
                0.0,
                vec![],
                Functions::EaseLinear,
            )),
            PointData::Vector4(Vector4PointData::new(
                Vector4Values::Static(Vec4::new(4.0, 4.0, 4.0, 4.0)),
                false,
                1.0,
                vec![],
                Functions::EaseLinear,
            )),
        ]);
        ctx.add_point_definition(
            "pv4".to_string(),
            Rc::new(BasePointDefinition::Vector4(v4p)),
        );

        // Quat definition
        let qp =
            QuaternionPointDefinition::new(vec![PointData::Quaternion(QuaternionPointData::new(
                QuaternionValues::Static(Vec3::ZERO, Quat::IDENTITY),
                0.0,
                vec![],
                Functions::EaseLinear,
            ))]);
        ctx.add_point_definition(
            "pq".to_string(),
            Rc::new(BasePointDefinition::Quaternion(qp)),
        );

        // retrieve and sanity check
        assert!(
            ctx.get_point_definition("pf", crate::ffi::types::WrapBaseValueType::Float)
                .is_some()
        );
        assert!(
            ctx.get_point_definition("pv3", crate::ffi::types::WrapBaseValueType::Vec3)
                .is_some()
        );
        assert!(
            ctx.get_point_definition("pv4", crate::ffi::types::WrapBaseValueType::Vec4)
                .is_some()
        );
        assert!(
            ctx.get_point_definition("pq", crate::ffi::types::WrapBaseValueType::Quat)
                .is_some()
        );

        // Add a track and run a simple coroutine using the context's coroutine manager
        let mut t = crate::animation::track::Track::default();
        t.name = "ctx_track".to_string();
        let key = ctx.tracks.add_track(t);

        let pd = ctx
            .get_point_definition("pf", crate::ffi::types::WrapBaseValueType::Float)
            .unwrap();

        let ev = EventData {
            raw_duration: 1.0,
            easing: Functions::EaseLinear,
            repeat: 0,
            start_song_time: 0.0,
            property: EventType::AnimateTrack(ValuePropertyHandle::new("dissolve")),
            track_key: key,
            point_data: Some((*pd).clone()),
        };

        ctx.coroutine_manager.start_event_coroutine(
            60.0,
            0.0,
            &ctx.base_providers,
            &mut ctx.tracks,
            ev,
        );
        ctx.coroutine_manager
            .poll_events(0.5, &ctx.base_providers, &mut ctx.tracks);

        let track = ctx.tracks.get_track(key).unwrap();
        let v = track
            .properties
            .dissolve
            .get_value()
            .unwrap()
            .as_float()
            .unwrap();
        assert!((v - 5.0).abs() < 1e-3, "expected ~5.0 got {}", v);
    }

    #[cfg(feature = "json")]
    #[test]
    fn tracks_context_json_parse() {
        use serde_json::json;
        let mut ctx = TracksContext::default();
        let base_ctx = ctx.get_base_provider_context();

        // simple JSON for float: [[0,0],[10,1]] wrapped as array
        let data = json!([[0, 0], [10, 1]]);
        let parsed: FloatPointDefinition = FloatPointDefinition::parse(data, base_ctx);
        assert_eq!(parsed.get_count(), 2);

        ctx.add_point_definition(
            "jf".to_string(),
            Rc::new(BasePointDefinition::Float(parsed)),
        );
        assert!(
            ctx.get_point_definition("jf", crate::ffi::types::WrapBaseValueType::Float)
                .is_some()
        );
    }
}
