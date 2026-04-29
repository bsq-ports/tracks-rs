use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use glam::{Quat, Vec3, Vec4};
use std::hint::black_box;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::base_value::BaseValue;
use tracks_rs::providers::{AbstractValueProvider, UpdateableValues};

fn seed_context(ctx: &mut BaseProviderContext) {
    ctx.set_values(
        "baseHeadPosition",
        BaseValue::from(Vec3::new(1.0, 2.0, 3.0)),
    );
    ctx.set_values(
        "baseHeadRotation",
        BaseValue::from(Quat::from_array([0.0, 0.38268343, 0.0, 0.9238795])),
    );
    ctx.set_values("baseSongTime", BaseValue::from(128.0_f32));
    ctx.set_values(
        "baseNote0Color",
        BaseValue::from(Vec4::new(0.2, 0.4, 0.6, 1.0)),
    );
}

fn bench_provider_cache_and_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("base_provider_cache");

    for expr in [
        "baseHeadPosition.zyx",
        "baseHeadPosition.zyx.s0_5",
        "baseHeadRotation.s0_5",
        "baseSongTime.s0_5.x",
    ] {
        group.bench_with_input(
            BenchmarkId::new("create_uncached", expr),
            &expr,
            |b, &expr| {
                b.iter_batched(
                    || {
                        let mut ctx = BaseProviderContext::new();
                        seed_context(&mut ctx);
                        ctx
                    },
                    |mut ctx| {
                        let provider = ctx.get_value_provider(expr);
                        black_box(provider);
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(BenchmarkId::new("get_cached", expr), &expr, |b, &expr| {
            let mut ctx = BaseProviderContext::new();
            seed_context(&mut ctx);
            let _ = ctx.get_value_provider(expr);

            b.iter(|| {
                black_box(ctx.get_value_provider(expr));
            });
        });
    }

    group.finish();
}

fn bench_provider_swizzle_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("base_provider_swizzle_update");

    group.bench_function("vec3_swizzle_read", |b| {
        let mut ctx = BaseProviderContext::new();
        seed_context(&mut ctx);
        let mut provider = ctx.get_value_provider("baseHeadPosition.zyx");

        b.iter(|| {
            ctx.update_providers(0.016);
            black_box(provider.values(&ctx));
        });
    });

    group.bench_function("vec3_swizzle_smooth", |b| {
        b.iter_batched(
            || {
                let mut ctx = BaseProviderContext::new();
                seed_context(&mut ctx);
                let provider = ctx.get_value_provider("baseHeadPosition.zyx.s0_5");
                (ctx, provider)
            },
            |(mut ctx, provider)| {
                for _ in 0..60 {
                    ctx.update_providers(0.016);
                    black_box(provider.values(&ctx));
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("quat_smooth_to_euler", |b| {
        b.iter_batched(
            || {
                let mut ctx = BaseProviderContext::new();
                seed_context(&mut ctx);
                let provider = ctx.get_value_provider("baseHeadRotation.s0_5");
                (ctx, provider)
            },
            |(mut ctx, provider)| {
                for _ in 0..60 {
                    ctx.update_providers(0.016);
                    black_box(provider.values(&ctx));
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("float_swizzle_smooth", |b| {
        b.iter_batched(
            || {
                let mut ctx = BaseProviderContext::new();
                seed_context(&mut ctx);
                let provider = ctx.get_value_provider("baseSongTime.s0_5.x");
                (ctx, provider)
            },
            |(mut ctx, provider)| {
                for _ in 0..120 {
                    ctx.update_providers(1.0 / 120.0);
                    black_box(provider.values(&ctx));
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_provider_cache_and_creation,
    bench_provider_swizzle_update
);
criterion_main!(benches);
