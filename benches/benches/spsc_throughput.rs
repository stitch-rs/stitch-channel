use core::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main};
use stitch_channel_core::topology::spsc::SpscBuffer;

fn bench_spsc_roundtrip(c: &mut Criterion) {
    let buffer: SpscBuffer<u64, 1024> = SpscBuffer::new();
    let (mut tx, mut rx) = buffer.split();

    c.bench_function("spsc_1024_roundtrip_hot_path", |b| {
        b.iter(|| {
            tx.try_send(black_box(42)).unwrap();
            black_box(rx.try_recv().unwrap());
        });
    });
}

criterion_group!(benches, bench_spsc_roundtrip);
criterion_main!(benches);
