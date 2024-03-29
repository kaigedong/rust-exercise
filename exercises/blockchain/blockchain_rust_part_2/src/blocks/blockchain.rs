use crate::Block;
use tracing::info;

// 将当前计算难度硬编码为8
const CURR_BITS: usize = 8;

pub struct Blockchain {
    blocks: Vec<Block>,
    height: usize,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis_block(CURR_BITS)],
            height: 0,
        }
    }

    pub fn mine_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let block = Block::new(data, prev_block.get_hash().as_str(), CURR_BITS);
        self.blocks.push(block);
        self.height += 1;
    }

    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
        }
    }
}
