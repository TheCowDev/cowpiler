use crate::compiler::Compiler;

mod lang;

use lang::lang_type::LangType;

pub mod compiler;

fn main() {
    let mut compiler = Compiler::new();
    let mut my_func = compiler.add_func("my_func", &vec![], LangType::i32()).unwrap();

    let builder = my_func.builder();

    let other = builder.create_block();

    let first_value = builder.const_i8(10);

    let second_value = builder.const_i32(10);


    compiler.jit();
}
