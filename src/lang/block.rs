use crate::lang::instr::Instr;

pub struct Block {
    id: usize,
}

impl Block {
    pub(crate) fn new(id: usize) -> Self { Block { id } }

    pub(crate) fn get_id(&self) -> usize { self.id }
}

pub(crate) struct LangBlock {
    instructions: Vec<Instr>,
}

impl LangBlock {
    pub(crate) fn new() -> Self {
        LangBlock { instructions: vec![] }
    }

    pub(crate) fn add_instr(&mut self, instruction: Instr) {
        self.instructions.push(instruction)
    }

    pub(crate) fn instructions(&mut self) -> &mut Vec<Instr> {
        &mut self.instructions
    }
}

