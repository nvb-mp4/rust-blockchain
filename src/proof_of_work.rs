use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};

use crate::block::Block;

const TARGET_BITS: i32 = 8;

pub const MAX_NONCE: i64 = i64::MAX;

pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target <<= 256 - TARGET_BITS;
        ProofOfWork { block, target }
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
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
            if hash_int < self.target {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        (nonce, HEXLOWER.encode(hash.as_slice()))
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.block.get_nonce());
        let hash = crate::sha256_digest(data.as_slice());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        hash_int < self.target
    }
}
