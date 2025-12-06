use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use glam::{Quat, Vec3, Vec4};
use tracks_rs::animation::coroutine_manager::CoroutineManager;
use tracks_rs::animation::events::{EventData, EventType};
use tracks_rs::animation::track::{Track, V2_POSITION, ValuePropertyHandle};
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::easings::functions::Functions;
use tracks_rs::modifiers::vector3_modifier::Vector3Values;
use tracks_rs::point_data::PointData;
use tracks_rs::point_data::vector3_point_data::Vector3PointData;
use tracks_rs::point_definition::PointDefinition;
use tracks_rs::point_definition::vector3_point_definition::Vector3PointDefinition;

use tracks_rs::animation::track::{V2_COLOR, V2_LOCAL_ROTATION, V2_SCALE};
use tracks_rs::modifiers::quaternion_modifier::QuaternionValues;
use tracks_rs::modifiers::vector4_modifier::Vector4Values;
use tracks_rs::point_data::quaternion_point_data::QuaternionPointData;
use tracks_rs::point_data::vector4_point_data::Vector4PointData;
use tracks_rs::point_definition::quaternion_point_definition::QuaternionPointDefinition;
use tracks_rs::point_definition::vector4_point_definition::Vector4PointDefinition;
use tracks_rs::quaternion_utils::QuaternionUtilsExt;

fn make_vec3_point(x: f32, y: f32, z: f32, time: f32) -> PointData {
    PointData::Vector3(Vector3PointData::new(
        Vector3Values::Static(Vec3::new(x, y, z)),
        false,
        time,
        vec![],
        Functions::EaseLinear,
    ))
}

fn make_vec4_point(r: f32, g: f32, b: f32, a: f32, time: f32) -> PointData {
    PointData::Vector4(Vector4PointData::new(
        Vector4Values::Static(Vec4::new(r, g, b, a)),
        false,
        time,
        vec![],
        Functions::EaseLinear,
    ))
}

fn make_quat_point(x: f32, y: f32, z: f32, time: f32) -> PointData {
    let raw_vec = Vec3::new(x, y, z);
    let quat = Quat::from_unity_euler_degrees(&raw_vec);
    PointData::Quaternion(QuaternionPointData::new(
        QuaternionValues::Static(raw_vec, quat),
        time,
        vec![],
        Functions::EaseLinear,
    ))
}

fn make_event(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    duration: f32,
    start: f32,
) -> EventData {
    EventData {
        raw_duration: duration,
        easing: Functions::EaseLinear,
        repeat: 0,
        start_song_time: start,
        property: EventType::AnimateTrack(ValuePropertyHandle::new(V2_POSITION)),
        track_key,
        point_data: Some(
            Vector3PointDefinition::new(vec![
                make_vec3_point(0.0, 0.0, 0.0, 0.0),
                make_vec3_point(1.0, 0.0, 1.0, 1.0),
            ])
            .into(),
        ),
    }
}

fn make_event2(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    property: &str,
    data: PointData,
    duration: f32,
    start: f32,
) -> EventData {
    EventData {
        raw_duration: duration,
        easing: Functions::EaseLinear,
        repeat: 0,
        start_song_time: start,
        property: EventType::AnimateTrack(ValuePropertyHandle::new(property)),
        track_key,
        point_data: Some(match data {
            PointData::Vector3(_) => Vector3PointDefinition::new(vec![data]).into(),
            PointData::Vector4(_) => Vector4PointDefinition::new(vec![data]).into(),
            PointData::Quaternion(_) => QuaternionPointDefinition::new(vec![data]).into(),
            _ => panic!("Unsupported point data for bench"),
        }),
    }
}

fn bench_start_and_poll(c: &mut Criterion) {
    let mut group = c.benchmark_group("coroutine_manager");

    for &n_tracks in &[10usize, 100usize, 500usize] {
        group.bench_with_input(
            BenchmarkId::new("start_and_poll", n_tracks),
            &n_tracks,
            |b, &n| {
                b.iter(|| {
                    let mut manager = CoroutineManager::default();
                    let mut holder = TracksHolder::new();

                    // create N tracks
                    let mut keys = Vec::with_capacity(n);
                    for i in 0..n {
                        let mut t = Track::default();
                        t.name = format!("track_{}", i);
                        let key = holder.add_track(t);
                        keys.push(key);
                    }

                    let ctx = BaseProviderContext::new();
                    let bpm = 120.0_f32;
                    let song_time = 0.0_f32;

                    // start one coroutine per track — compute song end from events
                    let mut events = Vec::with_capacity(keys.len());
                    let mut max_end = 0.0_f32;
                    for &key in &keys {
                        let ev = make_event(key, 2.0, 0.0);
                        max_end = max_end.max(ev.start_song_time + ev.raw_duration);
                        events.push(ev);
                    }

                    for ev in events.into_iter() {
                        manager.start_event_coroutine(bpm, song_time, &ctx, &mut holder, ev);
                    }

                    // poll repeatedly until song end
                    let step = 0.05_f32;
                    let steps = (max_end / step).ceil() as usize;
                    for i in 0..=steps {
                        let t = i as f32 * step;
                        manager.poll_events(t, &ctx, &mut holder);
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_multi_props(c: &mut Criterion) {
    let mut group = c.benchmark_group("coroutine_manager_multi_props");

    // test with a few different track counts
    for &n_tracks in &[10usize, 50usize, 200usize] {
        group.bench_with_input(
            BenchmarkId::new("multi_props_start_and_poll", n_tracks),
            &n_tracks,
            |b, &n| {
                b.iter(|| {
                    let mut manager = CoroutineManager::default();
                    let mut holder = TracksHolder::new();

                    // create N tracks
                    let mut keys = Vec::with_capacity(n);
                    for i in 0..n {
                        let mut t = Track::default();
                        t.name = format!("track_{}", i);
                        let key = holder.add_track(t);
                        keys.push(key);
                    }

                    let ctx = BaseProviderContext::new();
                    let bpm = 120.0_f32;
                    let song_time = 0.0_f32;

                    // For each track, create multiple events on multiple properties and compute song end
                    let mut events = Vec::new();
                    let mut max_end = 0.0_f32;

                    for &key in &keys {
                        // position: 3 events
                        let ev1 = make_event2(
                            key,
                            V2_POSITION,
                            make_vec3_point(0.0, 0.0, 0.0, 0.0),
                            0.5,
                            0.0,
                        );
                        let ev2 = make_event2(
                            key,
                            V2_POSITION,
                            make_vec3_point(1.0, 0.0, 1.0, 1.0),
                            1.0,
                            0.3,
                        );
                        let ev3 = make_event2(
                            key,
                            V2_POSITION,
                            make_vec3_point(2.0, 0.0, 0.0, 2.0),
                            0.8,
                            0.8,
                        );
                        max_end = max_end.max(ev1.start_song_time + ev1.raw_duration);
                        max_end = max_end.max(ev2.start_song_time + ev2.raw_duration);
                        max_end = max_end.max(ev3.start_song_time + ev3.raw_duration);
                        events.push(ev1);
                        events.push(ev2);
                        events.push(ev3);

                        // local rotation: 2 events
                        let r1 = make_event2(
                            key,
                            V2_LOCAL_ROTATION,
                            make_quat_point(0.0, 0.0, 0.0, 0.0),
                            0.6,
                            0.0,
                        );
                        let r2 = make_event2(
                            key,
                            V2_LOCAL_ROTATION,
                            make_quat_point(0.0, 90.0, 0.0, 1.0),
                            1.2,
                            0.5,
                        );
                        max_end = max_end.max(r1.start_song_time + r1.raw_duration);
                        max_end = max_end.max(r2.start_song_time + r2.raw_duration);
                        events.push(r1);
                        events.push(r2);

                        // scale: 2 events
                        let s1 = make_event2(
                            key,
                            V2_SCALE,
                            make_vec3_point(1.0, 1.0, 1.0, 0.0),
                            0.4,
                            0.0,
                        );
                        let s2 = make_event2(
                            key,
                            V2_SCALE,
                            make_vec3_point(2.0, 2.0, 2.0, 0.8),
                            0.9,
                            0.6,
                        );
                        max_end = max_end.max(s1.start_song_time + s1.raw_duration);
                        max_end = max_end.max(s2.start_song_time + s2.raw_duration);
                        events.push(s1);
                        events.push(s2);

                        // color: 2 events
                        let c1 = make_event2(
                            key,
                            V2_COLOR,
                            make_vec4_point(1.0, 0.0, 0.0, 1.0, 0.0),
                            0.7,
                            0.0,
                        );
                        let c2 = make_event2(
                            key,
                            V2_COLOR,
                            make_vec4_point(0.0, 1.0, 0.0, 1.0, 1.0),
                            1.1,
                            0.4,
                        );
                        max_end = max_end.max(c1.start_song_time + c1.raw_duration);
                        max_end = max_end.max(c2.start_song_time + c2.raw_duration);
                        events.push(c1);
                        events.push(c2);
                    }

                    // start all created events
                    for ev in events.into_iter() {
                        manager.start_event_coroutine(bpm, song_time, &ctx, &mut holder, ev);
                    }

                    // simulate time progression with multiple polls until song end
                    let step = 0.05_f32;
                    let steps = (max_end / step).ceil() as usize;
                    for i in 0..=steps {
                        let t = i as f32 * step;
                        manager.poll_events(t, &ctx, &mut holder);
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_start_and_poll, bench_multi_props);
criterion_main!(benches);
