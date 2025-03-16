use glam::{FloatExt, Vec4};
use palette::{Hsv, IntoColor, LinSrgb, RgbHue, rgb::Rgb};

use crate::{
    base_provider_context::BaseProviderContext,
    easings::functions::Functions,
    modifiers::{
        Modifier,
        operation::Operation,
        vector4_modifier::{Vector4Modifier, Vector4Values},
    },
    point_data::{PointData, vector4_point_data::Vector4PointData},
    values::{AbstractValueProvider, ValueProvider},
};

use super::PointDefinition;

#[derive(Default)]
pub struct Vector4PointDefinition {
    points: Vec<PointData>,
}

pub fn lerp_hsv_vec4(color1: Vec4, color2: Vec4, time: f32) -> Vec4 {
    // Convert RGBA to HSV
    let hsv1: Hsv<f32> = Rgb::new(color1.x, color1.y, color1.z).into_color();
    let hsv2: Hsv<f32> = Rgb::new(color2.x, color2.y, color2.z).into_color();

    // Lerp HSV components
    let h = RgbHue::from_radians(
        hsv1.hue
            .into_raw_radians()
            .lerp(hsv2.hue.into_raw_radians(), time),
    );
    let s = hsv1.saturation.lerp(hsv2.saturation, time);
    let v = hsv1.value.lerp(hsv2.value, time);

    // Convert back to RGB
    let rgb: LinSrgb<f32> = Hsv::new(h, s, v).into_color();

    // Lerp alpha
    let alpha = color1.w * (1.0 - time) + color2.w * time;

    // Return the new Vec4
    Vec4::new(rgb.red, rgb.green, rgb.blue, alpha)
}

impl PointDefinition for Vector4PointDefinition {
    type Value = Vec4;

    fn get_count(&self) -> usize {
        self.points.len()
    }

    fn has_base_provider(&self) -> bool {
        self.points.iter().any(|p| p.has_base_provider())
    }

    fn get_points_mut(&mut self) -> &mut Vec<PointData> {
        &mut self.points
    }

    fn create_modifier(
        &self,
        values: Vec<ValueProvider>,
        modifiers: Vec<Modifier>,
        operation: Operation,
        context: &BaseProviderContext,
    ) -> Modifier {
        let values = match values.as_slice() {
            [ValueProvider::Static(static_val)] if static_val.values.len() == 4 => {
                let vec4 = Vec4::new(
                    static_val.values(context)[0],
                    static_val.values(context)[1],
                    static_val.values(context)[2],
                    static_val.values(context)[3],
                );

                Vector4Values::Static(vec4)
            }
            _ => {
                let count: usize = values.iter().map(|v| v.values(context).len()).sum();
                assert_eq!(count, 4, "Vector4 modifier point must have 4 numbers");
                Vector4Values::Dynamic(values)
            }
        };

        Modifier::Vector4(Vector4Modifier::new(values, modifiers, operation))
    }

    fn create_point_data(
        &self,
        values: Vec<ValueProvider>,
        flags: Vec<String>,
        modifiers: Vec<Modifier>,
        easing: Functions,
        context: &BaseProviderContext,
    ) -> PointData {
        let (values, time) = match values.as_slice() {
            [ValueProvider::Static(static_val)] if static_val.values(context).len() == 5 => {
                let values = static_val.values(context);
                let point = Vec4::new(values[0], values[1], values[2], values[3]);
                (Vector4Values::Static(point), values[4])
            }
            _ => {
                let values_len: usize = values.iter().map(|v| v.values(context).len()).sum();

                let time = if values_len == 5 {
                    values
                        .last()
                        .and_then(|v| v.values(context).last().copied())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };

                (Vector4Values::Dynamic(values), time)
            }
        };

        PointData::Vector4(Vector4PointData::new(
            values,
            flags.iter().any(|f| f == "lerpHSV"),
            time,
            modifiers,
            easing,
        ))
    }

    fn interpolate_points(
        &self,
        points: &[PointData],
        l: usize,
        r: usize,
        time: f32,
        context: &BaseProviderContext,
    ) -> Vec4 {
        let point_l = points[l].get_vector4(context);
        let point_r = points[r].get_vector4(context);

        if let PointData::Vector4(vector4_point) = &points[l]
            && vector4_point.hsv_lerp
        {
            lerp_hsv_vec4(point_l, point_r, time)
        } else {
            point_l.lerp(point_r, time)
        }
    }

    fn get_points(&self) -> &Vec<PointData> {
        &self.points
    }

    fn get_point(&self, point: &PointData, context: &BaseProviderContext) -> Vec4 {
        point.get_vector4(context)
    }
}

impl Vector4PointDefinition {
    #[cfg(feature = "json")]
    pub fn new(value: serde_json::Value, context: &BaseProviderContext) -> Self {
        let mut instance = Self { points: Vec::new() };
        instance.parse(value, context);
        instance
    }
}
