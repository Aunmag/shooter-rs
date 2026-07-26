use bevy::math::{EulerRot, Quat};
use criterion::{BenchmarkId, Criterion};
use std::{f32::consts::PI, hint::black_box};

fn angle_euler(q: Quat) -> f32 {
    return q.to_euler(EulerRot::ZXY).0;
}

fn angle_custom(q: Quat) -> f32 {
    return f32::atan2(q.z, q.w) * 2.0;
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("quat_angle");

    for m in [0.0, 0.5, 1.0] {
        let q = Quat::from_rotation_z(PI * m);
        group.bench_with_input(BenchmarkId::new("Euler", m), &q, |b, q| {
            b.iter(|| angle_euler(black_box(*q)))
        });

        group.bench_with_input(BenchmarkId::new("Custom", m), &q, |b, q| {
            b.iter(|| angle_custom(black_box(*q)))
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
