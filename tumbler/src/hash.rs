use blake2::{Blake2s256, Digest};

pub fn hash_secret(secret: &str) -> String {
    // let mut hasher = Blake2s256::new();
    // hasher.update(secret.as_bytes());
    hex::encode(secret)
}

pub fn hash_two(first: &str, second: &str) -> String {
    let mut hasher = Blake2s256::new();
    hasher.update(first.as_bytes());
    hasher.update(second.as_bytes());
    hex::encode(hasher.finalize())
}
