use sha2::{Digest, Sha256};

pub fn hash_value_sha256(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value);
    hex::encode(hasher.finalize())
}
