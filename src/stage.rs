use crate::block::Block;

pub struct Stage {
    pub level: usize
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            level: 1
        }
    }
}
impl Stage {
    pub fn new(level: usize) -> Self {
        Stage {
            level
        }
    }

    pub fn gen_blocks(self, width: f64, height: f64) -> Vec<Box<Block>> {
        let mut blocks = Vec::new();
        for _i in 0..(self.level + 1) {
            blocks.push(Box::new(Block::new_rand(width, height)));
        }
        blocks
    }
}