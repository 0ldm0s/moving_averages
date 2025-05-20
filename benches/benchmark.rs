//! 移动平均算法性能基准测试

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use moving_averages::{Sma, Ema, Rma};

fn bench_sma(c: &mut Criterion) {
    let mut group = c.benchmark_group("SMA");
    
    group.bench_function("window=3", |b| {
        let mut sma = Sma::<f64, 3>::new();
        b.iter(|| {
            sma.next(black_box(10.0));
            sma.next(black_box(20.0));
            sma.next(black_box(30.0));
        })
    });

    group.bench_function("window=10", |b| {
        let mut sma = Sma::<f64, 10>::new();
        b.iter(|| {
            for i in 1..=10 {
                sma.next(black_box(i as f64));
            }
        })
    });

    group.finish();
}

fn bench_ema(c: &mut Criterion) {
    c.bench_function("EMA alpha=0.1", |b| {
        let mut ema = Ema::new(0.1);
        b.iter(|| {
            for i in 1..=10 {
                ema.next(black_box(i as f64));
            }
        })
    });
}

fn bench_rma(c: &mut Criterion) {
    let mut group = c.benchmark_group("RMA");
    
    group.bench_function("window=5", |b| {
        let mut rma = Rma::<f64, 5>::new();
        b.iter(|| {
            for i in 1..=10 {
                rma.next(black_box(i as f64));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_sma, bench_ema, bench_rma);
criterion_main!(benches);