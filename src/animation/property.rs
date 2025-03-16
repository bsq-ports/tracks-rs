use std::{cell::RefCell, rc::Rc};

use crate::{
    point_definition::{
        PointDefinition, base_point_definition::BasePointDefinition,
        point_definition_interpolation::PointDefinitionInterpolation,
    },
    values::value::BaseValue,
};

pub type ValueProperty = Option<BaseValue>;
pub type PathProperty<'a> = PointDefinitionInterpolation<'a>;
