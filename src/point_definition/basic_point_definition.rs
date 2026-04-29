use std::rc::Rc;

use smallvec::SmallVec;

use crate::{
    base_provider_context::BaseProviderContext,
    base_value::WrapBaseValueType,
    easings::functions::Functions,
    modifiers::{ModifierValues, basic_modifier::BasicModifier, operation::Operation},
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
        values: SmallVec<[ValueProvider; 1]>,
        modifiers: Vec<BasicModifier<T>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> BasicModifier<T> {
        let val: ModifierValues<T> = match values.as_slice() {
            // Single static value [T]
            [ValueProvider::Static(static_val)] if static_val.values.len() == T::VALUE_COUNT => {
                let values = &static_val.values;
                ModifierValues::Static(T::from_slice(values))
            }
            // Any other case is treated as dynamic and translated/padded at evaluation time.
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(
                    count,
                    T::VALUE_COUNT,
                    "modifier point must have {} numbers",
                    T::VALUE_COUNT
                );
                ModifierValues::Dynamic(values)
            }
        };
        Self::Modifier::new(val, modifiers, operation)
    }

    fn create_point_data(
        values: SmallVec<[ValueProvider; 1]>,
        flags: Vec<String>,
        modifiers: Vec<Self::Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> Self::PointData {
        // If one value is present and it contains two floats, the first is the point value and the second is time.

        let (value, time) = match &values[..] {
            // [x, ..., y]
            [ValueProvider::Static(static_val)]
                if static_val.values.len() == T::VALUE_COUNT + 1 =>
            {
                let values = &static_val.values;
                let point = T::from_slice(&values[0..T::VALUE_COUNT]);
                let time = values[T::VALUE_COUNT];
                (ModifierValues::Static(point), time)
            }

            _ => {
                // get time from last provider last value
                // https://github.com/Aeroluna/Heck/blob/1dc9f470a7f8d3e64d0e3bc34e2f2279190eb8b8/Heck/Animation/PointDefinition/Vector3PointDefinition.cs#L80-L81
                let time = values.last()
                    .and_then(|vp| vp.values(context).last().copied())
                    .unwrap_or_else(|| panic!("Expected at least one value provider with at least {} values for point data, with the last one being time",
                        T::VALUE_COUNT + 1));

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
