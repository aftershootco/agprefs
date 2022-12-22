use agprefs::Agpref;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let basic = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/db.agprefs"
    ))
    .unwrap();
    c.bench_function("parsing basic", |b| {
        b.iter(|| Agpref::from_str(black_box(&basic)))
    });

    let agpref = Agpref::from_str(&basic).unwrap();
    c.bench_function("composing basic", |b| {
        b.iter(|| Agpref::to_str(black_box(&agpref)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
