use glam::Vec4;
use smallvec::SmallVec;

use crate::easings::functions::Functions;

use crate::modifiers::base_modifier::BaseModifier;
use crate::modifiers::operation::Operation;

use crate::point_data::base_point_data::BasePointData;
use crate::point_definition::basic_point_definition::BasicPointDefinition;
use crate::point_definition::quaternion_point_definition::QuaternionPointDefinition;
use crate::point_definition::vector3_point_definition::Vector3PointDefinition;
use crate::providers::ValueProvider;

use crate::base_provider_context::BaseProviderContext;

use crate::base_value::{BaseValue, WrapBaseValueType};

use super::PointDefinitionLike;

/// Point definitions are used to describe what happens over the course of an animation,
/// they are used slightly differently for different properties.
/// They consist of a collection of points over time.
///
/// Cloning is cheap because the underlying data is reference counted, so you can clone a point definition to use it in multiple places without copying the underlying data.
#[derive(Debug, Clone)]
pub enum BasePointDefinition {
    Float(BasicPointDefinition<f32>),
    Vector3(Vector3PointDefinition),
    Vector4(BasicPointDefinition<Vec4>),
    Quaternion(QuaternionPointDefinition),
}

impl PointDefinitionLike<BaseValue> for BasePointDefinition {
    type PointData = BasePointData;
    type Modifier = BaseModifier;

    fn interpolate(&self, time: f32, context: &BaseProviderContext) -> (BaseValue, bool) {
        match self {
            BasePointDefinition::Float(def) => {
                let (v, done) = def.interpolate(time, context);
                (BaseValue::Float(v), done)
            }
            BasePointDefinition::Vector3(def) => {
                let (v, done) = def.interpolate(time, context);
                (BaseValue::Vector3(v), done)
            }
            BasePointDefinition::Vector4(def) => {
                let (v, done) = def.interpolate(time, context);
                (BaseValue::Vector4(v), done)
            }
            BasePointDefinition::Quaternion(def) => {
                let (v, done) = def.interpolate(time, context);
                (BaseValue::Quaternion(v), done)
            }
        }
    }

    fn get_count(&self) -> usize {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.get_count()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.get_count()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.get_count()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.get_count()
            }
        }
    }

    fn has_base_provider(&self) -> bool {
        match self {
            BasePointDefinition::Float(float_point_definition) => {
                float_point_definition.has_base_provider()
            }
            BasePointDefinition::Vector3(vector3_point_definition) => {
                vector3_point_definition.has_base_provider()
            }
            BasePointDefinition::Vector4(vector4_point_definition) => {
                vector4_point_definition.has_base_provider()
            }
            BasePointDefinition::Quaternion(quaternion_point_definition) => {
                quaternion_point_definition.has_base_provider()
            }
        }
    }

    fn create_modifier(
        _values: SmallVec<[ValueProvider; 1]>,
        _modifiers: Vec<BaseModifier>,
        _operation: Operation,
        _context: &BaseProviderContext,
    ) -> BaseModifier {
        unimplemented!(
            "Cannot create Modifier directly from BasePointDefinition; use specific point definition types instead."
        )
    }

    fn create_point_data(
        _values: SmallVec<[ValueProvider; 1]>,
        _flags: Vec<String>,
        _modifiers: Vec<BaseModifier>,
        _easing: Functions,
        _context: &BaseProviderContext,
    ) -> BasePointData {
        unimplemented!(
            "Cannot create PointData directly from BasePointDefinition; use specific point definition types instead."
        )
    }

    fn get_points(&self) -> &[BasePointData] {
        // BasePointDefinition stores typed point definitions internally, so there is no
        // zero-copy way to expose a unified &[BasePointData] view here.
        // This type provides its own interpolate() implementation, so default
        // PointDefinitionLike::interpolate() should never call this.
        panic!(
            "BasePointDefinition::get_points is unsupported; use BasePointDefinition::interpolate/get_count"
        )
    }

    fn get_type(&self) -> WrapBaseValueType {
        match self {
            BasePointDefinition::Float(_) => WrapBaseValueType::Float,
            BasePointDefinition::Vector3(_) => WrapBaseValueType::Vec3,
            BasePointDefinition::Vector4(_) => WrapBaseValueType::Vec4,
            BasePointDefinition::Quaternion(_) => WrapBaseValueType::Quat,
        }
    }

    fn new(_points: Vec<BasePointData>) -> Self {
        unimplemented!(
            "Cannot create BasePointDefinition directly; use specific point definition types instead."
        )
    }

    fn interpolate_points(
        &self,
        l: &Self::PointData,
        r: &Self::PointData,
        l_index: usize,
        r_index: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> BaseValue {
        match (self, l, r) {
            (
                BasePointDefinition::Float(float_point_definition),
                BasePointData::Float(l_data),
                BasePointData::Float(r_data),
            ) => BaseValue::Float(
                float_point_definition
                    .interpolate_points(l_data, r_data, l_index, r_index, time, context),
            ),
            (
                BasePointDefinition::Vector3(vector3_point_definition),
                BasePointData::Vector3(l_data),
                BasePointData::Vector3(r_data),
            ) => BaseValue::Vector3(
                vector3_point_definition
                    .interpolate_points(l_data, r_data, l_index, r_index, time, context),
            ),
            (
                BasePointDefinition::Vector4(vector4_point_definition),
                BasePointData::Vector4(l_data),
                BasePointData::Vector4(r_data),
            ) => BaseValue::Vector4(
                vector4_point_definition
                    .interpolate_points(l_data, r_data, l_index, r_index, time, context),
            ),
            (
                BasePointDefinition::Quaternion(quaternion_point_definition),
                BasePointData::Quaternion(l_data),
                BasePointData::Quaternion(r_data),
            ) => BaseValue::Quaternion(
                quaternion_point_definition
                    .interpolate_points(l_data, r_data, l_index, r_index, time, context),
            ),
            _ => panic!(
                "Mismatched PointDefinition and PointData types during interpolation {:?} {:?} {:?}",
                self, l, r
            ),
        }
    }
}

impl Default for BasePointDefinition {
    fn default() -> Self {
        BasePointDefinition::Float(Default::default())
    }
}

impl From<QuaternionPointDefinition> for BasePointDefinition {
    fn from(point_definition: QuaternionPointDefinition) -> Self {
        BasePointDefinition::Quaternion(point_definition)
    }
}

impl From<Vector3PointDefinition> for BasePointDefinition {
    fn from(point_definition: Vector3PointDefinition) -> Self {
        BasePointDefinition::Vector3(point_definition)
    }
}

impl From<BasicPointDefinition<Vec4>> for BasePointDefinition {
    fn from(point_definition: BasicPointDefinition<Vec4>) -> Self {
        BasePointDefinition::Vector4(point_definition)
    }
}

impl From<BasicPointDefinition<f32>> for BasePointDefinition {
    fn from(point_definition: BasicPointDefinition<f32>) -> Self {
        BasePointDefinition::Float(point_definition)
    }
}

impl BasePointDefinition {
    pub fn value_type(&self) -> WrapBaseValueType {
        match self {
            BasePointDefinition::Float(_) => WrapBaseValueType::Float,
            BasePointDefinition::Vector3(_) => WrapBaseValueType::Vec3,
            BasePointDefinition::Vector4(_) => WrapBaseValueType::Vec4,
            BasePointDefinition::Quaternion(_) => WrapBaseValueType::Quat,
        }
    }
}
#[cfg(feature = "json")]
mod json {
    use ::serde::de::Error as DeError;
    use ahash::HashMap;
    use serde_json::Value as JsonValue;

    use super::BasePointDefinition;
    use crate::{
        base_provider_context::BaseProviderContext,
        base_value::WrapBaseValueType,
        point_definition::{
            PointDefinitionLike, basic_point_definition::BasicPointDefinition,
            quaternion_point_definition::QuaternionPointDefinition,
            vector3_point_definition::Vector3PointDefinition,
        },
    };

    impl BasePointDefinition {
        /// Deserialize a non-null point definition as written in a map, as value type `ty`.
        ///
        /// * A string names a raw JSON definition in `named_point_defs`, parsed on demand as `ty`.
        /// * An array is parsed inline as `ty`.
        /// * `null` is an error; callers handle erasing a value themselves.
        ///
        /// # Examples
        ///
        /// Inline array, parsed as a `Vec3` (`[x, y, z, time]` per point):
        ///
        /// ```json
        /// [[0, 40, 0, 0], [0, 0, 0, 0.2]]
        /// ```
        ///
        /// Named definition: the value is the name, and `named_point_defs` holds the raw
        /// array under that key:
        ///
        /// ```json
        /// "ZigZagPosition"
        /// ```
        ///
        /// ```json
        /// { "ZigZagPosition": [[0, 0, 0, 0], [5, 0, 0, 0.25], [-5, 0, 0, 0.75]] }
        /// ```
        ///
        /// Rust:
        ///
        /// ```ignore
        /// let def = BasePointDefinition::deserialize(
        ///     json!([[0, 40, 0, 0], [0, 0, 0, 0.2]]),
        ///     WrapBaseValueType::Vec3,
        ///     &mut context,
        ///     &HashMap::default(),
        /// )?;
        /// ```
        pub fn deserialize(
            value: JsonValue,
            ty: WrapBaseValueType,
            context: &mut BaseProviderContext,
            named_point_defs: &HashMap<String, JsonValue>,
        ) -> Result<Self, serde_json::Error> {
            match value {
                JsonValue::String(name) => {
                    let raw = named_point_defs.get(&name).ok_or_else(|| {
                        serde_json::Error::custom(format!(
                            "named point definition '{name}' not found"
                        ))
                    })?;
                    Self::parse_as(raw.clone(), ty, context).map_err(serde_json::Error::custom)
                }
                array @ JsonValue::Array(_) => {
                    Self::parse_as(array, ty, context).map_err(serde_json::Error::custom)
                }
                _ => Err(serde_json::Error::custom(
                    "point definition must be an array or a named point definition",
                )),
            }
        }

        /// Parse an inline array of points as the given value type.
        pub fn parse_as(
            value: JsonValue,
            ty: WrapBaseValueType,
            context: &mut BaseProviderContext,
        ) -> Result<Self, String> {
            if value.as_array().is_none_or(Vec::is_empty) {
                return Err("point definition cannot be an empty array".to_owned());
            }

            Ok(match ty {
                WrapBaseValueType::Float => {
                    BasicPointDefinition::<f32>::parse(value, context).into()
                }
                WrapBaseValueType::Vec3 => Vector3PointDefinition::parse(value, context).into(),
                WrapBaseValueType::Vec4 => {
                    BasicPointDefinition::<glam::Vec4>::parse(value, context).into()
                }
                WrapBaseValueType::Quat => QuaternionPointDefinition::parse(value, context).into(),
                WrapBaseValueType::Unknown => {
                    return Err("point definition value type cannot be unknown".to_owned());
                }
            })
        }
    }
}
