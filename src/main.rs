use sha2::{Digest, Sha256};

mod block;
mod proof_of_work;
mod transaction;
mod utxo;

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is before UNIX epoch")
        .as_secs() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().as_slice().to_vec()
}
fn main() {}
