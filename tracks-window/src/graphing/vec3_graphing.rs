use std::cell::RefCell;

use glam::Vec3;
use minifb::Window;
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{
        BitMapBackend, Circle, DiscreteRanged, DrawingArea, EmptyElement, IntoLinspace, Text,
    },
    series::LineSeries,
    style::{BLACK, Color, IntoFont, RED, ShapeStyle, WHITE},
};
use serde_json::json;

use tracks_rs::{
    point_definition::{PointDefinition, vector3_point_definition::Vector3PointDefinition},
    base_provider_context::BaseProviderContext,
};

pub struct Vec3Context {
    pub definition: Vector3PointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl Vec3Context {
    pub fn new() -> Self {
        let context = BaseProviderContext::new();
        let definition = Vector3PointDefinition::new(json!(["baseLeftHandPosition"]), &context);
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_vec3(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    Vec3Context,
) {
    let mut chart = ChartBuilder::on(&root)
        .caption("3D Plot Test", ("sans", 20))
        .build_cartesian_3d(0.0..3.0, 0.0..3.0, 0.0..3.0)
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(1)
        .draw()
        .unwrap();

    (chart.into_chart_state(), Vec3Context::new())
}

pub fn draw_vec3(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    context: &Vec3Context,
    epoch: f64,
    _window: &Window,
) {
    {
        context.context.borrow_mut().set_values(
            "baseLeftHandPosition",
            Vec3::new(epoch.sin() as f32 + 1.0, 2.0, 3.0).into(),
        );
        let mut chart: plotters::prelude::ChartContext<
            '_,
            BitMapBackend<'_, BGRXPixel>,
            Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>,
        > = chart.clone().restore(root);
        chart.plotting_area().fill(&WHITE).unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = epoch / 10.0;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(1)
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (0.0..1.0).step(0.0001).values().map(|x| {
                    let point = context
                        .definition
                        .interpolate(x as f32, &context.context.borrow())
                        .0;
                    (point.x as f64, point.y as f64, point.z as f64)
                }),
                &RED,
            ))
            .unwrap();

        let dot_and_label = |x: f64, y: f64, z: f64| {
            EmptyElement::<(f64, f64, f64), BitMapBackend<BGRXPixel>>::at((x, y, z))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                + Text::new(
                    format!("({:.2},{:.2},{:.2})", x, y, z),
                    (10, 0),
                    ("sans-serif", 15.0).into_font(),
                )
        };

        let mut draw_t = |x: f32| {
            let point = context
                .definition
                .interpolate(x, &context.context.borrow())
                .0;
            chart
                .draw_series(std::iter::once(dot_and_label(
                    point.x as f64,
                    point.y as f64,
                    point.z as f64,
                )))
                .unwrap();
        };

        draw_t(0.0);
        draw_t(((epoch.sin() + 1.0) * 0.5) as f32);
        draw_t(1.0);

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
