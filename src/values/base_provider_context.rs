use std::borrow::Borrow;

use glam::{Quat, Vec3, Vec4};
use tracing::info;

use crate::modifiers::quaternion_modifier::QuaternionValues;

use super::{
    AbstractValueProvider, ValueProvider,
    base::BaseProviderValues,
    quat::QuaternionProviderValues,
    value::{BaseValue, BaseValueRef},
};

#[derive(Default)]
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
}

impl BaseProviderContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_values<'a>(&'a self, base: &str) -> BaseValueRef<'a> {
        match base {
            "baseCombo" => self.base_combo.borrow().into(),

            "baseMultipliedScore" => self.multiplied_score.borrow().into(),
            "baseImmediateMaxPossibleMultipliedScore" => {
                self.immediate_max_possible_multiplied_score.borrow().into()
            }

            "baseModifiedScore" => self.modified_score.borrow().into(),
            "baseImmediateMaxPossibleModifiedScore" => {
                self.immediate_max_possible_modified_score.borrow().into()
            }
            "baseRelativeScore" => self.relative_score.borrow().into(),
            "baseMultiplier" => self.multiplier.borrow().into(),
            "baseEnergy" => self.energy.borrow().into(),
            "baseSongTime" => self.song_time.borrow().into(),
            "baseSongLength" => self.song_length.borrow().into(),

            "baseEnvironmentColor0" => self.environment_color_0.borrow().into(),
            "baseEnvironmentColor0Boost" => self.environment_color_0_boost.borrow().into(),
            "baseEnvironmentColor1" => self.environment_color_1.borrow().into(),
            "baseEnvironmentColor1Boost" => self.environment_color_1_boost.borrow().into(),
            "baseEnvironmentColorW" => self.environment_color_w.borrow().into(),
            "baseEnvironmentColorWBoost" => self.environment_color_w_boost.borrow().into(),
            "baseNote0Color" => self.note_color_0.borrow().into(),
            "baseNote1Color" => self.note_color_1.borrow().into(),
            "baseObstaclesColor" => self.obstacles_color.borrow().into(),
            "baseSaberAColor" => self.saber_color_a.borrow().into(),
            "baseSaberBColor" => self.saber_color_b.borrow().into(),

            "baseHeadLocalPosition" => self.head_local_position.borrow().into(),
            "baseHeadLocalRotation" => self.head_local_rotation.borrow().into(),
            "baseHeadLocalScale" => self.head_local_scale.borrow().into(),
            "baseHeadPosition" => self.head_position.borrow().into(),
            "baseHeadRotation" => self.head_rotation.borrow().into(),
            "baseLeftHandLocalPosition" => self.left_hand_local_position.borrow().into(),
            "baseLeftHandLocalRotation" => self.left_hand_local_rotation.borrow().into(),
            "baseLeftHandLocalScale" => self.left_hand_local_scale.borrow().into(),
            "baseLeftHandPosition" => self.left_hand_position.borrow().into(),
            "baseLeftHandRotation" => self.left_hand_rotation.borrow().into(),
            "baseRightHandLocalPosition" => self.right_hand_local_position.borrow().into(),
            "baseRightHandLocalRotation" => self.right_hand_local_rotation.borrow().into(),
            "baseRightHandLocalScale" => self.right_hand_local_scale.borrow().into(),
            "baseRightHandPosition" => self.right_hand_position.borrow().into(),
            "baseRightHandRotation" => self.right_hand_rotation.borrow().into(),
            _ => panic!("Base provider not found {}", base),
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

    pub fn get_value_provider(&self, base: &str) -> ValueProvider {
        let split_base = base.split(".").collect::<Vec<&str>>();
        let base_name = split_base[0];

        let base_value = ValueProvider::BaseProvider(BaseProviderValues::new(base_name.to_owned()));
        let base_value: ValueProvider = match self.get_values(base_name) {
            BaseValueRef::Quaternion(_) => {
                info!("Quaternion provider");
                ValueProvider::QuaternionProvider(QuaternionProviderValues::new(base_value))
            }
            _ => base_value,
        };
        base_value
    }
}
