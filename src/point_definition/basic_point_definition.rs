use std::rc::Rc;

use itertools::Itertools;

use crate::{
    base_provider_context::BaseProviderContext,
    base_value::WrapBaseValueType,
    easings::functions::Functions,
    modifiers::{ModifierValues, modifier::BasicModifier, operation::Operation},
    point_data::{PointDataLike, basic_point_data::BasicPointData},
    prelude::{AbstractValueProvider, ValueProvider},
    value_types::ValueType,
};

use super::PointDefinitionLike;

#[derive(Default, Debug, Clone)]
pub struct BasicPointDefinition<T: ValueType> {
    points: Rc<[BasicPointData<T>]>,
}

impl<T: ValueType> PointDefinitionLike<T> for BasicPointDefinition<T>
where
    [f32; T::VALUE_COUNT + 1]: smallvec::Array,
{
    type Modifier = BasicModifier<T>;
    type PointData = BasicPointData<T>;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points
            .iter()
            .any(|p| PointDataLike::has_base_provider(p))
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
            // Single static value [T, time]
            [ValueProvider::Static(static_val)]
                if static_val.values.len() <= T::VALUE_COUNT + 1 =>
            {
                let values = &static_val.values;
                ModifierValues::Static(T::from_slice(values))
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(
                    count,
                    T::VALUE_COUNT + 1,
                    "modifier point must have {} numbers",
                    T::VALUE_COUNT + 1
                );
                ModifierValues::Dynamic(values)
            }
        };
        Self::Modifier::new(val, modifiers, operation)
    }

    fn create_point_data(
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Self::Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Self::PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.

        let (value, time) = match &values[..] {
            // [x, ..., y]
            [ValueProvider::Static(static_val)] => {
                let values = static_val.values(context);
                let point = T::from_slice(&values[0..T::VALUE_COUNT]);
                let time = values[T::VALUE_COUNT];
                (ModifierValues::Static(point), time)
            }

            _ => {
                // validate and get time
                let collected_values = values
                    .iter()
                    .map(|v| v.values(context))
                    .fold_while([0.0; T::VALUE_COUNT + 1], |mut acc, v| {
                        for (i, val) in v.iter().enumerate() {
                            if i < T::VALUE_COUNT {
                                acc[i] = *val;
                            }
                        }
                        itertools::FoldWhile::Continue(acc)
                    })
                    .into_inner();

                // TODO: I don't know if this is the best way to get time
                // or if we should collect all values then get time from the last value provider
                let time = collected_values[T::VALUE_COUNT];

                (ModifierValues::Dynamic(values), time)
            }
        };

        let smooth = flags.iter().any(|f| f == "splineCatmullRom");

        BasicPointData::new(value, time, smooth, modifiers, easing)
    }

    fn get_points(&self) -> &[Self::PointData] {
        &self.points
    }

    fn get_type(&self) -> WrapBaseValueType {
        T::base_type()
    }

    fn interpolate_points(
        &self,
        l: &Self::PointData,
        r: &Self::PointData,
        _l_index: usize,
        _r_index: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> T {
        let point_l = PointDataLike::get_point(l, context);
        let point_r = PointDataLike::get_point(r, context);

        T::value_lerp(point_l, point_r, time)
    }
}
