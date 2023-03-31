use std::ptr;
use crate::lang;
use lang::lang_type::Type;
use crate::lang::builder::Builder;

pub struct Function {
    name: String,
    args: Vec<Type>,
    return_type: Type,
    builder: Builder,
    code: Vec<u8>,
    jit_ptr: *mut u8,
}


impl Function {
    pub fn new(name: &str, args: &Vec<Type>, return_type: Type) -> Self {
        Function {
            name: String::from(name),
            args: args.clone(),
            return_type,
            builder: Builder::new(),
            code: vec![],
            jit_ptr: ptr::null_mut(),
        }
    }

    pub fn args(&self) -> &Vec<Type> {
        &self.args
    }

    pub fn builder(&mut self) -> &mut Builder {
        &mut self.builder
    }

    pub fn set_code(&mut self, code: &Vec<u8>) {
        self.code = code.clone();
    }

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn name(&self) -> &String{
        &self.name
    }

    pub fn set_jit_ptr(&mut self, ptr: *mut u8) {
        self.jit_ptr = ptr;
    }

    pub fn jit_ptr(&self) -> *mut u8 {
        self.jit_ptr
    }
}