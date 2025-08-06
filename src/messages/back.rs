use std::net::TcpStream;
use std::io::{Result, Read};

pub enum BackTypes {
    AuthenticationOk,
    ErrorResponse,
}

pub struct BackMessage {
    msg_type: BackTypes,
    data: Vec<u8>,
}

impl BackMessage {
    pub fn from_buf(stream: &mut TcpStream) -> Result<BackMessage> {
        let mut header = [0; 5];

        stream.read_exact(&mut header)?; 

        let msg_type = match header[0] {
            b'R' => BackTypes::AuthenticationOk,
            _ => BackTypes::ErrorResponse,
        };
        let msg_len = u32::from_be_bytes(
            [header[1], header[2], header[3], header[4]]
        ) as usize;

        let mut body = vec![0; msg_len - 4];
        stream.read_exact(&mut body)?;

        Ok(BackMessage {
            msg_type: msg_type, 
            data: body,
        })
    }

    #[inline]
    pub fn msg_type(&self) -> &BackTypes {
        &self.msg_type
    }

    #[inline]
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
