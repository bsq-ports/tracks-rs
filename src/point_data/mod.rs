pub mod base_point_data;
pub mod basic_point_data;
pub mod quaternion_point_data;

use glam::{Quat, Vec3, Vec4, usize};

use crate::base_provider_context::BaseProviderContext;
use crate::easings::functions::Functions;
use crate::point_data::basic_point_data::BasicPointData;
use crate::point_data::quaternion_point_data::QuaternionPointData;
use crate::providers::value::BaseValue;

// Generic trait for point data
pub trait PointDataLike<T>: Clone {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self, context: &BaseProviderContext) -> T;
    fn get_time(&self) -> f32;
}
