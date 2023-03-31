use crate::lang::instr::Instr;

#[derive(Clone, Copy)]
pub struct Block {
    id: usize,
}

impl Block {
    pub(crate) fn new(id: usize) -> Self { Block { id } }

    pub(crate) fn get_id(&self) -> usize { self.id }
}

pub(crate) struct LangBlock {
    instructions: Vec<Instr>,
    offset: usize,
}

impl LangBlock {
    pub(crate) fn new() -> Self {
        LangBlock { instructions: vec![], offset: 0 }
    }

    pub(crate) fn add_instr(&mut self, instruction: Instr) {
        self.instructions.push(instruction)
    }

    pub(crate) fn instructions(&mut self) -> &mut Vec<Instr> {
        &mut self.instructions
    }

    pub(crate) fn set_offset(&mut self, offset: usize) {
        self.offset = offset
    }

    pub(crate) fn offset(&mut self) -> usize {
        self.offset
    }
}

