use crate::lang::block::Block;
use crate::lang::lang_type::Type;
use crate::lang::value::Value;


pub(crate) enum Instr {
    ConstInt128 { const_value: i128, gen_value: Value },
    ConstInt64 { const_value: i64, gen_value: Value },
    ConstInt32 { const_value: i32, gen_value: Value },
    ConstInt16 { const_value: i16, gen_value: Value },
    ConstInt8 { const_value: i8, gen_value: Value },
    ConstPtr { const_value: usize, gen_value: Value },

    Add { left_value: Value, right_value: Value, gen_value: Value },
    Sub { left_value: Value, right_value: Value, gen_value: Value },
    Div { left_value: Value, right_value: Value, gen_value: Value },
    Mul { left_value: Value, right_value: Value, gen_value: Value },

    Eq { left_value: Value, right_value: Value, gen_value: Value },
    Diff { left_value: Value, right_value: Value, gen_value: Value },
    Larger { left_value: Value, right_value: Value, gen_value: Value },
    LargerEq { left_value: Value, right_value: Value, gen_value: Value },
    Smaller { left_value: Value, right_value: Value, gen_value: Value },
    SmallerEq { left_value: Value, right_value: Value, gen_value: Value },

    Not { value: Value, right_value: Value, gen_value: Value },

    Load { value_to_load: Value, gen_value: Value },
    Store { value_ptr: Value, value_to_store: Value },

    Br { block_to_br: Block },
    CondBr { block_to_br_true: Block, block_to_br_false: Block, value_cond: Value },

    CallPtr { ptr_to_call: Value, args: Vec<Value>, return_type: Type, gen_value: Value },
    CallFunc { func_to_call: String, args: Vec<Value>, gen_value: Value },

    Ret { value_to_return: Value },
    RetVoid,
}
