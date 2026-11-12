use data_encoding::HEXLOWER;
use num_bigint::BigInt;

use crate::{block::Block, sha256_digest};

pub const TARGET_BITS: i32 = 8;
pub const MAX_NONCE: i64 = i64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> Self {
        let mut target = BigInt::from(1);
        target <<= (256 - TARGET_BITS) as usize;
        Self { block, target }
    }

    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.block.get_pre_block_hash().as_bytes());
        data.extend(self.block.hash_transactions());
        data.extend(self.block.get_timestamp().to_be_bytes());
        data.extend(TARGET_BITS.to_be_bytes());
        data.extend(nonce.to_be_bytes());
        data
    }

    pub fn run(&self) -> (i64, String) {
        println!("Mining...");
        for nonce in 0..MAX_NONCE {
            let data = self.prepare_data(nonce);
            let hash = sha256_digest(&data);
            let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
            if hash_int < self.target {
                return (nonce, HEXLOWER.encode(&hash));
            }
        }
        panic!("MAX_NONCE exhausted");
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.get_nonce());
        let hash = sha256_digest(&data);
        let hash_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &hash);
        hash_int < self.target
    }
}
