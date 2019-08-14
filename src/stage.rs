use crate::block::Block;

#[derive(Clone, Copy)]
pub struct Stage {
    pub level: u8
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            level: 1
        }
    }
}
impl Stage {
    pub fn new(level: u8) -> Self {
        Stage {
            level
        }
    }

    pub fn level_up(&mut self, lv: u8) {
        self.level = self.level + lv;
    }

    pub fn level_down(&mut self, lv: u8) {
        self.level = self.level - lv;
    }

    pub fn gen_blocks(self, width: f64, height: f64) -> Vec<Box<Block>> {
        let mut blocks = Vec::new();
        for _i in 0..self.get_block_number() {
            blocks.push(Box::new(Block::new_rand(width, height)));
        }
        blocks
    }

    fn get_block_number(self) -> u8 {
        let block_number: u8 = match self.level as u8 {
            0..=10 => self.level,
            11..=20 => self.level * 2,
            21..=30 => self.level * 3,
            _ => self.level * 5
        } as u8;

        block_number
    }
}