use std::rc::Rc;

use glam::FloatExt;

use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{ModifierLike, ModifierValues, modifier::BasicModifier, operation::Operation},
    point_data::{BasePointData, PointDataLike, point_data::BasicPointData},
    prelude::{AbstractValueProvider, ValueProvider},
    values::ValueType,
};

use super::PointDefinitionLike;

#[derive(Default, Debug, Clone)]
pub struct BasicPointDefinition<T: ValueType> {
    points: Rc<[BasicPointData<T>]>,
}

impl<T: ValueType> PointDefinitionLike for BasicPointDefinition<T> {
    type Value = T;
    type Modifier = BasicModifier<T>;
    type PointData = BasicPointData<T>;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| PointDataLike::has_base_provider(p))
    }

    fn new(points: Vec<Self::PointData>) -> Self {
        Self {
            points: Rc::from(points),
        }
    }

    fn create_modifier(
        values: Vec<ValueProvider>,
        modifiers: Vec<BasicModifier<T>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> BasicModifier<T> {
        let val: ModifierValues<T> = match values.as_slice() {
            // Single static value
            [ValueProvider::Static(static_val)] if static_val.values(context).len() <= T::VALUE_COUNT => {
                let values = static_val.values(context);
                ModifierValues::Static(T::from_slice(&values))
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, T::VALUE_COUNT, "modifier point must have {} numbers", T::VALUE_COUNT);
                ModifierValues::Dynamic(values)
            }
        };
        Self::Modifier::new(val, modifiers, operation)
    }

    fn create_point_data(
        values: Vec<ValueProvider>,
        _flags: Vec<String>,
        modifiers: Vec<Self::Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Self::PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.

        let (value, time) = match &values[..] {
            // [x, y]
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 2 => {
                let values = static_val.values(context);
                let point = T::from_slice(&values[0..T::VALUE_COUNT - 1]);
                (ModifierValues::Static(point), values[T::VALUE_COUNT])
            }

            _ => {
                // validate and get time
                let values_len: usize = values.iter().map(|v| v.values(context).len()).sum();

                let time = if values_len == 2 {
                    values
                        .last()
                        .and_then(|v| v.values(context).last().copied())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };

                (ModifierValues::Dynamic(values), time)
            }
        };

        BasicPointData::new(value, time, modifiers, easing)
    }


    fn get_points(&self) -> &[Self::PointData] {
        &self.points
    }

    fn get_point(&self, point: &Self::PointData, context: &BaseProviderContext) -> T {
        PointDataLike::get_point(point, context)
    }

    fn get_type(&self) -> crate::ffi::types::WrapBaseValueType {
        crate::ffi::types::WrapBaseValueType::Float
    }
    
    fn interpolate_points(
        &self,
        l: &Self::PointData,
        r: &Self::PointData,
        time: f32,
        context: &BaseProviderContext,
    ) -> Self::Value {
        let point_l = PointDataLike::get_point(l, context);
        let point_r = PointDataLike::get_point(r, context);

        T::value_lerp(point_l, point_r, time)
    }
}
