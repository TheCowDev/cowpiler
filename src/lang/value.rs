use crate::lang::lang_type::LangType;

#[derive(Clone)]
pub struct Value {
    id: usize,
    value_type: LangType,
}

impl Value {
    pub fn new(id: usize, lang_type: LangType) -> Self {
        Value { id, value_type: lang_type }
    }

    pub fn get_type(&self) -> LangType {
        self.value_type.clone()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

