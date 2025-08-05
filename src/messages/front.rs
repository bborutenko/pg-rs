#[repr(u8)]
pub enum MessageType {
    QueryMessage,
}

pub trait FrontendMessage {
    fn to_vecu8(&mut self) -> &Vec<u8>;
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
    pub(in crate::messages) fn new(
        user: &str, database: &str
    ) -> StartupMessage {
        let mut message: Vec<u8> = Vec::new();

        message.extend(&0_i32.to_be_bytes());
        message.extend(&196608_i32.to_be_bytes());
        
        StartupMessage {
            user: user.to_string(),
            database: database.to_string(),
            message: message,
        }
    }
}

impl FrontendMessage for StartupMessage {
    fn to_vecu8(&mut self) -> &Vec<u8> {
        add_bytes(&mut self.message, "user");
        add_bytes(&mut self.message, "\0");
        add_bytes(&mut self.message, &self.user);
        add_bytes(&mut self.message, "\0");
        add_bytes(&mut self.message, "database");
        add_bytes(&mut self.message, "\0");   
        add_bytes(&mut self.message, &self.database);
        self.message.extend(b"\0\0");
          
        let len = self.message.len() as i32;
        self.message[..4].copy_from_slice(&len.to_be_bytes());

        &self.message   
    }
}

impl QueryMessage {
    pub(in crate::messages) fn new(query: &str) -> QueryMessage {
        let mut message: Vec<u8> = Vec::new(); 

        init_message(&mut message, MessageType::QueryMessage);

        QueryMessage { 
            query: query.to_string(),
            message: message,
        }
    }
}

impl FrontendMessage for QueryMessage {
    fn to_vecu8(&mut self) -> &Vec<u8> {
        add_bytes(&mut self.message, &self.query);
        self.message.extend(b"\0");

        change_len(&mut self.message);
        &self.message
    }
}

fn init_message(
    mess: &mut Vec<u8>, message_type: MessageType
) {

    mess.push(match message_type {
        MessageType::QueryMessage => b'Q',
    });
    mess.extend(&0_i32.to_be_bytes());
}

fn add_bytes(mess: &mut Vec<u8>, value: &str) { 
    mess.extend(value.as_bytes());
}

fn change_len(mess: &mut Vec<u8>) {
    let len = (mess.len() - 1) as i32;
    mess[1..5].copy_from_slice(&len.to_be_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_to_vecu8() {
        let mut test_message = QueryMessage::new("SELECT 1;");

        let mut expected_message: Vec<u8> = Vec::new();
        
        expected_message.push(b'Q');

        expected_message.extend(
            &14_i32.to_be_bytes()
        );
        expected_message.extend(b"SELECT 1;\0"); 

        assert_eq!(
            *test_message.to_vecu8(),
            expected_message
        );
    }

    #[test]
    fn test_startup_to_vecu8() {
        let mut test_message = StartupMessage::new("postgres", "postgres");

        let mut expected_message: Vec<u8> = Vec::new();

        expected_message.extend(
            &41_i32.to_be_bytes()
        );
        expected_message.extend(
            &196608_i32.to_be_bytes()
        );

        expected_message.extend(b"user\0postgres\0database\0postgres\0\0");

        assert_eq!(
            *test_message.to_vecu8(),
            expected_message
        );
    }

    #[test]
    fn test_init_message() {
        let mut test_message: Vec<u8> = Vec::new();
        let mut expected_message: Vec<u8> = Vec::new();

        init_message(&mut test_message, MessageType::QueryMessage);

        expected_message.push(b'Q');
        expected_message.extend(&0_i32.to_be_bytes());

        assert_eq!(
            test_message,
            expected_message
        )
    }

    #[test]
    fn test_add_bytes() {
        let mut test_message: Vec<u8> = Vec::new();
        let mut expected_message: Vec<u8> = Vec::new();

        add_bytes(&mut test_message, "test");

        expected_message.extend(b"test");

        assert_eq!(
            test_message,
            expected_message
        );
    }
}
