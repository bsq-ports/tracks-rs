use std::cell::RefCell;

use glam::vec4;
use minifb::Window;
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, types::RangedCoordf64},
    prelude::{BitMapBackend, Cartesian2d, DiscreteRanged, DrawingArea, IntoLinspace, Rectangle},
    style::{BLACK, Color, RED, RGBAColor, WHITE},
};
use serde_json::json;
use tracks_rs::{
    point_definition::{PointDefinition, vector4_point_definition::Vector4PointDefinition},
    base_provider_context::BaseProviderContext,
};

pub struct ColorContext {
    pub definition: Vector4PointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl ColorContext {
    pub fn new() -> Self {
        let mut context = BaseProviderContext::new();

        context.set_values("baseNote0Color", vec4(1.0, 0.0, 0.0, 1.0).into());
        let definition = Vector4PointDefinition::new(
            json!(["baseNote0Color", [0.4, 0.4, 0.4, 1, "opMul"]]),
            &context,
        );
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_color(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    ColorContext,
) {
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(BLACK.mix(0.15))
        .max_light_lines(1)
        .draw()
        .unwrap();

    (chart.into_chart_state(), ColorContext::new())
}

pub fn draw_color(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    context: &ColorContext,
    _epoch: f64,
    _window: &Window,
) {
    {
        let mut chart = chart.clone().restore(&root);
        chart.plotting_area().fill(&WHITE).unwrap();

        chart
            .configure_mesh()
            .light_line_style(BLACK.mix(0.15))
            .max_light_lines(1)
            .draw()
            .unwrap();

        chart
            .draw_series((0.0..1.0).step(0.01).values().map(|x| {
                let color = context
                    .definition
                    .interpolate(x as f32, &context.context.borrow())
                    .0;
                Rectangle::new(
                    [(x, 0.0), (x + 0.01, 1.0)],
                    RGBAColor {
                        0: (color.x * 255.0) as u8,
                        1: (color.y * 255.0) as u8,
                        2: (color.z * 255.0) as u8,
                        3: color.w as f64,
                    }
                    .filled(),
                )
            }))
            .unwrap();

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
