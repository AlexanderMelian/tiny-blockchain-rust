use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::time::SystemTime;

pub struct Block {
	pub index: u32,
	timestamp: u64,
	data: String,
	previous_hash: String,
	pub hash: String
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block {
            index: self.index,
            timestamp: self.timestamp,
            data: self.data.clone(),
            previous_hash: self.previous_hash.clone(),
            hash: self.hash.clone(),
        }
    }
}

impl Block {
	pub fn new(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
		let hash = Block::hash_block_generate(index, timestamp, &data, &previous_hash);
		Block { index, timestamp, data, previous_hash, hash}
	}
	
	fn hash_block_generate(index: u32, timestamp: u64, data: &str, previous_hash: &str) -> String {
		let mut sha = Sha256::new();
		let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
		sha.input_str(&input);
		return sha.result_str();
	}

	pub fn generate_genesis_block() -> Block {
		return Block::new(0, get_sys_time_in_secs(), String::from("Genesis Block"), String::from("0"));
	}

	pub fn next_block(last_block: &Block) -> Block{
		let index = last_block.index+1;
		let timestamp = get_sys_time_in_secs();
		let data = format!("this_block: {}", index);
		let previous_hash = &last_block.hash;
		return Block::new(index, timestamp, data, previous_hash.to_string());
	}



}
fn get_sys_time_in_secs() -> u64 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n) => n.as_secs(),
		Err(_) => panic!("SystemTime before UNIX EPOCH!"),
	}
}