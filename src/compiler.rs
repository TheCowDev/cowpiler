use std::collections::HashMap;
use std::iter::Map;
use crate::lang;
use lang::function::Function;
use lang::lang_type::LangType;

pub struct Compiler {
    funcs: HashMap<String, Function>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler { funcs: HashMap::new() }
    }

    pub fn add_func(&mut self, name: &str, args: &Vec<LangType>, return_type: LangType) -> Option<Function> {
        let new_func = Function::new(name, args, return_type);
        self.funcs.insert(name.to_string(), new_func)
    }

    pub fn jit(&mut self) {

    }
}