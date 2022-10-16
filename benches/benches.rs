use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, thread_rng};
use rand::distributions::{Standard};
use mpc_block::{BlockBytes, BlockSse, BlockU128, xor_blocks_bytes, xor_blocks_sse, xor_blocks_u128};

pub fn blocks_xor(c: &mut Criterion) {
    let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockU128::new).take(1_000_000).collect();
    let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockU128::new).take(1_000_000).collect();
    c.bench_function("BlockU128 xor", |bencher| bencher.iter(||
        { xor_blocks_u128(black_box(&mut a), black_box(&b)) }
    ));

    let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockBytes::new).take(1_000_000).collect();
    let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockBytes::new).take(1_000_000).collect();
    c.bench_function("BlockBytes xor", |bencher| bencher.iter(||
        { xor_blocks_bytes(black_box(&mut a), black_box(&b)) }
    ));

    let mut a: Vec<_> = thread_rng().sample_iter(Standard).map(BlockSse::new).take(1_000_000).collect();
    let b: Vec<_> = thread_rng().sample_iter(Standard).map(BlockSse::new).take(1_000_000).collect();
    c.bench_function("BlockSse xor", |bencher| bencher.iter(||
        { xor_blocks_sse(black_box(&mut a), black_box(&b)) }
    ));
}

criterion_group!(benches, blocks_xor);
criterion_main!(benches);