use agprefs::Agpref;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    const BASIC: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/db.agprefs"
    ));
    let agpref = Agpref::parse(BASIC).unwrap();
    c.bench_function("composing basic", |b| {
        b.iter(|| black_box(Agpref::to_str(black_box(&agpref))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
