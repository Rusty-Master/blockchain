use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub block_number: i32,
    pub block_timestamp: u64,
    pub hash: String,
    pub previous_hash: String,
    pub nonce: i32,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: u64,
    pub sender_address: String,
    pub receiver_address: String,
    pub gas_fee: Option<i32>,
    pub amount: i32,
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub secret_key: String,
    pub public_key: String,
}
