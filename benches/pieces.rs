use benchmarks_rs::{pieces_float, pieces_usize};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_pieces_float() -> usize {
    pieces_float(divan::black_box(90000), divan::black_box(32768))
}

#[divan::bench]
fn bench_pieces_usize() -> usize {
    pieces_usize(divan::black_box(90000), divan::black_box(32768))
}
