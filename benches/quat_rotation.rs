use bevy::math::{Quat, Vec2};
use criterion::Criterion;
use std::hint::black_box;

/// Default bevy implementation
fn v1(v: Vec2, q: Quat) -> Vec2 {
    return (q * v.extend(1.0)).truncate();
}

/// Custom implementation
fn v2(v: Vec2, q: Quat) -> Vec2 {
    let v = Vec2::new(v.x * q.w - v.y * q.z, v.x * q.z + v.y * q.w);
    let v = Vec2::new(v.x * q.w - v.y * q.z, v.x * q.z + v.y * q.w);
    return v;
}

/// Modified custom implementation
fn v3(v: Vec2, q: Quat) -> Vec2 {
    let rw = q.w * q.w - q.z * q.z;
    let rz = 2.0 * q.w * q.z;
    return Vec2::new(v.x * rw - v.y * rz, v.x * rz + v.y * rw);
}

fn bench(c: &mut Criterion) {
    let v = Vec2::from_angle(1.2) * 4.3;
    let a = 0.4;
    let q = Quat::from_rotation_z(a);
    let mut group = c.benchmark_group("quat_rotation");
    group.bench_function("v1", |b| b.iter(|| v1(black_box(v), black_box(q))));
    group.bench_function("v2", |b| b.iter(|| v2(black_box(v), black_box(q))));
    group.bench_function("v3", |b| b.iter(|| v3(black_box(v), black_box(q))));
    group.finish();
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
