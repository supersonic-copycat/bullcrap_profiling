use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
//use test_palindroms;

fn criterion_benchmark(c: &mut Criterion) {
    let test_string = "a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!a ,roza upa__la na lapu azora!";
    let mut g = c.benchmark_group("Palindrom");

    g.bench_function(BenchmarkId::new("v", "1"), |b| {
        b.iter(|| test_palindroms::pal_v1(black_box(test_string)))
    });
    g.bench_function(BenchmarkId::new("v", "2"), |b| {
        b.iter(|| test_palindroms::pal_v2(black_box(test_string)))
    });
    g.bench_function(BenchmarkId::new("v", "3"), |b| {
        b.iter(|| test_palindroms::pal_v3(black_box(test_string)))
    });
    g.bench_function(BenchmarkId::new("v", "4"), |b| {
        b.iter(|| test_palindroms::pal_v4(black_box(test_string)))
    });
    g.bench_function(BenchmarkId::new("v", "5"), |b| {
        b.iter(|| test_palindroms::pal_v5(black_box(test_string)))
    });
    g.finish();
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
