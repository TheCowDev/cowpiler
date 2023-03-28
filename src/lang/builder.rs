use crate::lang::block::Block;

pub struct Builder {
    blocks: Vec<Block>,
    current_block: usize,
}

impl Builder {
    pub fn new() -> Self {
        Builder { blocks: vec![], current_block: 0 }
    }
}