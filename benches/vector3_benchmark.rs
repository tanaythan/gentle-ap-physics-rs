#[macro_use]
extern crate criterion;
extern crate gentle_ap_physics_rs;

use criterion::Criterion;

use gentle_ap_physics_rs::util::vector3::Vector3;


fn criterion_benchmark(c: &mut Criterion) {
    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = Vector3::new (2.0, 2.0, 2.0);
    c.bench_function("vec_magnitude", |b| b.iter(|| x + y));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
