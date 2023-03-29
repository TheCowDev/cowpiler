use crate::lang::instr::Instr;

pub struct Block {
    instructions: Vec<Instr>,
}

impl Block {
    pub(crate) fn new() -> Self {
        Block { instructions: vec![] }
    }

    pub(crate) fn add_instr(&mut self, instruction: Instr) {
        self.instructions.push(instruction)
    }
}

