use crate::lang::block::Block;
use crate::lang::instr::Instr;
use crate::lang::lang_type::LangType;
use crate::lang::value::Value;

pub struct Builder {
    blocks: Vec<Block>,
    values: Vec<Value>,
    current_block: usize,
}

impl Builder {
    pub fn new() -> Self {
        Builder { blocks: vec![], values: vec![], current_block: 0 }
    }

    pub fn create_block(&mut self) -> &mut Block {
        let mut new_block = Block::new();
        self.blocks.push(new_block);
        self.blocks.last().as_mut().unwrap()
    }

    pub fn const_i8(&mut self, value: i8) -> Value {
        let new_value = Value::new(self.values.len(), LangType::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt8 { const_value: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i16(&mut self, value: i16) -> Value {
        let new_value = Value::new(self.values.len(), LangType::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt16 { const_value: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i32(&mut self, value: i32) -> Value {
        let new_value = Value::new(self.values.len(), LangType::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt32 { const_value: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn const_i64(&mut self, value: i64) -> Value {
        let new_value = Value::new(self.values.len(), LangType::i8());
        self.values.push(new_value.clone());
        let instr = Instr::ConstInt64 { const_value: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn add(&mut self, left_value: Value, right_value: Value) -> Value {
        let new_value = Value::new(self.values.len(), left_value.get_type());
        self.values.push(new_value.clone());
        let instr = Instr::Add { left_value, right_value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn ret(&mut self, value: Value) -> Value {
        let new_value = Value::new(self.values.len(), LangType::i8());
        self.values.push(new_value.clone());
        let instr = Instr::Ret { value_to_return: value };
        self.blocks[self.current_block].add_instr(instr);
        new_value
    }

    pub fn ret_void(&mut self) {
        self.blocks[self.current_block].add_instr(Instr::RetVoid);
    }
}