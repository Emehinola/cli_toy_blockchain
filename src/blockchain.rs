extern crate time;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

use serde_derive::Serialize;
use sha2::{Sha256, Digest};
use std::fmt::Write;
use time::OffsetDateTime;

#[derive (Debug, Clone, Serialize)]
struct Transaction {
    sender:  String,
    receiver: String,
    amount: f32
}

#[derive (Debug, Serialize)]
pub struct BlockHeader {
    timestamp: u8,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32
}

#[derive (Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}

pub struct Chain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32
}

impl Chain {

    pub fn new(miner_addr: String , difficulty: u32) -> Chain {
        let mut  chain: Chain = Chain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
            difficulty: difficulty,
            miner_addr: miner_addr,
            reward: 100.0
        };

        chain.generate_new_block();
        return chain;
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        // current transaction waits in the mempool for new block to be created so it can be added
        self.current_transactions.push(
            Transaction { 
                sender: sender, 
                receiver: receiver, 
                amount: amount 
            }
        );
        return true;
    }

    pub fn last_hash(&mut self) -> String {
        let block: &Block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap() // prefill with 48 zeros if no last chain is found
        };

        return Chain::hash(&block.header);
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        return true;
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        return true;
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header: BlockHeader = BlockHeader { 
            timestamp: OffsetDateTime::now_utc().second(),
            nonce: 0, pre_hash: self.last_hash(), difficulty: self.difficulty,
            merkle: String::new()
        };

        let reward_trans: Transaction = Transaction { 
            sender: String::from("Root"), 
            receiver: self.miner_addr.clone(), 
            amount: self.reward 
        };

        let mut  block: Block = Block { header: header, count: 0, transactions: vec![] };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.current_transactions);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);

        print!("{:#?}", &block);
        self.chain.push(block);

        return true;
    }

    fn get_merkle(current_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &current_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 { // check if the length of the merkle is odd
            let last = merkle.last().cloned().unwrap(); // .last is Option type, hence the the .cloned()
            merkle.push(last);
        }

        while merkle.len() > 1 {
            // remove first 2 hashes
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);

            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }

        return merkle.pop().unwrap();
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());

        let res = hasher.finalize();
        let vec_result = res.to_vec();

        return Chain::hex_to_string(vec_result.as_slice());
    }

    pub fn hex_to_string(vec_result: &[u8]) -> String {
        let mut s = String::new();

        for b in vec_result {
            write!(&mut s, "{:x}", b).expect("Unable to write");
        };
        return s;
    }
}