use std::slice;

use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::{
    current_timestamp, proof_of_work::ProofOfWork, sha256_digest, transaction::Transaction,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Self {
        let mut block = Self {
            timestamp: current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    pub fn serialize(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("failed to serialize block")
    }
    pub fn deserialize(bytes: &[u8]) -> Self {
        postcard::from_bytes(bytes).expect("failed to deserialize block")
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }
    pub fn get_hash(&self) -> &str {
        &self.hash
    }
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_nonce(&self) -> i64 {
        self.nonce
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.transactions.len() * 32);
        for tx in &self.transactions {
            buf.extend_from_slice(tx.id.as_bytes());
        }
        sha256_digest(&buf)
    }

    pub fn generate_genesis_block(transaction: &Transaction) -> Self {
        Block::new_block(String::from("None"), slice::from_ref(transaction), 0)
    }
}

impl From<Block> for IVec {
    fn from(value: Block) -> Self {
        Self::from(postcard::to_stdvec(&value).expect("failed to serialize block"))
    }
}
