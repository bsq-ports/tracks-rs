pub mod float_point_data;
pub mod quaternion_point_data;
pub mod vector3_point_data;
pub mod vector4_point_data;

use float_point_data::FloatPointData;
use glam::{Quat, Vec3, Vec4};
use quaternion_point_data::QuaternionPointData;
use vector3_point_data::Vector3PointData;
use vector4_point_data::Vector4PointData;

use crate::easings::functions::Functions;
use crate::base_provider_context::BaseProviderContext;

pub enum PointData {
    Float(FloatPointData),
    Vector3(Vector3PointData),
    Vector4(Vector4PointData),
    Quaternion(QuaternionPointData),
}

impl PointData {
    pub fn get_easing(&self) -> Functions {
        match self {
            PointData::Float(point_data) => point_data.get_easing(),
            PointData::Vector3(point_data) => point_data.get_easing(),
            PointData::Vector4(point_data) => point_data.get_easing(),
            PointData::Quaternion(point_data) => point_data.get_easing(),
        }
    }

    pub fn get_time(&self) -> f32 {
        match self {
            PointData::Float(point_data) => point_data.get_time(),
            PointData::Vector3(point_data) => point_data.get_time(),
            PointData::Vector4(point_data) => point_data.get_time(),
            PointData::Quaternion(point_data) => point_data.get_time(),
        }
    }

    pub fn has_base_provider(&self) -> bool {
        match self {
            PointData::Float(point_data) => point_data.has_base_provider(),
            PointData::Vector3(point_data) => point_data.has_base_provider(),
            PointData::Vector4(point_data) => point_data.has_base_provider(),
            PointData::Quaternion(point_data) => point_data.has_base_provider(),
        }
    }

    pub fn get_float(&self, context: &BaseProviderContext) -> f32 {
        match self {
            PointData::Float(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a FloatPointData"),
        }
    }

    pub fn get_vector3(&self, context: &BaseProviderContext) -> Vec3 {
        match self {
            PointData::Vector3(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a Vector3PointData"),
        }
    }

    pub fn get_vector4(&self, context: &BaseProviderContext) -> Vec4 {
        match self {
            PointData::Vector4(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a Vector4PointData"),
        }
    }

    pub fn get_quaternion(&self, context: &BaseProviderContext) -> Quat {
        match self {
            PointData::Quaternion(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a QuaternionPointData"),
        }
    }
}

// Generic trait for point data
pub trait BasePointData<T> {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self, context: &BaseProviderContext) -> T;
    fn get_time(&self) -> f32;
}
