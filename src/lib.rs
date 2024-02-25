mod pieces;
pub use pieces::{pieces_float, pieces_usize};

pub fn urlencode_1(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());
    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }
    encoded
}

pub fn urlencode_2(t: &[u8; 20]) -> String {
    t.iter()
        .fold(String::with_capacity(3 * t.len()), |mut acc, &b| {
            acc.push('%');
            acc.push_str(&hex::encode([b]));
            acc
        })
}

pub fn urlencode_3(t: &[u8; 20]) -> String {
    t.iter().map(|b| format!("%{:02x}", b)).collect()
}

pub fn urlencode_4(bytes: &[u8; 20]) -> anyhow::Result<String> {
    let mut res = vec![0; 3 * bytes.len()];
    let mut buf = [0; 1];
    for (i, &b) in bytes.iter().enumerate() {
        let mut idx = i * 3;
        res[idx] = b'%';

        idx += 1;
        buf[0] = b;
        hex::encode_to_slice(&buf[..], &mut res[idx..idx + 2])?;
    }
    String::from_utf8(res).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    static HASH_BYTES: [u8; 20] = [
        214, 159, 145, 230, 178, 174, 76, 84, 36, 104, 209, 7, 58, 113, 212, 234, 19, 135, 154, 127,
    ];

    #[test]
    fn test_url_encode_bytes() {
        let res1 = urlencode_1(&HASH_BYTES);
        let res2 = urlencode_2(&HASH_BYTES);
        let res3 = urlencode_3(&HASH_BYTES);
        let res4 = urlencode_4(&HASH_BYTES).unwrap();
        assert_eq!(res4, res1);
        assert_eq!(res4, res2);
        assert_eq!(res4, res3);
    }
}
