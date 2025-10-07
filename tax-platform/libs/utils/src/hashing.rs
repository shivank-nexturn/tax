use sha2::{Sha256, Digest};
use hex;

pub fn calculate_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn verify_checksum(data: &[u8], expected_checksum: &str) -> bool {
    let calculated = calculate_sha256(data);
    calculated == expected_checksum
}
