#![allow(unused)]

use std::collections::{HashMap, HashSet};

use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::block::Block;
use crate::transaction::{OutPoint, Transaction, UTXO, pub_key_hash};

pub struct UTXOSet {
    map: HashMap<OutPoint, UTXO>,
}

impl UTXOSet {
    pub fn new() -> Self {
        UTXOSet {
            map: HashMap::new(),
        }
    }

    pub fn from_map(map: HashMap<OutPoint, UTXO>) -> Self {
        UTXOSet { map }
    }

    pub fn into_map(self) -> HashMap<OutPoint, UTXO> {
        self.map
    }

    pub fn rebuild(blocks: &[Block]) -> UTXOSet {
        let mut outputs: HashMap<OutPoint, UTXO> = HashMap::new();
        let mut spent: HashSet<OutPoint> = HashSet::new();

        for block in blocks {
            for tx in block.get_transactions() {
                for (i, out) in tx.vout.iter().enumerate() {
                    let op = OutPoint {
                        txid: tx.id,
                        vout: i,
                    };
                    outputs.insert(
                        op,
                        UTXO {
                            out_point: (tx.id, i),
                            output: out.clone(),
                        },
                    );
                }
                for input in &tx.vin {
                    spent.insert(OutPoint {
                        txid: input.txid,
                        vout: input.vout,
                    });
                }
            }
        }
        for op in &spent {
            outputs.remove(op);
        }
        UTXOSet { map: outputs }
    }

    pub fn validate_inputs_exist(&self, tx: &Transaction) -> Result<(), String> {
        if tx.vin.is_empty() {
            return Ok(());
        }
        if !tx.verify_id() {
            return Err("transaction id does not match content".into());
        }

        for input in &tx.vin {
            let op = OutPoint {
                txid: input.txid,
                vout: input.vout,
            };
            let utxo = self
                .map
                .get(&op)
                .ok_or_else(|| format!("missing or already spent UTXO: vout={}", input.vout))?;

            let key_bytes: [u8; 32] = input
                .pub_key
                .as_slice()
                .try_into()
                .map_err(|_| "invalid pub_key length (expected 32)".to_string())?;
            let verifying_key = VerifyingKey::from_bytes(&key_bytes)
                .map_err(|e| format!("invalid pub_key: {}", e))?;

            if pub_key_hash(&verifying_key) != utxo.output.pub_key_hash {
                return Err("pub_key does not match UTXO owner".into());
            }

            let sig_bytes: [u8; 64] = input
                .signature
                .as_slice()
                .try_into()
                .map_err(|_| "invalid signature length (expected 64)".to_string())?;
            let signature = Signature::from_bytes(&sig_bytes);
            verifying_key
                .verify(tx.id.as_bytes(), &signature)
                .map_err(|_| "invalid signature".to_string())?;
        }
        Ok(())
    }

    pub fn validate_sum(&self, tx: &Transaction) -> Result<i64, String> {
        if tx.vin.is_empty() {
            return Ok(0);
        }
        let mut input_sum: i64 = 0;
        for input in &tx.vin {
            let op = OutPoint {
                txid: input.txid,
                vout: input.vout,
            };
            let utxo = self
                .map
                .get(&op)
                .ok_or_else(|| format!("missing UTXO: vout={}", input.vout))?;
            input_sum += utxo.output.value as i64;
        }
        let output_sum = tx.output_sum();
        if input_sum < output_sum {
            return Err(format!("insufficient: {} < {}", input_sum, output_sum));
        }
        Ok(input_sum - output_sum)
    }

    pub fn validate_transaction(&self, tx: &Transaction) -> Result<i64, String> {
        self.validate_inputs_exist(tx)?;
        self.validate_sum(tx)
    }

    pub fn apply(&mut self, tx: &Transaction) {
        for input in &tx.vin {
            self.map.remove(&OutPoint {
                txid: input.txid,
                vout: input.vout,
            });
        }
        for (i, out) in tx.vout.iter().enumerate() {
            self.map.insert(
                OutPoint {
                    txid: tx.id,
                    vout: i,
                },
                UTXO {
                    out_point: (tx.id, i),
                    output: out.clone(),
                },
            );
        }
    }

    pub fn balance(&self, pkh: &[u8; 32]) -> i64 {
        self.map
            .values()
            .filter(|u| &u.output.pub_key_hash == pkh)
            .map(|u| u.output.value as i64)
            .sum()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
