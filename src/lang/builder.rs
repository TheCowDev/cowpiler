use crate::lang::block::{Block, LangBlock};
use crate::lang::instr::Instr;
use crate::lang::lang_type::Type;
use crate::lang::value::Value;

pub struct Builder {
    blocks: Vec<LangBlock>,
    values: Vec<Value>,
    current_block: usize,
}

impl Builder {
    pub fn new() -> Self {

        let mut builder = Builder { blocks: vec![], values: vec![], current_block: 0 };
        builder.blocks.push(LangBlock::new());
        builder
    }

    pub fn create_block(&mut self) -> Block {
        self.blocks.push(LangBlock::new());
        Block::new(self.blocks.len() - 1)
    }

    pub fn set_current_block(&mut self, block: Block) {
        self.current_block = block.get_id()
    }

    pub fn const_i8(&mut self, value: i8) -> Value {
        let new_value = Value::new(self.values.len(), Type::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt8 { const_value: value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i16(&mut self, value: i16) -> Value {
        let new_value = Value::new(self.values.len(), Type::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt16 { const_value: value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i32(&mut self, value: i32) -> Value {
        let new_value = Value::new(self.values.len(), Type::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt32 { const_value: value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i64(&mut self, value: i64) -> Value {
        let new_value = Value::new(self.values.len(), Type::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt64 { const_value: value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn add(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Add { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn sub(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Sub { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn mul(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Mul { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn div(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Div { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn eq(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Eq { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn diff(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Diff { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn larger(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Larger { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn larger_eq(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::LargerEq { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn smaller(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Smaller { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn smaller_eq(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::SmallerEq { left_value, right_value, gen_value: new_value.clone() };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn br(&mut self, block: Block) {
        let instr = Instr::Br { block_to_br: block };
        self.blocks[self.current_block].add_instr(instr);
    }

    pub fn cond_br(&mut self, block_true: Block, block_false: Block, value_cond: Value) {
        let instr = Instr::CondBr { block_to_br_true: block_true, block_to_br_false: block_false, value_cond };
        self.blocks[self.current_block].add_instr(instr);
    }

    pub fn ret(&mut self, value: Value) -> Value {
        let new_value = Value::new(self.values.len(), Type::i8());
        self.values.push(new_value.clone());
        let instr = Instr::Ret { value_to_return: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn ret_void(&mut self) {
        self.blocks[self.current_block].add_instr(Instr::RetVoid);
    }

    pub(crate) fn blocks(&mut self) -> &mut Vec<LangBlock> {
        &mut self.blocks
    }
}