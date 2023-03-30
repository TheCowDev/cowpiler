use crate::lang::lang_type::Type;

#[derive(Clone)]
pub struct Value {
    id: usize,
    value_type: Type,
}

impl Value {
    pub fn new(id: usize, lang_type: Type) -> Self {
        Value { id, value_type: lang_type }
    }

    pub fn get_type(&self) -> Type {
        self.value_type.clone()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

