
use crate::{
    point_definition::point_definition_interpolation::PointDefinitionInterpolation,
    values::value::BaseValue,
};

pub type ValueProperty = Option<BaseValue>;
pub type PathProperty<'a> = PointDefinitionInterpolation<'a>;
