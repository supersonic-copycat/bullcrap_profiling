use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use test_palindroms;

fn criterion_benchmark(c: &mut Criterion) {
    let test_string = "a ,roza upa__la na lapu azora!";
    c.bench_function("v1", |b| {
        b.iter(|| test_palindroms::pal_v1(black_box(test_string)))
    });
    c.bench_function("v2", |b| {
        b.iter(|| test_palindroms::pal_v2(black_box(test_string)))
    });
    c.bench_function("v3", |b| {
        b.iter(|| test_palindroms::pal_v3(black_box(test_string)))
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
