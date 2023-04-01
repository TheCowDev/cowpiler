use std::collections::HashMap;
use crate::lang;
use lang::function::Function;
use lang::lang_type::Type;
use crate::gen::x86_64::gen::X86_64Gen;
use std::os::raw::{c_void, c_ulong};

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RELEASE: u32 = 0x8000;

use std::mem::transmute;

extern "system" {
    fn VirtualAlloc(lpAddress: *mut c_void, dwSize: usize, flAllocationType: u32, flProtect: u32) -> *mut c_void;
    fn VirtualFree(lpAddress: *mut c_void, dwSize: usize, dwFreeType: u32) -> i32;
}

fn create_executable_function(function_bytes: &Vec<u8>) -> *mut u8 {
    let length = function_bytes.len();

    let addr = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            length,
            MEM_COMMIT,
            PAGE_EXECUTE_READWRITE,
        )
    };

    let addr_ptr = addr as *mut u8;

    for (i, byte) in function_bytes.iter().enumerate() {
        unsafe { *addr_ptr.add(i) = *byte };
    }

    addr_ptr
}

pub struct Compiler {
    funcs: HashMap<String, Function>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { funcs: HashMap::new() }
    }

    pub fn add_func(&mut self, name: &str, args: &Vec<Type>, return_type: Type) -> Option<&mut Function> {
        let new_func = Function::new(name, args, return_type);
        if Option::is_some(&self.funcs.insert(name.to_string(), new_func)) {
            None
        } else {
            self.funcs.get_mut(name)
        }
    }

    pub fn get_func_by_name(&self, name: &str) -> Option<&Function> {
        self.funcs.get(name)
    }

    pub fn jit(&mut self) {
        let mut gen = X86_64Gen::new();
        gen.gen(&mut self.funcs);

        for func in self.funcs.values_mut() {
            func.set_jit_ptr(create_executable_function(func.code()))
        }
    }
}