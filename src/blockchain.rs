#![allow(unused)]

use sled::{
    Db,
    transaction::{ConflictableTransactionError, Transactional},
};

use crate::block::Block;
use crate::transaction::{OutPoint, TxId, UTXO};
use crate::utxo::UTXOSet;

pub struct Blockchain {
    db: Db,
    blocks: sled::Tree,
    chainstate: sled::Tree,
}

impl Blockchain {
    pub fn open(path: &str) -> sled::Result<Self> {
        let db = sled::open(path)?;
        let blocks = db.open_tree(b"blocks")?;
        let chainstate = db.open_tree(b"chainstate")?;
        Ok(Blockchain {
            db,
            blocks,
            chainstate,
        })
    }

    pub fn load_utxo(&self) -> sled::Result<UTXOSet> {
        let mut map = std::collections::HashMap::new();
        for item in self.chainstate.iter() {
            let (key, value) = item?;
            if key.len() != 40 {
                continue;
            }
            let txid_bytes: [u8; 32] = key[..32].try_into().unwrap();
            let vout = u64::from_be_bytes(key[32..].try_into().unwrap()) as usize;
            let utxo: UTXO = postcard::from_bytes(&value)
                .map_err(|e| sled::Error::Unsupported(format!("decode utxo: {}", e).into()))?;
            map.insert(
                OutPoint {
                    txid: TxId(txid_bytes),
                    vout,
                },
                utxo,
            );
        }
        Ok(UTXOSet::from_map(map))
    }

    pub fn commit_block(&self, block: &Block) -> sled::Result<()> {
        let block_key = block.get_hash().as_bytes().to_vec();
        let block_value = postcard::to_stdvec(block)
            .map_err(|e| sled::Error::Unsupported(format!("encode block: {}", e).into()))?;

        (&self.blocks, &self.chainstate)
            .transaction(|(blocks, chainstate)| {
                blocks.insert(block_key.clone(), block_value.clone())?;

                for tx in block.get_transactions() {
                    for input in &tx.vin {
                        let mut key = Vec::with_capacity(40);
                        key.extend_from_slice(input.txid.as_bytes());
                        key.extend_from_slice(&(input.vout as u64).to_be_bytes());
                        chainstate.remove(key)?;
                    }
                    for (i, out) in tx.vout.iter().enumerate() {
                        let mut key = Vec::with_capacity(40);
                        key.extend_from_slice(tx.id.as_bytes());
                        key.extend_from_slice(&(i as u64).to_be_bytes());
                        let utxo = UTXO {
                            out_point: (tx.id, i),
                            output: out.clone(),
                        };
                        let value = postcard::to_stdvec(&utxo).map_err(|e| {
                            ConflictableTransactionError::Abort(sled::Error::Unsupported(
                                format!("encode utxo: {}", e).into(),
                            ))
                        })?;
                        chainstate.insert(key, value)?;
                    }
                }
                Ok(())
            })
            .map_err(|e| sled::Error::Unsupported(format!("transaction: {:?}", e).into()))?;

        Ok(())
    }

    pub fn has_block(&self, hash: &str) -> sled::Result<bool> {
        Ok(self.blocks.contains_key(hash.as_bytes())?)
    }

    pub fn last_block(&self) -> sled::Result<Option<Block>> {
        match self.blocks.last()? {
            Some((_, value)) => {
                let block: Block = postcard::from_bytes(&value)
                    .map_err(|e| sled::Error::Unsupported(format!("decode block: {}", e).into()))?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
}
