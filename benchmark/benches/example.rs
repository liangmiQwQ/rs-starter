use criterion::{Criterion, criterion_group, criterion_main};

fn bench(c: &mut Criterion) {
  c.bench_function("hello_benchmark", |b| {
    b.iter(|| {
      let x = 1;
      let y = 2;
      x + y
    });
  });
}

criterion_group!(benches, bench);
criterion_main!(benches);
