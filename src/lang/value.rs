pub struct Value {
    id: usize,
    value_type: LangType,
}

impl Value {
    pub fn get_type(&self) -> LangType {
        self.value_type
    }
}

