use crate::lang;
use lang::lang_type::LangType;
use crate::lang::builder::Builder;

pub struct Function {
    name: String,
    args: Vec<LangType>,
    return_type: LangType,
    builder: Builder,
}


impl Function {
    pub fn new(name: &str, args: &Vec<LangType>, return_type: LangType) -> Self {
        Function { name: String::from(name), args: args.clone(), return_type, builder: Builder::new() }
    }

    pub fn args(&self) -> &Vec<LangType> {
        &self.args
    }

    pub fn builder(&mut self) -> &mut Builder {
        &mut self.builder
    }
}