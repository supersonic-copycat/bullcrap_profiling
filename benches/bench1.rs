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
fn mutate(s: &str) -> String {
    return s.into();
}

fn repeat_str(s: &str, n: usize) -> String {
    let mut res = String::with_capacity(s.len() * n);
    for _ in 0..n {
        res.push_str(&mutate(s));
    }
    res
}

fn criterion_bench_inputs(c: &mut Criterion) {
    let base_string = "a roza upala na lapu azora";
    let mut g = c.benchmark_group("Plindrom_inps");
    for i in 4..=10 {
        let test_string = repeat_str(base_string, i);
        g.throughput(criterion::Throughput::Bytes((i * base_string.len()) as u64));
        g.bench_with_input(
            BenchmarkId::from_parameter(i),
            &test_string.len(),
            |b, _s| {
                //
                b.iter(|| test_palindroms::pal_v4(black_box(&test_string)));
            },
        );
    }
    g.finish();
}

//criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, criterion_bench_inputs);
criterion_main!(benches);
