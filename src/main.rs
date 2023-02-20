#![allow(unused)] //silence unused warnings while exploring

use secp256k1::{
    rand::{rngs::StdRng, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::{
    fs::{self, File},
    vec,
};

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

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    secret_key: String,
    public_key: String,
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
    generate_wallets();
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

pub fn get_wallets() -> Vec<Wallet> {
    let wallets_file = fs::read_to_string("src/wallets.json").unwrap();
    let wallets: Vec<Wallet> = serde_json::from_str(&wallets_file).unwrap();
    wallets
}

fn write_wallets(wallets: Vec<Wallet>) {
    let result: Vec<String> = wallets
        .iter()
        .map(|wallet| serde_json::to_string(&wallet).unwrap())
        .collect();

    let contents = result.join("\n");
    println!("contents: {contents}");

    let mut file = File::create("src/wallets.json").unwrap();
    file.write_all(contents.as_bytes());
}

fn generate_wallets() {
    let secp = secp256k1::Secp256k1::new();
    let mut rng = StdRng::seed_from_u64(111);
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    println!("secret key: {}", secret_key.display_secret());
    println!("public key: {public_key}");

    let wallet = Wallet {
        secret_key: secret_key.display_secret().to_string(),
        public_key: public_key.to_string(),
    };

    write_wallets(vec![wallet]);
}
