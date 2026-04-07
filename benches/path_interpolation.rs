use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use serde_json::json;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::WrapBaseValueType;
use tracks_rs::point_definition::base_point_definition::BasePointDefinition;
use tracks_rs::point_definition::point_definition_interpolation::PointDefinitionInterpolation;
use tracks_rs::test_helpers::{
    parse_float_point_definition, parse_quaternion_point_definition, parse_vector3_point_definition,
};

fn make_float_pair(ctx: &mut BaseProviderContext) -> (BasePointDefinition, BasePointDefinition) {
    let prev = parse_float_point_definition(json!([[0.0, 0.0], [10.0, 1.0]]), ctx);
    let next = parse_float_point_definition(json!([[10.0, 0.0], [20.0, 1.0]]), ctx);

    (
        BasePointDefinition::Float(prev),
        BasePointDefinition::Float(next),
    )
}

fn make_vec3_pair(ctx: &mut BaseProviderContext) -> (BasePointDefinition, BasePointDefinition) {
    let prev =
        parse_vector3_point_definition(json!([[0.0, 0.0, 0.0, 0.0], [3.0, 3.0, 3.0, 1.0]]), ctx);
    let next =
        parse_vector3_point_definition(json!([[3.0, 3.0, 3.0, 0.0], [6.0, 6.0, 6.0, 1.0]]), ctx);

    (
        BasePointDefinition::Vector3(prev),
        BasePointDefinition::Vector3(next),
    )
}

fn make_quat_pair(ctx: &mut BaseProviderContext) -> (BasePointDefinition, BasePointDefinition) {
    let prev = parse_quaternion_point_definition(json!([[0.0, 0.0, 0.0, 0.0]]), ctx);
    let next = parse_quaternion_point_definition(json!([[0.0, 180.0, 0.0, 0.0]]), ctx);

    (
        BasePointDefinition::Quaternion(prev),
        BasePointDefinition::Quaternion(next),
    )
}

fn bench_path_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_interpolation");
    let mut parse_ctx = BaseProviderContext::new();
    let ctx = BaseProviderContext::new();

    group.bench_function("float_interpolate", |b| {
        let (prev, next) = make_float_pair(&mut parse_ctx);
        let mut interp = PointDefinitionInterpolation::new(Some(next), WrapBaseValueType::Float);
        interp.prev_point = Some(prev);
        interp.interpolate_time = 0.5;

        b.iter(|| {
            black_box(interp.interpolate(0.25, &ctx));
        });
    });

    group.bench_function("vec3_interpolate", |b| {
        let (prev, next) = make_vec3_pair(&mut parse_ctx);
        let mut interp = PointDefinitionInterpolation::new(Some(next), WrapBaseValueType::Vec3);
        interp.prev_point = Some(prev);
        interp.interpolate_time = 0.5;

        b.iter(|| {
            black_box(interp.interpolate(0.25, &ctx));
        });
    });

    group.bench_function("quat_interpolate", |b| {
        let (prev, next) = make_quat_pair(&mut parse_ctx);
        let mut interp = PointDefinitionInterpolation::new(Some(next), WrapBaseValueType::Quat);
        interp.prev_point = Some(prev);
        interp.interpolate_time = 0.25;

        b.iter(|| {
            black_box(interp.interpolate(0.0, &ctx));
        });
    });

    group.bench_function("init_swap_float", |b| {
        let (prev, next) = make_float_pair(&mut parse_ctx);
        b.iter_batched(
            || PointDefinitionInterpolation::new(Some(prev.clone()), WrapBaseValueType::Float),
            |mut interp| {
                interp.init(Some(next.clone()));
                black_box(interp);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_path_interpolation);
criterion_main!(benches);
