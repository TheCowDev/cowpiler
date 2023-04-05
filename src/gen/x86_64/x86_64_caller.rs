use crate::gen::x86_64::x86_64_allocator::{X86_64Allocator, X86Register};
use crate::gen::x86_64::x86_64_encoder::X86_64Encoder;
use crate::lang::value::Value;

pub(crate) struct X86_64Caller {
    args_register: Vec<X86Register>,
    args_xmm: Vec<X86Register>,
    volatiles: Vec<X86Register>,
}

impl X86_64Caller {
    pub(crate) fn new() -> Self {
        let mut args = vec![];
        let mut xmms = vec![];
        let mut volatiles = vec![];

        let os = if cfg!(target_os = "windows") {
            args = vec![X86Register::RCX, X86Register::RDX, X86Register::R8, X86Register::R9];
            xmms = vec![X86Register::XMM0, X86Register::XMM1, X86Register::XMM2, X86Register::XMM3];
            volatiles = vec![X86Register::RAX, X86Register::RCX, X86Register::RDX, X86Register::R8,
                             X86Register::R9, X86Register::R10, X86Register::R11];
        } else { // we suppose that every other platform use the linux calling convention
            args = vec![X86Register::RDI, X86Register::RSI, X86Register::RDX, X86Register::RCX, X86Register::R8, X86Register::R9];
            xmms = vec![X86Register::XMM0, X86Register::XMM1, X86Register::XMM2, X86Register::XMM3,
                        X86Register::XMM4, X86Register::XMM5, X86Register::XMM6, X86Register::XMM7];
            volatiles = vec![X86Register::RAX, X86Register::RCX, X86Register::RDX, X86Register::RSI, X86Register::RDI, X86Register::R8, X86Register::R9,
                             X86Register::R10, X86Register::R11,
                             X86Register::XMM0, X86Register::XMM1, X86Register::XMM2, X86Register::XMM3,
                             X86Register::XMM4, X86Register::XMM5, X86Register::XMM6,
                             X86Register::XMM7, X86Register::XMM8, X86Register::XMM9,
                             X86Register::XMM10, X86Register::XMM11, X86Register::XMM12,
                             X86Register::XMM13, X86Register::XMM14, X86Register::XMM15];
        };

        Self { args_register: args, args_xmm: xmms, volatiles }
    }

    pub(crate) fn generate_call(&self, encoder: &mut X86_64Encoder, allocator: &mut X86_64Allocator, values: &Vec<Value>) -> Vec<X86Register> {
        let mut pushed_register = vec![];

        // to keep track of what argument we are working on at the moment
        let mut arg_index = 0;
        let mut xmm_index = 0;

        for value in values {
            let reg = allocator.obtain_register_for_value(value.clone());

            if reg.is_xmm() {
                let reg_to_mov = self.args_xmm[xmm_index];
                if allocator.is_register_allocated(reg_to_mov) && reg_to_mov != reg {
                    encoder.push_reg(reg_to_mov);
                    pushed_register.push(reg_to_mov);
                }
                encoder.mov_xmm_to_xmm(reg, reg_to_mov);
                xmm_index += 1;
            } else {
                let reg_to_mov = self.args_register[arg_index];
                if allocator.is_register_allocated(reg_to_mov) && reg_to_mov != reg {
                    encoder.push_reg(reg_to_mov);
                    pushed_register.push(reg_to_mov);
                }
                encoder.mov_reg_to_reg(reg, reg_to_mov);
                arg_index += 1;
            }
        }

        pushed_register
    }
}