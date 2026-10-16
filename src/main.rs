mod block;
mod proof_of_work;
mod transaction;

use sha2::{Digest, Sha256};

use crate::block::Block;
use crate::transaction::{TXInput, TXOutput, Transaction};

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn main() {
    println!("rust-blockchain is ready");

    let tx = Transaction::new_utxo_transaction(
        vec![TXInput {
            txid: vec![],
            vout: 0,
            signature: vec![],
            pub_key: vec![],
        }],
        vec![TXOutput {
            value: 10,
            pub_key_hash: vec![],
        }],
    );

    println!("Creating genesis block...");
    let genesis = Block::generate_genesis_block(&tx);
    println!("genesis hash:   {}", genesis.get_hash());
    println!("genesis height: {}", genesis.get_height());
    println!("genesis nonce:  {}", genesis.get_nonce());
}
