use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::prelude::*;

pub enum MainError {
    UpdateError,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    hash: String,
    previous_hash: String,
    nonce: i32,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    sender_address: String,
    receiver_address: String,
    gas_fee: Option<i32>,
    amount: i32,
    signature: Option<String>,
}

fn main() {
    let genesis_block = Block {
        hash: "0".to_string(),
        previous_hash: "0".to_string(),
        nonce: 0,
        transactions: vec![Transaction {
            sender_address: "0".to_string(),
            receiver_address: "0000".to_string(),
            gas_fee: None,
            amount: 190000000,
            signature: None,
        }],
    };

    write_blockchain(vec![genesis_block]);
}

fn write_blockchain(blockchain: Vec<Block>) {
    let result: Vec<String> = blockchain
        .iter()
        .map(|block| serde_json::to_string(&block).unwrap())
        .collect();

    let contents = result.join("\n");
    println!("contents: {contents}");

    let mut file = File::create("src/blockchain.json").unwrap();
    file.write_all(contents.as_bytes());
}
