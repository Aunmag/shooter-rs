use bevy::math::{EulerRot, Quat};
use criterion::{BenchmarkId, Criterion, Throughput};
use std::{f32::consts::PI, hint::black_box};

fn angle_euler(q: Quat) -> f32 {
    return q.to_euler(EulerRot::ZXY).0;
}

fn angle_custom(q: Quat) -> f32 {
    return f32::atan2(q.z, q.w) * 2.0;
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Quat Angle");

    for r in [0.0, PI / 3.0, PI] {
        let q = Quat::from_rotation_z(r);
        group.bench_with_input(BenchmarkId::new("Euler", q), &q, |b, q| {
            b.iter(|| angle_euler(*q))
        });

        group.bench_with_input(BenchmarkId::new("Custom", q), &q, |b, q| {
            b.iter(|| angle_custom(*q))
        });
    }

    group.finish();
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
