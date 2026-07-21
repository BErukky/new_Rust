use serde_derive::Serialize;
use serde_json;
use sha2::{Digest, Sha256};
use std::fmt::Write as FmtWrite;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    timestamp: i64,
    nonce: u32,
    prev_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Chain {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Self {
        let mut chain = Self {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };
        chain.mine_genesis();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.pending_transactions.push(Transaction {
            sender,
            receiver,
            amount,
        });
        true
    }

    pub fn mine(&mut self) -> Result<(), String> {
        let reward_transaction = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut transactions = vec![reward_transaction];
        transactions.append(&mut self.pending_transactions);

        let block = self.create_block(transactions);
        self.blocks.push(block);
        println!("Mined block:");
        println!("{:#?}", self.blocks.last().unwrap());
        Ok(())
    }

    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.difficulty = difficulty;
    }

    pub fn set_reward(&mut self, reward: f32) {
        self.reward = reward;
    }

    pub fn last_hash(&self) -> String {
        self.blocks
            .last()
            .map(|block| block.hash.clone())
            .unwrap_or_else(|| "0".repeat(64))
    }

    fn mine_genesis(&mut self) {
        let genesis_transactions = vec![Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        }];
        let genesis_block = self.create_block(genesis_transactions);
        self.blocks.push(genesis_block);
        println!("Genesis block created:");
        println!("{:#?}", self.blocks[0]);
    }

    fn create_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let prev_hash = self.last_hash();
        let mut block = Block {
            timestamp: current_timestamp(),
            nonce: 0,
            prev_hash: prev_hash.clone(),
            hash: String::new(),
            transactions,
            difficulty: self.difficulty,
        };

        self.proof_of_work(&mut block);
        block.hash = Self::hash_block(&block);
        block
    }

    fn proof_of_work(&self, block: &mut Block) {
        loop {
            block.nonce += 1;
            let hash = Self::hash_block(block);
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                block.hash = hash;
                return;
            }
        }
    }

    fn hash_block(block: &Block) -> String {
        let input = serde_json::to_string(&(
            block.timestamp,
            block.nonce,
            &block.prev_hash,
            &block.transactions,
            block.difficulty,
        ))
        .unwrap();

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex_encode(&result)
    }
}

pub fn new_chain(miner_addr: String, difficulty: u32) -> Chain {
    Chain::new(miner_addr, difficulty)
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        let _ = write!(&mut output, "{byte:02x}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chain_and_mine_work() {
        let mut chain = Chain::new("miner".into(), 1);
        assert!(chain.new_transaction("alice".into(), "bob".into(), 10.0));
        assert!(chain.mine().is_ok());
    }
}



