use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use serde_json::json;
use std::hint::black_box;
use tracks_rs::{
    base_provider_context::BaseProviderContext,
    point_definition::{
        parse_float_point_definition, parse_quaternion_point_definition,
        parse_vector3_point_definition, parse_vector4_point_definition,
    },
};

fn bench_json_point_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_point_parsing");

    let float_multi = json!([[0.0, 0.0], [1.0, 1.0, "easeInOutSine"]]);
    let float_single = json!([0.5]);

    let vec3_multi = json!([[0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0, "easeInOutQuad"]]);
    let vec3_single = json!([1.0, 2.0, 3.0]);

    let vec4_multi = json!([
        [0.0, 0.5, 1.0, 1.0, 0.0],
        [1.0, 0.0, 0.0, 1.0, 1.0, "easeInOutSine"]
    ]);
    let vec4_single = json!([0.25, 0.5, 0.75, 1.0]);

    let quat_multi = json!([[0.0, 0.0, 0.0, 0.0], [0.0, -90.0, 0.0, 0.5]]);

    let float_base_swizzle_smooth = json!([["baseSongTime.s0_5.x"]]);
    let vec3_base_swizzle_smooth = json!([["baseHeadPosition.zyx.s0_5"]]);
    let vec4_base_modifier_mul = json!([["baseNote0Color", [0.5, 0.25, 2.0, 1.0, "opMul"]]]);

    group.bench_function("float_multi", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), float_multi.clone()),
            |(mut context, value)| {
                black_box(parse_float_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("float_single", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), float_single.clone()),
            |(mut context, value)| {
                black_box(parse_float_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec3_multi", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec3_multi.clone()),
            |(mut context, value)| {
                black_box(parse_vector3_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec3_single", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec3_single.clone()),
            |(mut context, value)| {
                black_box(parse_vector3_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec4_color_multi", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec4_multi.clone()),
            |(mut context, value)| {
                black_box(parse_vector4_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec4_color_single", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec4_single.clone()),
            |(mut context, value)| {
                black_box(parse_vector4_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("quaternion_multi", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), quat_multi.clone()),
            |(mut context, value)| {
                black_box(parse_quaternion_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("float_base_swizzle_smooth", |b| {
        b.iter_batched(
            || {
                (
                    BaseProviderContext::new(),
                    float_base_swizzle_smooth.clone(),
                )
            },
            |(mut context, value)| {
                black_box(parse_float_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec3_base_swizzle_smooth", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec3_base_swizzle_smooth.clone()),
            |(mut context, value)| {
                black_box(parse_vector3_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.bench_function("vec4_base_modifier_mul", |b| {
        b.iter_batched(
            || (BaseProviderContext::new(), vec4_base_modifier_mul.clone()),
            |(mut context, value)| {
                black_box(parse_vector4_point_definition(value, &mut context));
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, bench_json_point_parsing);
criterion_main!(benches);
