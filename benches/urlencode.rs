use benchmarks_rs::{urlencode_1, urlencode_2, urlencode_3, urlencode_4};

use divan::{counter::BytesCount, AllocProfiler, Bencher};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_urlencode_1(bencher: Bencher) {
    bencher
        .counter(BytesCount::new(20u8))
        .with_inputs(|| {
            let mut rng = fastrand::Rng::new();
            let mut bytes: [u8; 20] = [0; 20];
            for i in 0..20 {
                bytes[i] = rng.u8(..);
            }
            bytes
        })
        .bench_local_refs(|bytes| urlencode_1(bytes));
}

#[divan::bench]
fn bench_urlencode_2(bencher: Bencher) {
    bencher
        .counter(BytesCount::new(20u8))
        .with_inputs(|| {
            let mut rng = fastrand::Rng::new();
            let mut bytes: [u8; 20] = [0; 20];
            for i in 0..20 {
                bytes[i] = rng.u8(..);
            }
            bytes
        })
        .bench_local_refs(|bytes| urlencode_2(bytes));
}

#[divan::bench]
fn bench_urlencode_3(bencher: Bencher) {
    bencher
        .counter(BytesCount::new(20u8))
        .with_inputs(|| {
            let mut rng = fastrand::Rng::new();
            let mut bytes: [u8; 20] = [0; 20];
            for i in 0..20 {
                bytes[i] = rng.u8(..);
            }
            bytes
        })
        .bench_local_refs(|bytes| urlencode_3(bytes));
}

#[divan::bench]
fn bench_urlencode_4(bencher: Bencher) {
    bencher
        .counter(BytesCount::new(20u8))
        .with_inputs(|| {
            let mut rng = fastrand::Rng::new();
            let mut bytes: [u8; 20] = [0; 20];
            for i in 0..20 {
                bytes[i] = rng.u8(..);
            }
            bytes
        })
        .bench_local_refs(|bytes| urlencode_4(bytes));
}
