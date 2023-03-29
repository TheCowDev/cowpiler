use crate::lang::lang_type::LangType;
use crate::lang::value::Value;


pub(crate) enum Instr {
    ConstInt128 { const_value: i128 },
    ConstInt64 { const_value: i64 },
    ConstInt32 { const_value: i32 },
    ConstInt16 { const_value: i16 },
    ConstInt8 { const_value: i8 },
    ConstPtr { const_value: usize },

    Add { left_value: Value, right_value: Value },
    Sub { left_value: Value, right_value: Value },
    Div { left_value: Value, right_value: Value },
    Mul { left_value: Value, right_value: Value },

    Eq { left_value: Value, right_value: Value },
    Diff { left_value: Value, right_value: Value },
    Larger { left_value: Value, right_value: Value },
    LargerEq { left_value: Value, right_value: Value },
    Smaller { left_value: Value, right_value: Value },
    SmallerEq { left_value: Value, right_value: Value },

    Not { value: Value, right_value: Value },

    Load { value_to_load: Value },
    Store { value_ptr: Value, value_to_store: Value },

    Br { block_to_br: usize },
    CondBr { block_to_br_true: usize, block_to_br_false: usize, value_cond: Value },

    CallPtr { ptr_to_call: Value, args: Vec<Value>, return_type: LangType },
    CallFunc { func_to_call: String, args: Vec<Value> },

    Ret { value_to_return: Value },
    RetVoid,
}
