use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use serde_json::json;
use tracks_rs::animation::coroutine_manager::CoroutineManager;
use tracks_rs::animation::events::{EventData, EventType};
use tracks_rs::animation::track::{Track, V2_POSITION, ValuePropertyHandle};
use tracks_rs::animation::tracks_holder::TracksHolder;
use tracks_rs::base_provider_context::BaseProviderContext;
use tracks_rs::easings::functions::Functions;
use tracks_rs::point_definition::{
    parse_quaternion_point_definition, parse_vector3_point_definition,
    parse_vector4_point_definition,
};

use tracks_rs::animation::track::{V2_COLOR, V2_LOCAL_ROTATION, V2_SCALE};

fn make_event(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    context: &mut BaseProviderContext,
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
            parse_vector3_point_definition(
                json!([[0.0, 0.0, 0.0, 0.0], [1.0, 0.0, 1.0, 1.0]]),
                context,
            )
            .into(),
        ),
    }
}

fn make_event_vec3(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    context: &mut BaseProviderContext,
    property: &str,
    values: [f32; 3],
    time: f32,
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
        point_data: Some(
            parse_vector3_point_definition(
                json!([[values[0], values[1], values[2], time]]),
                context,
            )
            .into(),
        ),
    }
}

fn make_event_vec4(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    context: &mut BaseProviderContext,
    property: &str,
    values: [f32; 4],
    time: f32,
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
        point_data: Some(
            parse_vector4_point_definition(
                json!([[values[0], values[1], values[2], values[3], time]]),
                context,
            )
            .into(),
        ),
    }
}

fn make_event_quat(
    track_key: tracks_rs::animation::tracks_holder::TrackKey,
    context: &mut BaseProviderContext,
    property: &str,
    euler_deg: [f32; 3],
    time: f32,
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
        point_data: Some(
            parse_quaternion_point_definition(
                json!([[euler_deg[0], euler_deg[1], euler_deg[2], time]]),
                context,
            )
            .into(),
        ),
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

                    let mut ctx = BaseProviderContext::new();
                    let bpm = 120.0_f32;
                    let song_time = 0.0_f32;

                    // start one coroutine per track — compute song end from events
                    let mut events = Vec::with_capacity(keys.len());
                    let mut max_end = 0.0_f32;
                    for &key in &keys {
                        let ev = make_event(key, &mut ctx, 2.0, 0.0);
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

                    let mut ctx = BaseProviderContext::new();
                    let bpm = 120.0_f32;
                    let song_time = 0.0_f32;

                    // For each track, create multiple events on multiple properties and compute song end
                    let mut events = Vec::new();
                    let mut max_end = 0.0_f32;

                    for &key in &keys {
                        // position: 3 events
                        let ev1 = make_event_vec3(
                            key,
                            &mut ctx,
                            V2_POSITION,
                            [0.0, 0.0, 0.0],
                            0.0,
                            0.5,
                            0.0,
                        );
                        let ev2 = make_event_vec3(
                            key,
                            &mut ctx,
                            V2_POSITION,
                            [1.0, 0.0, 1.0],
                            1.0,
                            1.0,
                            0.3,
                        );
                        let ev3 = make_event_vec3(
                            key,
                            &mut ctx,
                            V2_POSITION,
                            [2.0, 0.0, 0.0],
                            2.0,
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
                        let r1 = make_event_quat(
                            key,
                            &mut ctx,
                            V2_LOCAL_ROTATION,
                            [0.0, 0.0, 0.0],
                            0.0,
                            0.6,
                            0.0,
                        );
                        let r2 = make_event_quat(
                            key,
                            &mut ctx,
                            V2_LOCAL_ROTATION,
                            [0.0, 90.0, 0.0],
                            1.0,
                            1.2,
                            0.5,
                        );
                        max_end = max_end.max(r1.start_song_time + r1.raw_duration);
                        max_end = max_end.max(r2.start_song_time + r2.raw_duration);
                        events.push(r1);
                        events.push(r2);

                        // scale: 2 events
                        let s1 = make_event_vec3(
                            key,
                            &mut ctx,
                            V2_SCALE,
                            [1.0, 1.0, 1.0],
                            0.0,
                            0.4,
                            0.0,
                        );
                        let s2 = make_event_vec3(
                            key,
                            &mut ctx,
                            V2_SCALE,
                            [2.0, 2.0, 2.0],
                            0.8,
                            0.9,
                            0.6,
                        );
                        max_end = max_end.max(s1.start_song_time + s1.raw_duration);
                        max_end = max_end.max(s2.start_song_time + s2.raw_duration);
                        events.push(s1);
                        events.push(s2);

                        // color: 2 events
                        let c1 = make_event_vec4(
                            key,
                            &mut ctx,
                            V2_COLOR,
                            [1.0, 0.0, 0.0, 1.0],
                            0.0,
                            0.7,
                            0.0,
                        );
                        let c2 = make_event_vec4(
                            key,
                            &mut ctx,
                            V2_COLOR,
                            [0.0, 1.0, 0.0, 1.0],
                            1.0,
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
