use rand::{thread_rng, Rng};

fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key_bytes = key.as_bytes();
    let mut encoded = String::with_capacity(s.len());
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        let shift = (key_bytes[i % key_bytes.len()] - b'a') as i32;
        let shifted = (((c as u8 - b'a') as i32 + shift) % 26 + 26) % 26;
        encoded.push((shifted as u8 + b'a') as char);
    }
    Some(encoded)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }
    let key_bytes = key.as_bytes();
    let mut decoded = String::with_capacity(s.len());
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_lowercase() {
            return None;
        }
        let shift = (key_bytes[i % key_bytes.len()] - b'a') as i32;
        let shifted = (((c as u8 - b'a') as i32 - shift) % 26 + 26) % 26;
        decoded.push((shifted as u8 + b'a') as char);
    }
    Some(decoded)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = thread_rng();
    let key: String = (0..100)
        .map(|_| rng.gen_range(b'a'..=b'z') as char)
        .collect();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
