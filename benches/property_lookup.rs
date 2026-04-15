use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use tracks_rs::animation::property::{PathProperty, ValueProperty};
use tracks_rs::animation::track::{PathPropertyHandle, Track, V2_POSITION, ValuePropertyHandle};
use tracks_rs::base_value::{BaseValue, WrapBaseValueType};

fn make_track(custom_count: usize) -> Track {
    let mut track = Track::default();

    for i in 0..custom_count {
        let key = format!("custom_{}", i);
        track.properties.insert(
            key.clone(),
            ValueProperty::new(Some(BaseValue::Float(i as f32)), WrapBaseValueType::Float),
        );
        track
            .path_properties
            .insert(key, PathProperty::empty(WrapBaseValueType::Float));
    }

    track
}

fn bench_property_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("property_lookup");

    for &size in &[32usize, 256usize, 1024usize] {
        group.bench_with_input(
            BenchmarkId::new("value_get_builtin_str", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                b.iter(|| {
                    black_box(track.properties.get(V2_POSITION));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("value_get_builtin_handle", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                let handle = ValuePropertyHandle::new(V2_POSITION);
                b.iter(|| {
                    black_box(track.properties.get_by_handle(&handle));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("value_get_custom_str", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                let key = format!("custom_{}", n / 2);
                b.iter(|| {
                    black_box(track.properties.get(black_box(&key)));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("value_get_custom_handle", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                let key = format!("custom_{}", n / 2);
                let handle = ValuePropertyHandle::ByName(key);
                b.iter(|| {
                    black_box(track.properties.get_by_handle(&handle));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("path_get_custom_str", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                let key = format!("custom_{}", n / 2);
                b.iter(|| {
                    black_box(track.path_properties.get(black_box(&key)));
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("path_get_custom_handle", size),
            &size,
            |b, &n| {
                let track = make_track(n);
                let key = format!("custom_{}", n / 2);
                let handle = PathPropertyHandle::ByName(key);
                b.iter(|| {
                    black_box(track.path_properties.get_by_handle(&handle));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_property_lookup);
criterion_main!(benches);
