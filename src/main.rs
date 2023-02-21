#![allow(unused)] //silence unused warnings while exploring

use secp256k1::{
    hashes::{sha256, Hash},
    rand::{rngs::StdRng, SeedableRng},
    PublicKey, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    hash, vec,
};
use std::{io::prelude::*, str::FromStr};

pub enum MainError {
    UpdateError,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hash: String,
    pub nonce: i32,
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

const GENESIS_ADDRESS: &str = "0000";
const BLOCK_REWARD: i32 = 50;

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
    mine_blocks();
    mine_blocks();
}

fn write_blockchain(blockchain: Vec<Block>) {
    let blockchain_string: String = serde_json::to_string(&blockchain).unwrap();
    println!("blockchain_string: {blockchain_string}");
    let mut file = File::create("src/blockchain.json").unwrap();
    file.write_all(blockchain_string.as_bytes());
}

pub fn get_wallets() -> Vec<Wallet> {
    let wallets_file = fs::read_to_string("src/wallets.json").unwrap();
    let wallets: Vec<Wallet> = serde_json::from_str(&wallets_file).unwrap();
    wallets
}

pub fn get_blockchain() -> Vec<Block> {
    let blockchain_file = fs::read_to_string("src/blockchain.json").unwrap();
    let blockchain: Vec<Block> = serde_json::from_str(&blockchain_file).unwrap();
    blockchain
}

fn write_wallets(wallets: Vec<Wallet>) {
    let wallets_string: String = serde_json::to_string(&wallets).unwrap();
    println!("wallets_string: {wallets_string}");
    let mut file = File::create("src/wallets.json").unwrap();
    file.write_all(wallets_string.as_bytes());
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

fn mine_blocks() {
    let mut blockchain = get_blockchain();
    let mut list_of_transactions: Vec<Transaction> = Vec::new();

    let miner_public_key = get_wallets().get(0).unwrap().public_key.clone();
    println!(
        "get_address_balance 1: {}",
        get_address_balance("0000".to_string())
    );
    println!(
        "get_address_balance 2: {}",
        get_address_balance2("0000".to_string())
    );

    let is_supply_available = get_address_balance(GENESIS_ADDRESS.to_string()) > BLOCK_REWARD;

    let reward_transaction = Transaction {
        sender_address: GENESIS_ADDRESS.to_string(),
        receiver_address: miner_public_key,
        amount: BLOCK_REWARD,
        gas_fee: None,
        signature: None,
    };

    if is_supply_available {
        list_of_transactions.push(reward_transaction);
    }

    if let Some(block) = blockchain.last() {
        let previous_hash = block.hash.clone();
        let mut new_hash = String::new();
        let mut nonce = 0;
        while !new_hash.starts_with("00") {
            nonce += 1;
            let phrase = format!(
                "{}{}{}",
                nonce,
                previous_hash,
                serde_json::to_string(&list_of_transactions).unwrap()
            );
            new_hash = sha256::Hash::hash(phrase.as_bytes()).to_string();
            println!("{new_hash}");
        }

        let new_block = Block {
            hash: new_hash,
            previous_hash,
            nonce,
            transactions: list_of_transactions,
        };

        blockchain.push(new_block);

        write_blockchain(blockchain);
    }
}

fn get_address_balance(address: String) -> i32 {
    let blockchain = get_blockchain();
    let mut balance = 0;

    for block in blockchain {
        for transaction in block.transactions {
            if transaction.sender_address == address {
                balance -= transaction.amount;
            }
            if transaction.receiver_address == address {
                balance += transaction.amount;
            }
        }
    }

    balance
}

fn get_address_balance2(address: String) -> i32 {
    let blockchain = get_blockchain();
    let mut balance = 0;

    let transactions = blockchain
        .into_iter()
        .flat_map(|block| block.transactions)
        .collect::<Vec<Transaction>>();

    let result = transactions.iter().fold(0, |acc, t| {
        if t.sender_address == address {
            acc - t.amount
        } else if t.receiver_address == address {
            acc + t.amount
        } else {
            acc
        }
    });
    result
}
