#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TxId(pub [u8; 32]);

impl TxId {
    pub fn from_slice(v: &[u8]) -> Option<TxId> {
        <[u8; 32]>::try_from(v).ok().map(TxId)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OutPoint {
    pub txid: TxId,
    pub vout: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TXInput {
    pub txid: TxId,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TxId,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UTXO {
    pub out_point: (TxId, usize),
    pub output: TXOutput,
}

impl Transaction {
    pub fn hash(&self) -> TxId {
        let for_hash = Transaction {
            id: TxId::default(),
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        let data = postcard::to_stdvec(&for_hash).expect("failed to serialize transaction");
        let raw = crate::sha256_digest(&data);
        TxId::from_slice(&raw).expect("sha256 digest is always 32 bytes")
    }

    pub fn new_utxo_transaction(vin: Vec<TXInput>, vout: Vec<TXOutput>) -> Transaction {
        let mut tx = Transaction {
            id: TxId::default(),
            vin,
            vout,
        };
        tx.id = tx.hash();
        tx
    }

    pub fn verify_id(&self) -> bool {
        self.id == self.hash()
    }

    pub fn output_sum(&self) -> i64 {
        self.vout.iter().map(|o| o.value as i64).sum()
    }
}
