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

    let tx_genesis = Transaction::new_utxo_transaction(
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

    println!("\nCreating genesis block...");
    let genesis = Block::generate_genesis_block(&tx_genesis);
    println!("  hash:   {}", genesis.get_hash());
    println!("  height: {}", genesis.get_height());
    println!("  nonce:  {}", genesis.get_nonce());
    println!("  prev:   {}", genesis.get_pre_block_hash());

    let tx1 = Transaction::new_utxo_transaction(
        vec![TXInput {
            txid: genesis.get_transactions()[0].get_id(),
            vout: 0,
            signature: vec![],
            pub_key: vec![],
        }],
        vec![TXOutput {
            value: 10,
            pub_key_hash: vec![1, 2, 3],
        }],
    );

    println!("\nCreating block 1...");
    let block1 = Block::new_block(genesis.get_hash().to_string(), &[tx1], 1);
    println!("  hash:   {}", block1.get_hash());
    println!("  height: {}", block1.get_height());
    println!("  nonce:  {}", block1.get_nonce());
    println!("  prev:   {}", block1.get_pre_block_hash());

    let tx2 = Transaction::new_utxo_transaction(
        vec![TXInput {
            txid: block1.get_transactions()[0].get_id(),
            vout: 0,
            signature: vec![],
            pub_key: vec![1, 2, 3],
        }],
        vec![
            TXOutput {
                value: 5,
                pub_key_hash: vec![4, 5, 6],
            },
            TXOutput {
                value: 5,
                pub_key_hash: vec![1, 2, 3],
            },
        ],
    );

    println!("\nCreating block 2...");
    let block2 = Block::new_block(block1.get_hash().to_string(), &[tx2], 2);
    println!("  hash:   {}", block2.get_hash());
    println!("  height: {}", block2.get_height());
    println!("  nonce:  {}", block2.get_nonce());
    println!("  prev:   {}", block2.get_pre_block_hash());
}
