pub use super::model::*;
use std::{
    fs::{self, File},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn get_blockchain() -> Vec<Block> {
    let blockchain_file = fs::read_to_string("src/blockchain.json").unwrap();
    let blockchain: Vec<Block> = serde_json::from_str(&blockchain_file).unwrap();
    blockchain
}

pub fn write_blockchain(blockchain: Vec<Block>) {
    let blockchain_string: String = serde_json::to_string(&blockchain).unwrap();
    let mut file = File::create("src/blockchain.json").unwrap();
    file.write_all(blockchain_string.as_bytes());
}

pub fn get_transactions() -> Vec<Transaction> {
    let transactions_file = fs::read_to_string("src/transactions.json").unwrap();
    let transactions: Vec<Transaction> = serde_json::from_str(&transactions_file).unwrap();
    transactions
}

pub fn write_transactions(transaction: Vec<Transaction>) {
    let transaction_string: String = serde_json::to_string_pretty(&transaction).unwrap();
    let mut file = File::create("src/transactions.json").unwrap();
    file.write_all(transaction_string.as_bytes());
}

pub fn get_wallets() -> Vec<Wallet> {
    let wallets_file = fs::read_to_string("src/wallets.json").unwrap();
    let wallets: Vec<Wallet> = serde_json::from_str(&wallets_file).unwrap();
    wallets
}

pub fn write_wallets(wallets: Vec<Wallet>) {
    let wallets_string: String = serde_json::to_string(&wallets).unwrap();
    let mut file = File::create("src/wallets.json").unwrap();
    file.write_all(wallets_string.as_bytes());
}

pub fn get_address_balance(address: &String) -> i32 {
    let blockchain = get_blockchain();
    let mut balance = 0;

    for block in blockchain {
        for transaction in block.transactions {
            if transaction.sender_address == *address {
                balance -= transaction.amount;
            }
            if transaction.receiver_address == *address {
                balance += transaction.amount;
            }
        }
    }

    // checking mem pool for double spending
    let transactions = get_transactions();

    for transaction in transactions {
        if transaction.sender_address == *address {
            balance -= (transaction.amount + transaction.gas_fee.unwrap());
        }
    }

    balance
}

// fn get_address_balance2(address: String) -> i32 {
//     let blockchain = get_blockchain();
//     let mut balance = 0;

//     let transactions = blockchain
//         .into_iter()
//         .flat_map(|block| block.transactions)
//         .collect::<Vec<Transaction>>();

//     let result = transactions.iter().fold(0, |mut acc, t| {
//         if t.sender_address == address {
//             acc -= t.amount;
//         }
//         if t.receiver_address == address {
//             acc += t.amount;
//         }
//         acc
//     });
//     result
// }
