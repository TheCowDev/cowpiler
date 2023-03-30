use std::collections::HashMap;
use std::process::id;
use crate::lang::value::Value;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum X86Register {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RDI,
    RSI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
    XMM8,
    XMM9,
    XMM10,
    XMM11,
    XMM12,
    XMM13,
    XMM14,
    XMM15,
}

pub(crate) struct X86_64Allocator {
    free_registers: Vec<X86Register>,
    allocated_registers: HashMap<usize, X86Register>,
}

impl X86_64Allocator {
    pub(crate) fn new() -> Self {
        X86_64Allocator { free_registers: vec![], allocated_registers: HashMap::new() }
    }

    pub(crate) fn obtain_register_for_value(&mut self, value: Value) -> X86Register {

        // the case where a register is available to put the value inn
        if value.get_type().is_float() {
            if let Some(idx) = self.free_registers.iter().position(|reg| reg.is_xmm()) {
                return self.allocate_register(value, self.free_registers[idx]);
            }
        } else {
            if let Some(idx) = self.free_registers.iter().position(|reg| !reg.is_xmm()) {
                return self.allocate_register(value, self.free_registers[idx]);
            }
        }

        // the case where no register is available
        return X86Register::RAX;
    }

    pub(crate) fn free_register_from_value(&mut self, value: Value) -> bool {
        let id = value.get_id();
        if self.allocated_registers.contains_key(&id) {
            let register_to_free = self.allocated_registers.get(&id).unwrap();
            self.free_registers.push(register_to_free.clone());
            return true;
        }

        false
    }

    fn allocate_register(&mut self, value: Value, reg: X86Register) -> X86Register {
        self.free_registers.retain(|&current| current != reg);
        self.allocated_registers.insert(value.get_id(), reg);
        reg
    }
}

impl X86Register {
    pub(crate) fn is_xmm(&self) -> bool {
        vec![X86Register::XMM0, X86Register::XMM1, X86Register::XMM2, X86Register::XMM3,
             X86Register::XMM4, X86Register::XMM5, X86Register::XMM6, X86Register::XMM7,
             X86Register::XMM8, X86Register::XMM9, X86Register::XMM10, X86Register::XMM11,
             X86Register::XMM12, X86Register::XMM13, X86Register::XMM14, X86Register::XMM15]
            .contains(self)
    }

    pub(crate) fn encode(&self) -> u8 {
        match self {
            X86Register::RAX => { 0 }
            X86Register::RCX => { 1 }
            X86Register::RDX => { 2 }
            X86Register::RBX => { 3 }
            X86Register::RSP => { 4 }
            X86Register::RBP => { 5 }
            X86Register::RDI => { 6 }
            X86Register::RSI => { 7 }
            X86Register::R8 => { 8 }
            X86Register::R9 => { 9 }
            X86Register::R10 => { 10 }
            X86Register::R11 => { 11 }
            X86Register::R12 => { 12 }
            X86Register::R13 => { 13 }
            X86Register::R14 => { 14 }
            X86Register::R15 => { 15 }
            X86Register::XMM0 => { 0 }
            X86Register::XMM1 => { 1 }
            X86Register::XMM2 => { 2 }
            X86Register::XMM3 => { 3 }
            X86Register::XMM4 => { 4 }
            X86Register::XMM5 => { 5 }
            X86Register::XMM6 => { 6 }
            X86Register::XMM7 => { 7 }
            X86Register::XMM8 => { 8 }
            X86Register::XMM9 => { 9 }
            X86Register::XMM10 => { 10 }
            X86Register::XMM11 => { 11 }
            X86Register::XMM12 => { 12 }
            X86Register::XMM13 => { 13 }
            X86Register::XMM14 => { 14 }
            X86Register::XMM15 => { 15 }
        }
    }
}