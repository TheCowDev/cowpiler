#[derive(PartialEq, Clone)]
pub enum LangDataType {
    DataTypeVoid,
    DataTypeI64,
    DataTypeI32,
    DataTypeI16,
    DataTypeI8,
    DataTypeF64,
    DataTypeF32,
    DataTypePtr,
}


#[derive(Clone)]
pub struct Type {
    data_type: LangDataType,
}

impl Type {
    pub fn void() -> Self { Self { data_type: LangDataType::DataTypeVoid } }

    pub fn i8() -> Self {
        Self { data_type: LangDataType::DataTypeI8 }
    }

    pub fn i16() -> Self {
        Self { data_type: LangDataType::DataTypeI16 }
    }

    pub fn i32() -> Self {
        Self { data_type: LangDataType::DataTypeI32 }
    }

    pub fn i64() -> Self {
        Self { data_type: LangDataType::DataTypeI64 }
    }

    pub fn f32() -> Self {
        Self { data_type: LangDataType::DataTypeF32 }
    }

    pub fn f64() -> Self {
        Self { data_type: LangDataType::DataTypeF64 }
    }

    pub fn ptr() -> Self {
        Self { data_type: LangDataType::DataTypePtr }
    }

    pub fn is_float(&self) -> bool {
        self.data_type == LangDataType::DataTypeF64 || self.data_type == LangDataType::DataTypeF32
    }

    pub fn is_int(&self) -> bool {
        !(self.is_float() && self.is_ptr())
    }

    pub fn is_ptr(&self) -> bool {
        self.data_type == LangDataType::DataTypePtr
    }
}