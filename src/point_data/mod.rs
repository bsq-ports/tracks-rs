pub mod point_data;
pub mod quaternion_point_data;

use glam::{Quat, Vec3, Vec4};

use crate::base_provider_context::BaseProviderContext;
use crate::easings::functions::Functions;
use crate::point_data::point_data::BasicPointData;
use crate::point_data::quaternion_point_data::QuaternionPointData;
use crate::providers::value::BaseValue;

// Generic trait for point data
pub trait PointDataLike<T> {
    fn get_easing(&self) -> Functions;
    fn has_base_provider(&self) -> bool;
    fn get_point(&self, context: &BaseProviderContext) -> T;
    fn get_time(&self) -> f32;
}

#[derive(Debug)]
pub enum BasePointData {
    Float(BasicPointData<f32>),
    Vector3(BasicPointData<Vec3>),
    Vector4(BasicPointData<Vec4>),
    Quaternion(QuaternionPointData),
}

impl BasePointData {
    pub fn get_float(&self, context: &BaseProviderContext) -> f32 {
        match self {
            BasePointData::Float(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a FloatPointData"),
        }
    }

    pub fn get_vector3(&self, context: &BaseProviderContext) -> Vec3 {
        match self {
            BasePointData::Vector3(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a Vector3PointData"),
        }
    }

    pub fn get_vector4(&self, context: &BaseProviderContext) -> Vec4 {
        match self {
            BasePointData::Vector4(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a Vector4PointData"),
        }
    }

    pub fn get_quaternion(&self, context: &BaseProviderContext) -> Quat {
        match self {
            BasePointData::Quaternion(point_data) => point_data.get_point(context),
            _ => panic!("PointData is not a QuaternionPointData"),
        }
    }
}

impl PointDataLike<BaseValue> for BasePointData {
    fn get_easing(&self) -> Functions {
        match self {
            BasePointData::Float(point_data) => point_data.get_easing(),
            BasePointData::Vector3(point_data) => point_data.get_easing(),
            BasePointData::Vector4(point_data) => point_data.get_easing(),
            BasePointData::Quaternion(point_data) => point_data.get_easing(),
        }
    }

    fn has_base_provider(&self) -> bool {
        match self {
            BasePointData::Float(point_data) => point_data.has_base_provider(),
            BasePointData::Vector3(point_data) => point_data.has_base_provider(),
            BasePointData::Vector4(point_data) => point_data.has_base_provider(),
            BasePointData::Quaternion(point_data) => point_data.has_base_provider(),
        }
    }

    fn get_point(&self, context: &BaseProviderContext) -> BaseValue {
        match self {
            BasePointData::Float(point_data) => BaseValue::Float(point_data.get_point(context)),
            BasePointData::Vector3(point_data) => BaseValue::Vector3(point_data.get_point(context)),
            BasePointData::Vector4(point_data) => BaseValue::Vector4(point_data.get_point(context)),
            BasePointData::Quaternion(point_data) => {
                BaseValue::Quaternion(point_data.get_point(context))
            }
        }
    }

    fn get_time(&self) -> f32 {
        match self {
            BasePointData::Float(point_data) => point_data.get_time(),
            BasePointData::Vector3(point_data) => point_data.get_time(),
            BasePointData::Vector4(point_data) => point_data.get_time(),
            BasePointData::Quaternion(point_data) => point_data.get_time(),
        }
    }
}
