use std::cell::RefCell;

use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, types::RangedCoordf64},
    prelude::{BitMapBackend, Cartesian2d, DiscreteRanged, DrawingArea, IntoLinspace},
    series::LineSeries,
    style::{BLACK, BLUE, Color, GREEN, IntoFont, TRANSPARENT},
};
use serde_json::json;

use tracks_rs::{
    point_definition::{PointDefinition, float_point_definition::FloatPointDefinition},
    base_provider_context::BaseProviderContext,
};

pub struct FloatContext {
    pub definition: FloatPointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl FloatContext {
    pub fn new() -> Self {
        let context = BaseProviderContext::new();
        let definition =
            FloatPointDefinition::new(json!([[0.0, 0.0], [1.0, 1.0, "easeInOutSine"]]), &context);
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_2d(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    FloatContext,
) {
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .set_all_label_area_size(30)
        .build_cartesian_2d(-1.2..1.2, -1.2..12.0)
        .unwrap();

    chart
        .configure_mesh()
        .label_style(("sans-serif", 15).into_font().color(&GREEN))
        .axis_style(GREEN)
        .draw()
        .unwrap();

    (chart.into_chart_state(), FloatContext::new())
}

pub fn draw_2d(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    context: &FloatContext,
    _epoch: f64,
) {
    {
        let mut chart = chart.clone().restore(root);
        chart.plotting_area().fill(&BLACK).unwrap();

        chart
            .configure_mesh()
            .bold_line_style(GREEN.mix(0.2))
            .light_line_style(TRANSPARENT)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0f64..1f64).step(0.0001).values().map(|x| {
                    (
                        x,
                        context
                            .definition
                            .interpolate(x as f32, &context.context.borrow())
                            .0 as f64,
                    )
                }),
                &BLUE,
            ))
            .unwrap();
    }
}
