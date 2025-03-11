use glam::{Quat, Vec3, Vec4};

use crate::point_definition::{BasePointDefinition, PointDefinition};

pub enum Property {
    Float(f32),
    Vec3(Vec3),
    Vec4(Vec4),
    Quat(Quat),
    None,
}
impl Property {
    fn set_null(&mut self) {
        *self = Property::None;
    }
}

pub struct PathProperty {
    pub time: f32,
    pub prev_point: Option<BasePointDefinition>,
    pub point: Option<BasePointDefinition>,
}

impl PathProperty {
    pub fn finish(&mut self) {
        self.prev_point = None;
    }

    pub fn init(&mut self, new_point_data: Option<BasePointDefinition>) {
        self.time = 0.0;
        self.prev_point = self.point.take();
        self.point = new_point_data;
    }
}

pub enum BaseProperty {
    Property(Property),
    PathProperty(PathProperty),
}
impl BaseProperty {
    pub(crate) fn set_null(&mut self) {
        match self {
            BaseProperty::Property(property) => property.set_null(),
            BaseProperty::PathProperty(path_property) => path_property.init(None),
        }
    }
}
