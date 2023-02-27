//silence unused warnings while exploring
#![allow(unused)]
use secp256k1::{
    ecdsa::Signature,
    hashes::{sha256, Hash},
    rand::{rngs::OsRng, SeedableRng},
    Message, PublicKey, SecretKey,
};
use std::{
    fs::{self, File},
    hash,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};
use std::{io::prelude::*, str::FromStr};
use utils::*;
use uuid::Uuid;

mod model;
mod utils;

pub enum MainError {
    UpdateError,
}

const GENESIS_ADDRESS: &str = "0000";
const BLOCK_REWARD: i32 = 50;
//TODO: Add this
const MINIMUM_GAS_FEE: i32 = 1;

fn main() {
    init();
    generate_wallets(3);
    let wallets = get_wallets();
    let miner = wallets.get(0).unwrap();
    let sender = wallets.get(1).unwrap();
    let receiver = wallets.get(2).unwrap();
    mine_block(miner.public_key.clone());
    mine_block(miner.public_key.clone());
    mine_block(miner.public_key.clone());
    transfer(
        SecretKey::from_str(&miner.secret_key).unwrap(),
        100,
        2,
        sender.public_key.clone(),
    );
    transfer(
        SecretKey::from_str(&miner.secret_key).unwrap(),
        30,
        2,
        receiver.public_key.clone(),
    );
    mine_block(miner.public_key.clone());
    transfer(
        SecretKey::from_str(&sender.secret_key).unwrap(),
        50,
        1,
        receiver.public_key.clone(),
    );
    transfer(
        SecretKey::from_str(&receiver.secret_key).unwrap(),
        20,
        1,
        sender.public_key.clone(),
    );
    mine_block(miner.public_key.clone());
    println!("{}", get_address_balance(&miner.public_key));
    println!("{}", get_address_balance(&sender.public_key));
    println!("{}", get_address_balance(&receiver.public_key));
}

fn init() {
    let genesis_block = Block {
        block_number: 0,
        block_timestamp: get_timestamp(),
        hash: "0".to_string(),
        previous_hash: "0".to_string(),
        nonce: 0,
        transactions: vec![Transaction {
            transaction_id: Uuid::new_v4().to_string(),
            transaction_timestamp: get_timestamp(),
            sender_address: "0".to_string(),
            receiver_address: "0000".to_string(),
            gas_fee: None,
            amount: 190000000,
            signature: None,
        }],
    };

    write_blockchain(vec![genesis_block]);
    write_transactions(vec![]);
}

fn verify_transaction(transaction: &Transaction) -> bool {
    let secp = secp256k1::Secp256k1::new();
    secp.verify_ecdsa(
        &Message::from_hashed_data::<sha256::Hash>(
            (transaction.sender_address.clone() + &transaction.amount.to_string()).as_bytes(),
        ),
        &Signature::from_str(&transaction.signature.clone().unwrap()).unwrap(),
        &PublicKey::from_str(&transaction.sender_address).unwrap(),
    )
    .is_ok()
}

fn generate_wallets(amount: i32) {
    let mut wallets: Vec<Wallet> = Vec::new();
    let secp = secp256k1::Secp256k1::new();
    for i in 0..amount {
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        let wallet = Wallet {
            secret_key: secret_key.display_secret().to_string(),
            public_key: public_key.to_string(),
        };

        wallets.push(wallet)
    }

    write_wallets(wallets);
}

fn mine_block(miner_public_key: String) {
    let mut blockchain = get_blockchain();
    let mut list_of_transactions: Vec<Transaction> = get_transactions();
    // let miner_public_key = get_wallets().get(0).unwrap().public_key.clone();

    let is_supply_available = get_address_balance(&GENESIS_ADDRESS.to_string()) > BLOCK_REWARD;

    let reward_transaction = Transaction {
        transaction_id: Uuid::new_v4().to_string(),
        transaction_timestamp: get_timestamp(),
        sender_address: GENESIS_ADDRESS.to_string(),
        receiver_address: miner_public_key.clone(),
        amount: BLOCK_REWARD,
        gas_fee: None,
        signature: None,
    };

    if is_supply_available {
        list_of_transactions.push(reward_transaction);
    }

    let mut gas_transactions: Vec<Transaction> = list_of_transactions
        .iter()
        .filter(|t| t.gas_fee.is_some())
        .map(|t| Transaction {
            transaction_id: Uuid::new_v4().to_string(),
            transaction_timestamp: get_timestamp(),
            sender_address: t.sender_address.clone(),
            receiver_address: miner_public_key.clone(),
            amount: t.gas_fee.unwrap(),
            gas_fee: None,
            signature: None,
        })
        .collect();

    list_of_transactions.append(&mut gas_transactions);

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
        }

        let new_block_number = block.block_number + 1;

        let new_block = Block {
            block_number: new_block_number,
            block_timestamp: get_timestamp(),
            hash: new_hash,
            previous_hash,
            nonce,
            transactions: list_of_transactions,
        };

        blockchain.push(new_block);

        write_blockchain(blockchain);
        write_transactions(vec![]);
    }
}

//TODO: Change private key to string
fn transfer(sender_private_key: SecretKey, amount: i32, gas_fee: i32, receiver_public_key: String) {
    let secp = secp256k1::Secp256k1::new();
    let sender_public_address = sender_private_key.public_key(&secp).to_string();
    if get_address_balance(&sender_public_address) >= amount + gas_fee {
        let message = Message::from_hashed_data::<sha256::Hash>(
            (sender_public_address.clone() + &amount.to_string()).as_bytes(),
        );
        let signature = secp.sign_ecdsa(&message, &sender_private_key);

        let transaction = Transaction {
            transaction_id: Uuid::new_v4().to_string(),
            transaction_timestamp: get_timestamp(),
            sender_address: sender_public_address,
            receiver_address: receiver_public_key,
            gas_fee: Some(gas_fee),
            amount,
            signature: Some(signature.to_string()),
        };

        let mut current_transactions = get_transactions();
        current_transactions.push(transaction);
        write_transactions(current_transactions);
    }
}

// Run consecutively using cargo test -- --test-threads=1
// TODO: Should use separate files for tests
// or change write and get functions to be generic and pass them a filename
//https://docs.rs/tempdir/latest/tempdir/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_wallets_ok() {
        //given
        //when
        generate_wallets(2);

        //then
        let wallets = get_wallets();
        assert_eq!(wallets.len(), 2);
    }

    #[test]
    fn init_ok() {
        //given

        //when
        init();
        //then
        let blockchain = get_blockchain();
        assert_eq!(blockchain.len(), 1);
        assert_eq!(blockchain.get(0).unwrap().hash, "0".to_string());
    }

    #[test]
    fn mine_blocks_ok() {
        //given
        generate_wallets(1);
        //when
        init();
        mine_block(get_wallets().get(0).unwrap().public_key.clone());

        //then
        let blockchain = get_blockchain();
        assert_eq!(blockchain.len(), 2);
    }

    #[test]
    fn transfer_ok() {
        //given
        generate_wallets(2);
        let wallets = get_wallets();
        let sender = wallets.get(0).unwrap();
        let receiver = wallets.get(1).unwrap();

        //when
        init();
        mine_block(sender.public_key.clone());
        transfer(
            SecretKey::from_str(&sender.secret_key).unwrap(),
            40,
            1,
            receiver.public_key.clone(),
        );
        mine_block(sender.public_key.clone());

        //then
        assert_eq!(get_address_balance(&sender.public_key), 60);
        assert_eq!(get_address_balance(&receiver.public_key), 40);
    }
}
