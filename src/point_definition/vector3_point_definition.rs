use std::rc::Rc;

use glam::{Vec3, Vec3A};
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
pub struct Vector3PointDefinition {
    points: Rc<[BasicPointData<Vec3>]>,
}

impl Vector3PointDefinition {
    fn smooth_vector_lerp(
        &self,

        l_0: Vec3A,
        l_sub_1: Option<Vec3A>,
        r_0: Vec3A,
        r_add_1: Option<Vec3A>,

        time: f32,
        _context: &BaseProviderContext,
    ) -> Vec3 {
        // Convert to Vec3A for SIMD-friendly spline interpolation, convert back at the end
        // let point_a_a = glam::Vec3A::from(points[l].get_point(context));
        // let point_b_a = glam::Vec3A::from(points[r].get_point(context));

        // Catmull-Rom Spline
        // let p0_a = if l > 0 {
        //     glam::Vec3A::from(points[l - 1].get_point(context))
        // } else {
        //     point_a_a
        // };
        // let p3_a = if r + 1 < len {
        // // let p3_a = if r + 1 < points.len() {
        //     glam::Vec3A::from(points[r + 1].get_point(context))
        // } else {
        //     point_b_a
        // };

        let point_a_a = l_0;
        let point_b_a = r_0;
        let p0_a = l_sub_1.unwrap_or(l_0);
        let p3_a = r_add_1.unwrap_or(r_0);

        let tt = time * time;
        let ttt = tt * time;

        let q0 = -ttt + (2.0 * tt) - time;
        let q1 = (3.0 * ttt) - (5.0 * tt) + 2.0;
        let q2 = (-3.0 * ttt) + (4.0 * tt) + time;
        let q3 = ttt - tt;

        let res_a = 0.5 * ((p0_a * q0) + (point_a_a * q1) + (point_b_a * q2) + (p3_a * q3));
        Vec3::from(res_a)
    }
}

impl PointDefinitionLike<Vec3> for Vector3PointDefinition {
    type Modifier = BasicModifier<Vec3>;
    type PointData = BasicPointData<Vec3>;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(PointDataLike::has_base_provider)
    }

    fn new(points: Vec<Self::PointData>) -> Self {
        Self {
            points: Rc::from(points),
        }
    }

    fn create_modifier(
        values: Vec<ValueProvider>,
        modifiers: Vec<BasicModifier<Vec3>>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> BasicModifier<Vec3> {
        let val: ModifierValues<Vec3> = match values.as_slice() {
            // Single static value [T, time]
            [ValueProvider::Static(static_val)]
                if static_val.values.len() <= Vec3::VALUE_COUNT + 1 =>
            {
                let values = &static_val.values;
                ModifierValues::Static(Vec3::from_slice(values))
            }
            // Any other case
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(
                    count,
                    Vec3::VALUE_COUNT + 1,
                    "modifier point must have {} numbers",
                    Vec3::VALUE_COUNT + 1
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
                let point = Vec3::from_slice(&values[0..Vec3::VALUE_COUNT]);
                let time = values[Vec3::VALUE_COUNT];
                (ModifierValues::Static(point), time)
            }

            _ => {
                // validate and get time
                let collected_values = values
                    .iter()
                    .map(|v| v.values(context))
                    .fold_while([0.0; Vec3::VALUE_COUNT + 1], |mut acc, v| {
                        for (i, val) in v.iter().enumerate() {
                            if i < Vec3::VALUE_COUNT {
                                acc[i] = *val;
                            }
                        }
                        itertools::FoldWhile::Continue(acc)
                    })
                    .into_inner();

                // TODO: I don't know if this is the best way to get time
                // or if we should collect all values then get time from the last value provider
                let time = collected_values[Vec3::VALUE_COUNT];

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
        Vec3::base_type()
    }

    fn interpolate_points(
        &self,
        l: &Self::PointData,
        r: &Self::PointData,
        l_index: usize,
        r_index: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Vec3 {
        if r.smooth {
            let point_a_a = PointDataLike::get_point(l, context);
            let point_b_a = PointDataLike::get_point(r, context);
            let l_sub_1 = self
                .points
                .get(l_index - 1)
                .map(|p| p.get_point(context))
                .map(Vec3A::from);
            let r_add_1 = self
                .points
                .get(r_index + 1)
                .map(|p| p.get_point(context))
                .map(Vec3A::from);

            return self.smooth_vector_lerp(
                point_a_a.into(),
                l_sub_1,
                point_b_a.into(),
                r_add_1,
                time,
                context,
            );
        }

        let point_l = PointDataLike::get_point(l, context);
        let point_r = PointDataLike::get_point(r, context);

        Vec3::value_lerp(point_l, point_r, time)
    }
}
