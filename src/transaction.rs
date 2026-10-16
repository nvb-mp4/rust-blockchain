use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl Transaction {
    pub fn get_id(&self) -> Vec<u8> {
        self.id.clone()
    }

    pub fn hash(&self) -> Vec<u8> {
        let encoded = bincode::serialize(self).expect("tx serialize failed");
        crate::sha256_digest(encoded.as_slice())
    }

    pub fn new_utxo_transaction(vin: Vec<TXInput>, vout: Vec<TXOutput>) -> Transaction {
        let mut tx = Transaction {
            id: Vec::new(),
            vin,
            vout,
        };
        tx.id = tx.hash();
        tx
    }
}
