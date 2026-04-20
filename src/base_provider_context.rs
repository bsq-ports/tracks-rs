use std::{cell::RefCell, collections::HashMap, rc::Rc};

use glam::{Quat, Vec3, Vec4};
use log::{error, warn};

use crate::{
    base_value::BaseValue,
    providers::{
        AbstractValueProvider, UpdateableValues, ValueProvider, base::BaseProviderValues,
        quat::QuaternionProviderValues, smooth::SmoothProvidersValues,
        smooth_rot::SmoothRotationProvidersValues,
    },
};

/// Context for base value providers
/// Holds all the base values that can be accessed
/// by base value providers
///
/// This context is passed to the value providers
/// to get the current base values
#[derive(Default, Clone)]
pub struct BaseProviderContext {
    //Score
    base_combo: f32,
    multiplied_score: f32,
    immediate_max_possible_multiplied_score: f32,
    modified_score: f32,
    immediate_max_possible_modified_score: f32,
    relative_score: f32,
    multiplier: f32,
    energy: f32,
    song_time: f32,
    song_length: f32,

    //Colors
    environment_color_0: Vec4,
    environment_color_0_boost: Vec4,
    environment_color_1: Vec4,
    environment_color_1_boost: Vec4,
    environment_color_w: Vec4,
    environment_color_w_boost: Vec4,
    note_color_0: Vec4,
    note_color_1: Vec4,
    obstacles_color: Vec4,
    saber_color_a: Vec4,
    saber_color_b: Vec4,

    //Transforms
    head_local_position: Vec3,
    head_local_rotation: Quat,
    head_local_scale: Vec3,
    head_position: Vec3,
    head_rotation: Quat,
    left_hand_local_position: Vec3,
    left_hand_local_rotation: Quat,
    left_hand_local_scale: Vec3,
    left_hand_position: Vec3,
    left_hand_rotation: Quat,
    right_hand_local_position: Vec3,
    right_hand_local_rotation: Quat,
    right_hand_local_scale: Vec3,
    right_hand_position: Vec3,
    right_hand_rotation: Quat,

    updatable_providers: Vec<Rc<RefCell<dyn UpdateableValues>>>,
    provider_cache: HashMap<String, ValueProvider>,
}

impl BaseProviderContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_values(&self, base: &str) -> BaseValue {
        match base {
            "baseCombo" => self.base_combo.into(),

            "baseMultipliedScore" => self.multiplied_score.into(),
            "baseImmediateMaxPossibleMultipliedScore" => {
                self.immediate_max_possible_multiplied_score.into()
            }

            "baseModifiedScore" => self.modified_score.into(),
            "baseImmediateMaxPossibleModifiedScore" => {
                self.immediate_max_possible_modified_score.into()
            }
            "baseRelativeScore" => self.relative_score.into(),
            "baseMultiplier" => self.multiplier.into(),
            "baseEnergy" => self.energy.into(),
            "baseSongTime" => self.song_time.into(),
            "baseSongLength" => self.song_length.into(),

            "baseEnvironmentColor0" => self.environment_color_0.into(),
            "baseEnvironmentColor0Boost" => self.environment_color_0_boost.into(),
            "baseEnvironmentColor1" => self.environment_color_1.into(),
            "baseEnvironmentColor1Boost" => self.environment_color_1_boost.into(),
            "baseEnvironmentColorW" => self.environment_color_w.into(),
            "baseEnvironmentColorWBoost" => self.environment_color_w_boost.into(),
            "baseNote0Color" => self.note_color_0.into(),
            "baseNote1Color" => self.note_color_1.into(),
            "baseObstaclesColor" => self.obstacles_color.into(),
            "baseSaberAColor" => self.saber_color_a.into(),
            "baseSaberBColor" => self.saber_color_b.into(),

            "baseHeadLocalPosition" => self.head_local_position.into(),
            "baseHeadLocalRotation" => self.head_local_rotation.into(),
            "baseHeadLocalScale" => self.head_local_scale.into(),
            "baseHeadPosition" => self.head_position.into(),
            "baseHeadRotation" => self.head_rotation.into(),
            "baseLeftHandLocalPosition" => self.left_hand_local_position.into(),
            "baseLeftHandLocalRotation" => self.left_hand_local_rotation.into(),
            "baseLeftHandLocalScale" => self.left_hand_local_scale.into(),
            "baseLeftHandPosition" => self.left_hand_position.into(),
            "baseLeftHandRotation" => self.left_hand_rotation.into(),
            "baseRightHandLocalPosition" => self.right_hand_local_position.into(),
            "baseRightHandLocalRotation" => self.right_hand_local_rotation.into(),
            "baseRightHandLocalScale" => self.right_hand_local_scale.into(),
            "baseRightHandPosition" => self.right_hand_position.into(),
            "baseRightHandRotation" => self.right_hand_rotation.into(),
            _ => panic!("Base provider not found {base}"),
        }
    }

    pub fn set_values(&mut self, base: &str, values: BaseValue) {
        match base {
            "baseCombo" => {
                self.base_combo = values[0];
            }
            "baseMultipliedScore" => {
                self.multiplied_score = values[0];
            }
            "baseImmediateMaxPossibleMultipliedScore" => {
                self.immediate_max_possible_multiplied_score = values[0];
            }
            "baseModifiedScore" => {
                self.modified_score = values[0];
            }
            "baseImmediateMaxPossibleModifiedScore" => {
                self.immediate_max_possible_modified_score = values[0];
            }
            "baseRelativeScore" => {
                self.relative_score = values[0];
            }
            "baseMultiplier" => {
                self.multiplier = values[0];
            }
            "baseEnergy" => {
                self.energy = values[0];
            }
            "baseSongTime" => {
                self.song_time = values[0];
            }
            "baseSongLength" => {
                self.song_length = values[0];
            }
            "baseEnvironmentColor0" => {
                self.environment_color_0 = values.as_vec4().unwrap();
            }
            "baseEnvironmentColor0Boost" => {
                self.environment_color_0_boost = values.as_vec4().unwrap();
            }
            "baseEnvironmentColor1" => {
                self.environment_color_1 = values.as_vec4().unwrap();
            }
            "baseEnvironmentColor1Boost" => {
                self.environment_color_1_boost = values.as_vec4().unwrap();
            }
            "baseEnvironmentColorW" => {
                self.environment_color_w = values.as_vec4().unwrap();
            }
            "baseEnvironmentColorWBoost" => {
                self.environment_color_w_boost = values.as_vec4().unwrap();
            }
            "baseNote0Color" => {
                self.note_color_0 = values.as_vec4().unwrap();
            }
            "baseNote1Color" => {
                self.note_color_1 = values.as_vec4().unwrap();
            }
            "baseObstaclesColor" => {
                self.obstacles_color = values.as_vec4().unwrap();
            }
            "baseSaberAColor" => {
                self.saber_color_a = values.as_vec4().unwrap();
            }
            "baseSaberBColor" => {
                self.saber_color_b = values.as_vec4().unwrap();
            }
            "baseHeadLocalPosition" => {
                self.head_local_position = values.as_vec3().unwrap();
            }
            "baseHeadLocalRotation" => {
                self.head_local_rotation = values.as_quat().unwrap();
            }
            "baseHeadLocalScale" => {
                self.head_local_scale = values.as_vec3().unwrap();
            }
            "baseHeadPosition" => {
                self.head_position = values.as_vec3().unwrap();
            }
            "baseHeadRotation" => {
                self.head_rotation = values.as_quat().unwrap();
            }
            "baseLeftHandLocalPosition" => {
                self.left_hand_local_position = values.as_vec3().unwrap();
            }
            "baseLeftHandLocalRotation" => {
                self.left_hand_local_rotation = values.as_quat().unwrap();
            }
            "baseLeftHandLocalScale" => {
                self.left_hand_local_scale = values.as_vec3().unwrap();
            }
            "baseLeftHandPosition" => {
                self.left_hand_position = values.as_vec3().unwrap();
            }
            "baseLeftHandRotation" => {
                self.left_hand_rotation = values.as_quat().unwrap();
            }
            "baseRightHandLocalPosition" => {
                self.right_hand_local_position = values.as_vec3().unwrap();
            }
            "baseRightHandLocalRotation" => {
                self.right_hand_local_rotation = values.as_quat().unwrap();
            }
            "baseRightHandLocalScale" => {
                self.right_hand_local_scale = values.as_vec3().unwrap();
            }
            "baseRightHandPosition" => {
                self.right_hand_position = values.as_vec3().unwrap();
            }
            "baseRightHandRotation" => {
                self.right_hand_rotation = values.as_quat().unwrap();
            }
            _ => panic!("Base provider not found"),
        }
    }

    /// Get or create a ValueProvider for the given base provider name
    ///
    /// This parses the base provider syntax and builds the appropriate provider, caching it for future accesses
    ///
    /// e.g `baseHeadPosition` -> BaseProvider(baseHeadPosition)
    ///     `baseHeadPosition.x` -> PartialProvider(baseHeadPosition, [0])
    ///     `baseHeadPosition.s0_5` -> SmoothProvider(baseHeadPosition, 0.5)
    pub fn get_value_provider(&mut self, base: &str) -> ValueProvider {
        // If we already created a provider for the full key, return it
        // we can avoid string parsing this way for repeated accesses to the same provider
        if let Some(cached) = self.provider_cache.get(base) {
            return cached.clone();
        }

        let provider = self.create_value_provider(base);
        self.provider_cache
            .insert(base.to_string(), provider.clone());

        provider
    }

    /// Creates a ValueProvider for the given base provider name
    fn create_value_provider(&mut self, base: &str) -> ValueProvider {
        let splits: Vec<&str> = base.split('.').collect();
        if splits.is_empty() {
            panic!("empty provider key");
        }

        // Base provider name
        let base_name = splits[0];

        // Quick path: single-name base
        let base_value = ValueProvider::BaseProvider(BaseProviderValues::new(base_name.to_owned()));
        let mut result = match self.get_values(base_name) {
            BaseValue::Quaternion(_) => {
                ValueProvider::QuaternionProvider(QuaternionProviderValues::new(base_value))
            }
            _ => base_value,
        };

        if splits.len() == 1 {
            return result;
        }

        // Start from the base provider value and apply each split part to build the final provider
        // Iterate through dotted parts and build/caches intermediate providers
        for i in 1..splits.len() {
            let split = splits[i];
            let sub_key = splits[0..=i].join(".");

            // we can avoid string parsing
            if let Some(cached) = self.provider_cache.get(&sub_key) {
                result = cached.clone();
                continue;
            }

            let updateable_values = self.handle_split_part(split, &result);

            // If updateable, register it so it will be ticked via `update_providers`
            if updateable_values.is_updateable() {
                self.register_updatable_provider(&updateable_values);
            }

            result = updateable_values;
        }

        result
    }

    pub fn register_updatable_provider(&mut self, provider: &ValueProvider) {
        match provider {
            ValueProvider::SmoothProviders(v) => {
                self.updatable_providers
                    .push(v.clone() as Rc<RefCell<dyn UpdateableValues>>);
            }
            ValueProvider::SmoothRotationProviders(v) => {
                self.updatable_providers
                    .push(v.clone() as Rc<RefCell<dyn UpdateableValues>>);
            }
            _ => {}
        }
    }

    pub fn update_providers(&self, delta: f32) {
        for provider in &self.updatable_providers {
            provider.borrow_mut().update(delta, self);
        }
    }

    fn handle_split_part(&self, split: &str, result: &ValueProvider) -> ValueProvider {
        if split.starts_with('s') {
            return self.create_smooth_provider(result, split);
        }
        // partial swizzle like x/y/z/w
        self.create_partial_provider(result, split)
    }

    /// Build a `PartialProvider` from a swizzle string like "x", "xy", "zw", etc.
    fn create_partial_provider(&self, source: &ValueProvider, swizzle: &str) -> ValueProvider {
        let parts: Vec<usize> = swizzle
            .chars()
            .flat_map(|s| s.to_lowercase())
            .map(|n| match n {
                'x' => 0,
                'y' => 1,
                'z' => 2,
                'w' => 3,
                other => {
                    warn!("invalid swizzle char: {}", other);
                    0
                }
            })
            .collect();

        let src = source.values(self);
        ValueProvider::PartialProvider(crate::providers::partial::PartialProviderValues::new(
            src, parts,
        ))
    }

    /// Build a smoothing provider from a spec like `s1` or `s0_5`.
    fn create_smooth_provider(&self, source: &ValueProvider, spec: &str) -> ValueProvider {
        let rest = spec[1..].replace('_', ".");
        let mult = rest.parse::<f32>();

        let mult = match mult {
            Ok(mult) => mult,
            Err(e) => {
                error!(
                    "Invalid smooth provider specifier: {} due to parse error: {}, defaulting to 1.0",
                    spec, e
                );
                1.0
            }
        };
        match source {
            ValueProvider::QuaternionProvider(qpv) => {
                // clone the underlying source provider so the smooth rotation provider can sample it each update
                let src_provider = (*qpv.source).clone();

                ValueProvider::SmoothRotationProviders(Rc::new(RefCell::new(
                    SmoothRotationProvidersValues::new(src_provider, mult),
                )))
            }
            _ => {
                // pass the source provider (clone) so smooth provider can sample it during updates
                let src_provider = source.clone();
                ValueProvider::SmoothProviders(Rc::new(RefCell::new(SmoothProvidersValues::new(
                    src_provider,
                    mult,
                    self,
                ))))
            }
        }
    }
}
