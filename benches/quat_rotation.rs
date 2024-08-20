use bevy::math::{Quat, Vec2};
use criterion::Criterion;
use std::hint::black_box;

fn rotate_default(v: Vec2, q: Quat) -> Vec2 {
    return (q * v.extend(1.0)).truncate();
}

fn rotate_custom(v: Vec2, q: Quat) -> Vec2 {
    let v = Vec2::new((v.x * q.w) - (v.y * q.z), (v.x * q.z) + (v.y * q.w));
    let v = Vec2::new((v.x * q.w) - (v.y * q.z), (v.x * q.z) + (v.y * q.w));
    return v;
}

fn bench(c: &mut Criterion) {
    let v = Vec2::from_angle(1.2) * 4.3;
    let a = 0.4;
    let q = Quat::from_rotation_z(a);

    c.bench_function("Default", |b| {
        b.iter(|| rotate_default(black_box(v), black_box(q)))
    });

    c.bench_function("Custom", |b| {
        b.iter(|| rotate_custom(black_box(v), black_box(q)))
    });
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
