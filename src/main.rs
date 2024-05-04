use crate::block::Block;

pub mod block;

fn main() {
    println!("Hello, world!");
    let mut blockchain: Vec<Block> = Vec::new();

    let mut last_block = Block::generate_genesis_block();
    
    blockchain.push(last_block.clone());

    for _ in 1..=10 {
        let block_to_add = Block::next_block(&last_block);
        blockchain.push(block_to_add.clone());
        last_block = block_to_add;
        println!("Block #{} has been added", last_block.index);
        println!("Hash: {:}", last_block.hash);
    }
}
