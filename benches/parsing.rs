use agprefs::Agpref;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    const BASIC: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/db.agprefs"
    ));
    c.bench_function("nom parsing basic", |b| {
        b.iter(|| {
            let s = black_box(&BASIC);
            black_box(Agpref::parse(s).unwrap());
        })
    });
    c.bench_function("chumsky parsing basic", |b| {
        b.iter(|| {
            let s = black_box(&BASIC);
            black_box(Agpref::parse(s).unwrap());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
