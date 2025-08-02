pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut data: Vec<u8> = Vec::new(); 

        data.extend(&0_i32.to_be_bytes());
        data.extend(&196608_i32.to_be_bytes());

        Buffer { data }
    }

    pub fn auth_message(&mut self, user: &str, database: &str) -> &[u8] {
        self.add_bytes("user", user);
        self.add_bytes("database", database);
        self.message()
    }

    fn message(&mut self) -> &[u8] {
        self.data.extend(b"\0");
        let len = (self.data.len() - 4) as i32;
        self.data[..4].copy_from_slice(&len.to_be_bytes());
        &self.data
    }

    fn add_bytes(&mut self, param: &str, value: &str) {
        self.data.extend(param.as_bytes());
        self.data.extend(b"\0");
        self.data.extend(value.as_bytes());
        self.data.extend(b"\0");
    }
}

//todo write tests and standart_message method
