use std::collections::HashMap;
use crate::gen::x86_64::x86_64_allocator::{X86_64Allocator, X86Register};
use crate::gen::x86_64::x86_64_encoder::X86_64Encoder;
use crate::lang::function::Function;
use crate::lang::instr::Instr;

pub(crate) struct X86_64Gen {}

impl X86_64Gen {
    pub(crate) fn new() -> Self {
        X86_64Gen {}
    }


    pub(crate) fn gen(&mut self, funcs: &mut HashMap<String, Function>) {
        for func in funcs.values_mut() {
            self.gen_func(func)
        }
    }

    fn gen_func(&mut self, func: &mut Function) {
        let mut builder = func.builder();
        let mut allocator = X86_64Allocator::new();
        let mut encoder = X86_64Encoder::new();

        for block in builder.blocks() {
            for instr in block.instructions() {
                self.gen_instr(instr, &mut allocator, &mut encoder);
            }
        }

        func.set_code(encoder.bytes())
    }

    fn gen_instr(&mut self, instr: &mut Instr, allocator: &mut X86_64Allocator, encode: &mut X86_64Encoder) {
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

            Instr::Add { .. } => {}

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

            Instr::Br { .. } => {}

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