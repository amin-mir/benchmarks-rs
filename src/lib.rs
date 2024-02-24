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
