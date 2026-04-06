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
    type Modifer = BasicModifier<T>;
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
        let val = match values.as_slice() {
            // Single static value
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 1 => {
                ModifierValues::Static(static_val.values(context))
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, T::VALUE_COUNT, "modifier point must have {} numbers", T::VALUE_COUNT);
                ModifierValues::Dynamic(values)
            }
        };
        Self::Modifer::new(val, modifiers, operation)
    }

    fn create_point_data(
        values: Vec<ValueProvider>,
        _flags: Vec<String>,
        modifiers: Vec<Self::Modifer>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Self::PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.

        let (value, time) = match &values[..] {
            // [x, y]
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 2 => {
                let values = static_val.values(context);
                let point = &values[0..T::VALUE_COUNT - 1];
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

    fn interpolate_points(
        &self,
        points: &[Self::PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> T {
        let point_l = points[l].get_point(context);
        let point_r = points[r].get_point(context);

        T::lerp(point_l, point_r, time)
    }

    fn get_points(&self) -> &[Self::PointData] {
        &self.points
    }

    fn get_point(&self, point: &Self::PointData, context: &BaseProviderContext) -> T {
        point.get_point(context)
    }

    fn get_type(&self) -> crate::ffi::types::WrapBaseValueType {
        crate::ffi::types::WrapBaseValueType::Float
    }
}
