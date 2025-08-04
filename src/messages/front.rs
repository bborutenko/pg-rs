#[repr(u8)]
pub enum MessageType {
    StartupMessage,
    QueryMessage,
}

pub trait FrontendMessage {
    fn message(&mut self) -> &Vec<u8>;
}

pub struct StartupMessage {
    user: String,
    database: String,
    message: Vec<u8>
}

pub struct QueryMessage {
    query: String,
    message: Vec<u8>,
}

impl StartupMessage {
    pub fn new(user: &str, database: &str) -> StartupMessage {
        let mut message: Vec<u8> = Vec::new();

        init_message(&mut message, MessageType::StartupMessage);

        StartupMessage {
            user: user.to_string(),
            database: database.to_string(),
            message: message,
        }
    }
}

impl FrontendMessage for StartupMessage {
    fn message(&mut self) -> &Vec<u8> {
        add_bytes(&mut self.message, "user");
        add_bytes(&mut self.message, &self.user);
        add_bytes(&mut self.message, "database");   
        add_bytes(&mut self.message, &self.database);
        change_len(&mut self.message);  

        &self.message   
    }
}

impl QueryMessage {
    pub fn new(query: &str) -> QueryMessage {
        let mut message: Vec<u8> = Vec::new(); 

        init_message(&mut message, MessageType::QueryMessage);

        QueryMessage { 
            query: query.to_string(),
            message: message,
        }
    }
}

fn init_message(
    mess: &mut Vec<u8>, message_type: MessageType
) -> Result<(), &str> {
    if let MessageType::StartupMessage = message_type {
        mess.extend(&0_i32.to_be_bytes());
        mess.extend(&196608_i32.to_be_bytes());
        return Ok(())
    } 

    let byte_type = match message_type {
        MessageType::QueryMessage => b'Q',
        _ => return Err("Unsupported message type.")
    };

    mess.extend(byte_type.to_be_bytes());
    mess.extend(&0_i32.to_be_bytes());
    Ok(())
}

fn add_bytes(mess: &mut Vec<u8>, value: &str) { 
    mess.extend(value.as_bytes());
    mess.extend(b"\0");
}

fn change_len(mess: &mut Vec<u8>) {
        let len = mess.len();
        mess[..4].copy_from_slice(&len.to_be_bytes());
}

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
