#[macro_use]
extern crate criterion;
extern crate gentle_ap_physics_rs;

use criterion::Criterion;

use gentle_ap_physics_rs::util::vector3::Vector3;


fn criterion_benchmark(c: &mut Criterion) {
    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = Vector3::new (2.0, 2.0, 2.0);
    c.bench_function("vec_add", |b| b.iter(|| x + y));

    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = Vector3::new (1.0, 1.0, 1.0);
    c.bench_function("vec_sub", |b| b.iter(|| x - y));

    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = 2.0;
    c.bench_function("vec_mul", |b| b.iter(|| x * y));

    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = 2.0;
    c.bench_function("vec_div", |b| b.iter(|| x / y));

    let x = Vector3::new (2.0, 3.0, 4.0);
    c.bench_function("vec_mag", |b| b.iter(|| x.magnitude()));

    let x = Vector3::new (2.0, 3.0, 4.0);
    c.bench_function("vec_normalize", |b| b.iter(|| x.normalized()));


    let x = Vector3::new (1.0, 1.0, 1.0);
    let y = Vector3::new (1.0, 1.0, 1.0);
    c.bench_function("vec_dot", |b| b.iter(|| x.dot_product(y)));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
