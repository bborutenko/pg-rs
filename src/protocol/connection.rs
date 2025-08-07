use std::{io::Result, net::TcpStream};
use std::io::Write;
use crate::messages::front::FrontendMessage;
use crate::protocol::conn_params::ConnParams;
use crate::messages::back::BackMessage;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(conn_params: &ConnParams) -> Result<Connection> {
        let addr = format!(
            "{}:{}",
            conn_params.host().to_string(), 
            conn_params.port().to_string(),
        );

        Ok(Connection {
            stream: match TcpStream::connect(addr) {
                Ok(strm) => strm,
                Err(err) => return Err(err),
            } 
        })
     }

    pub fn send_message<T: FrontendMessage>(
        &mut self,  
        message: &mut T
    ) -> Result<BackMessage> {
        self.stream.write_all(message.to_vecu8())?;
        Ok(BackMessage::from_buf(&mut self.stream)?)
    }
}

