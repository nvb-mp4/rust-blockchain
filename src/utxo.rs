use std::collections::{HashMap, HashSet};

use crate::block::Block;
use crate::transaction::{OutPoint, Transaction, UTXO};

pub struct UTXOSet {
    map: HashMap<OutPoint, UTXO>,
}

impl UTXOSet {
    pub fn rebuild(blocks: &[Block]) -> UTXOSet {
        let mut outputs: HashMap<OutPoint, UTXO> = HashMap::new();
        let mut spent: HashSet<OutPoint> = HashSet::new();

        for block in blocks {
            for tx in block.get_transactions() {
                let tx_id = tx.id;

                for (i, out) in tx.vout.iter().enumerate() {
                    let op = OutPoint {
                        txid: tx_id,
                        vout: i,
                    };
                    outputs.insert(
                        op,
                        UTXO {
                            out_point: (tx_id, i),
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
        tx.vin.iter().try_for_each(|input| {
            let op = OutPoint {
                txid: input.txid,
                vout: input.vout,
            };
            if self.map.contains_key(&op) {
                Ok(())
            } else {
                Err(format!(
                    "missing or already spent UTXO: vout={}",
                    input.vout
                ))
            }
        })
    }

    pub fn validate_sum(&self, tx: &Transaction) -> Result<i64, String> {
        if tx.vin.is_empty() {
            return Ok(0);
        }
        let input_sum: i64 = tx
            .vin
            .iter()
            .map(|input| {
                let op = OutPoint {
                    txid: input.txid,
                    vout: input.vout,
                };
                self.map
                    .get(&op)
                    .map(|u| u.output.value as i64)
                    .ok_or_else(|| format!("missing UTXO: vout={}", input.vout))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .sum();

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

    pub fn find_by_pub_key_hash(&self, pkh: &[u8]) -> Vec<&UTXO> {
        self.map
            .values()
            .filter(|u| u.output.pub_key_hash == pkh)
            .collect()
    }

    pub fn balance(&self, pkh: &[u8]) -> i64 {
        self.find_by_pub_key_hash(pkh)
            .iter()
            .map(|u| u.output.value as i64)
            .sum()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}
