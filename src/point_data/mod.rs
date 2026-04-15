pub mod base_point_data;
pub mod basic_point_data;
pub mod quaternion_point_data;

use crate::base_provider_context::BaseProviderContext;
use crate::easings::functions::Functions;

// Generic trait for point data
pub trait PointDataLike<T>: Clone {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self, context: &BaseProviderContext) -> T;
    fn get_time(&self) -> f32;
}
