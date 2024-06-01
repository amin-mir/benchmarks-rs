use std::str;

const VALUES: &[&[u8]] = &[b"-0.1", b"-50.2", b"-66.9", b"43.4", b"92.2"];

fn main() {
    divan::main();
}

#[divan::bench(args = VALUES)]
fn parse_f32(b: &[u8]) -> f32 {
    let mut i = 0;
    let mut int = 0i32;

    let mut is_neg = false;
    if b[i] == b'-' {
        is_neg = true;
        i += 1;
    }
    while b[i] != b'.' {
        let digit = b[i] - b'0';
        int = int * 10 + digit as i32;
        i += 1;
    }

    // There's only one digit after '.'
    let digit = b[i + 1] - b'0';
    let frac: f32 = digit as f32 / 10.0;
    let mut result = int as f32 + frac;
    if is_neg {
        result = -result;
    }
    result
}

#[divan::bench(args = VALUES)]
fn bench_std_parse(b: &[u8]) -> f32 {
    let s = str::from_utf8(b).unwrap();
    s.parse::<f32>().unwrap()
}
