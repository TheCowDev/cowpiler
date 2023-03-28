struct ByteWriter {
    data: Vec<u8>,
}

impl ByteWriter {
    pub fn new() -> Self {
        ByteWriter { data: vec![] }
    }

    pub fn write_i8(&mut self, value: i8) -> usize {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn write_u8(&mut self, value: u8) {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn write_i32(&mut self, value: i32) {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn write_i64(&mut self, value: i64) {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn write_f32(&mut self, value: f32) {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn write_f64(&mut self, value: f64) {
        let result = self.data.len();
        self.data.extend_from_slice(value.to_le_bytes());
        result
    }

    pub fn rewrite_i64(&mut self, index: usize, value: i64) {
        let value_as_bytes = value.to_le_bytes();
        self.data.splice(index..index + std::mem::size_of_val(value), value_as_bytes.iter().cloned());
    }
}