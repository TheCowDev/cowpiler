extern crate core;

use crate::compiler::Compiler;

mod lang;
mod gen;
mod misc;

use lang::lang_type::Type;

pub mod compiler;

fn main() {
    let mut compiler = Compiler::new();
    let mut my_func = compiler.add_func("my_func", &vec![], Type::i32()).unwrap();

    {
        let builder = my_func.builder();

        let block = builder.create_block();

        builder.br(block);

        builder.set_current_block(block);
        let first = builder.const_i64(10);
        builder.ret(first);

        compiler.jit();
    }

    let func_ptr: *mut u8 = compiler.get_func_by_name("my_func").unwrap().jit_ptr();

    // Transmute the pointer to a function type
    let func: unsafe extern "C" fn() -> i64 = unsafe { std::mem::transmute(func_ptr) };

    // Call the function
    let result = unsafe { func() };

    println!("Function returned: {}", result);
}
