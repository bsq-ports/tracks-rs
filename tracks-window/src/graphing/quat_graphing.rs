use std::cell::RefCell;

use glam::{Mat4, Vec3};
use minifb::Window;
use plotters::{
    backend::BGRXPixel,
    chart::{ChartBuilder, ChartState},
    coord::{Shift, ranged3d::Cartesian3d, types::RangedCoordf64},
    prelude::{BitMapBackend, DiscreteRanged, DrawingArea, IntoLinspace},
    series::LineSeries,
    style::{BLACK, Color, RED, RGBAColor, WHITE},
};
use serde_json::json;

use tracks_rs::{
    point_definition::{PointDefinition, quaternion_point_definition::QuaternionPointDefinition},
    base_provider_context::BaseProviderContext,
};

pub struct QuatContext {
    pub definition: QuaternionPointDefinition,
    pub context: RefCell<BaseProviderContext>,
}

impl QuatContext {
    pub fn new() -> Self {
        let context = BaseProviderContext::new();
        let definition = QuaternionPointDefinition::new(json!([0, "baseCombo", 0]), &context);
        Self {
            definition,
            context: RefCell::new(context),
        }
    }
}

pub fn graph_quat(
    root: DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
) -> (
    ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    QuatContext,
) {
    let mut chart = ChartBuilder::on(&root)
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

    (chart.into_chart_state(), QuatContext::new())
}

pub fn draw_quat(
    root: &DrawingArea<BitMapBackend<'_, BGRXPixel>, Shift>,
    chart: &ChartState<Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>,
    context: &QuatContext,
    epoch: f64,
    _window: &Window,
) {
    {
        context.context.borrow_mut().set_values(
            "baseCombo",
            ((epoch.sin() as f32 + 1.0) * 0.5 * 45.0).into(),
        );
        let mut chart = chart.clone().restore(root);
        chart.plotting_area().fill(&WHITE).unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = epoch / 2.0;
            pb.pitch = 0.5;
            pb.scale = 0.7;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(1)
            .draw()
            .unwrap();

        let _ = (0.0..1.0)
            .step(0.01)
            .values()
            .map(|x: f64| {
                let point = context
                    .definition
                    .interpolate(x as f32, &context.context.borrow())
                    .0
                    .normalize();
                let to: Vec3 = Mat4::from_quat(point)
                    .transform_vector3(Vec3::Z)
                    .normalize();

                chart
                    .draw_series(LineSeries::new(
                        [(0.0, 0.0, x), (to.x as f64, to.y as f64, x + to.z as f64)],
                        RGBAColor {
                            0: (255.0 * x) as u8,
                            1: (255.0 * (1.0 - x)) as u8,
                            2: 0,
                            3: 1.0,
                        },
                    ))
                    .unwrap();
            })
            .collect::<Vec<_>>();

        chart
            .configure_series_labels()
            .border_style(RED)
            .draw()
            .unwrap();
    }
}
