use std::collections::HashMap;
use std::vec;
use crate::gen::x86_64::x86_64_allocator::{X86_64Allocator, X86Register};
use crate::gen::x86_64::x86_64_encoder::X86_64Encoder;
use crate::lang::block::LangBlock;
use crate::lang::function::{Function};
use crate::lang::instr::Instr;
use crate::misc::byte_writer::ByteWriter;

#[derive(Clone)]
struct BlockOffset {
    offset: usize,
    block: usize,
}

#[derive(Clone)]
pub struct FunctionOffset {
    func_name: String,
    offset: usize,
    func_id: usize,
}

pub(crate) struct X86_64Gen {}

impl X86_64Gen {
    pub(crate) fn new() -> Self {
        X86_64Gen {}
    }


    pub(crate) fn gen(&mut self, funcs: &mut HashMap<String, Function>) {
        let mut func_offsets = vec![];

        // generate the function
        for func in funcs.values_mut() {
            self.gen_func(func, &mut func_offsets)
        }

        // replace the offset of all functions
        for offset in func_offsets {}
    }

    fn gen_func(&mut self, func: &mut Function, func_offsets: &mut Vec<FunctionOffset>) {
        let mut func_offset: Vec<FunctionOffset> = vec![];
        let mut block_offset: Vec<BlockOffset> = vec![];
        let func_name = func.name().clone();
        let builder = func.builder();
        let mut allocator = X86_64Allocator::new();
        let mut encoder = X86_64Encoder::new();

        for block in builder.blocks() {
            block.set_offset(encoder.bytes().len());
            for instr in block.instructions() {
                self.gen_instr(&func_name, instr, &mut allocator, &mut encoder, &mut func_offset, &mut block_offset);
            }
        }

        let mut writer = ByteWriter::from_bytes(encoder.bytes());

        for offset in block_offset {
            writer.rewrite_i32(offset.offset, builder.blocks()[offset.block].offset() as i32 - offset.offset as i32 - 4);
        }

        func.set_code(writer.bytes())
    }

    fn gen_instr(&mut self, func_name: &String, instr: &mut Instr, allocator: &mut X86_64Allocator, encode: &mut X86_64Encoder,
                 func_offsets: &mut Vec<FunctionOffset>, block_offsets: &mut Vec<BlockOffset>) {
        match instr {
            Instr::ConstInt128 { .. } => {}

            Instr::ConstInt64 { const_value, gen_value } => {
                let const_reg = allocator.obtain_register_for_value(gen_value.clone());
                encode.move_reg_i64(const_reg, *const_value);
            }

            Instr::ConstInt32 { .. } => {}

            Instr::ConstInt16 { .. } => {}

            Instr::ConstInt8 { .. } => {}

            Instr::ConstPtr { .. } => {}

            Instr::Add { left_value, right_value, gen_value } => {
                let left_reg = allocator.obtain_register_for_value(left_value.clone());
                let right_reg = allocator.obtain_register_for_value(right_value.clone());
                let result_reg = allocator.allocate_register(gen_value.clone()).unwrap();
                if left_reg.is_xmm() {
                    encode.add_reg_reg(left_reg, right_reg);
                } else {
                    encode.add_reg_reg(left_reg, right_reg);
                    encode.mov_reg_to_reg(left_reg, result_reg);
                }
            }

            Instr::Sub { .. } => {}

            Instr::Div { .. } => {}

            Instr::Mul { .. } => {}

            Instr::Eq { .. } => {}

            Instr::Diff { .. } => {}

            Instr::Larger { .. } => {}

            Instr::LargerEq { .. } => {}

            Instr::Smaller { .. } => {}

            Instr::SmallerEq { .. } => {}

            Instr::Not { .. } => {}

            Instr::Load { .. } => {}

            Instr::Store { .. } => {}

            Instr::Br { block_to_br } => {
                let offset = BlockOffset { block: block_to_br.get_id(), offset: encode.jmp() };
                block_offsets.push(offset);
            }

            Instr::CondBr { .. } => {}

            Instr::CallPtr { .. } => {}

            Instr::CallFunc { .. } => {}

            Instr::Ret { value_to_return } => {
                let reg_to_return = allocator.obtain_register_for_value(value_to_return.clone());
                if value_to_return.get_type().is_float() {
                    encode.mov_reg_to_reg(reg_to_return, X86Register::XMM0);
                } else {
                    encode.mov_reg_to_reg(reg_to_return, X86Register::RAX);
                }

                encode.ret();
            }

            Instr::RetVoid => {}
        }
    }
}