#[repr(u8)]
pub enum MessageType {
    pub enum FrontendMessage {
    Password = b'p',
    Query = b'Q',
    Parse = b'P',
    Bind = b'B',
    Execute = b'E',
    Close = b'C',
    Describe = b'D',
    Flush = b'H',
    Sync = b'S',
    Terminate = b'X',}

pub struct StartupMessage {
    user: String,
    database: String,
}

pub struct StartupMessageBuilder<'a> {
    user: &'a str,
    database: &'a str,
}

pub struct QueryMessage {
    q_type: String,
    payload: String,
}

pub struct QueryMessageBuilder<'a> {
    q_type: &'a str,
    payload: &'a str,
}

impl<'a> StartupMessageBuilder<'a> {
    pub fn new() -> StartupMessageBuilder<'a> {
        StartupMessageBuilder { 
            user: "postgres", 
            database: "postgres",
        }
    }

    pub fn user(mut self, user: &'a str) -> StartupMessageBuilder<'a> {
        self.user = user;
        self
    }

    pub fn database(mut self, database: &'a str) -> StartupMessageBuilder<'a> {
        self.database = database;
        self
    }

    pub fn build(self) -> StartupMessage {
        StartupMessage { 
            user: self.user.to_string(), 
            database: self.database.to_string(), 
        }
    }
}

impl<'a> QueryMessageBuilder<'a> {
    pub fn new() -> QueryMessageBuilder<'a> {
        QueryMessageBuilder { 
            q_type: "Q", 
            payload:  ""
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut buf = Buffer::new();
        let test_message = buf.message();
        let mut expected_message = Vec::new();
        expected_message.extend(
            &9_i32.to_be_bytes()
        );
        expected_message.extend(
            &196608_i32.to_be_bytes()
        );
        expected_message.extend(b"\0");
        
        assert_eq!(
            test_message,
            expected_message
        );
    }

    #[test]
    fn test_auth_message() {
        let mut buf = Buffer::new();

        let test_message = buf.auth_message("postgres", "postgres");
        
        let mut expected_message = Vec::new();
        expected_message.extend(
            &9_i32.to_be_bytes()
        );
        expected_message.extend(
            &196608_i32.to_be_bytes()
        );
        
        expected_message.extend(b"user\0postgres\0database\0postgres\0\0");

        assert_eq!(
            test_message,
            expected_message
        );
    }
}
