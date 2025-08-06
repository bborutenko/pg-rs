use std::{io::Result, net::TcpStream};
use std::io::Write;
use crate::messages::back::BackTypes;
use crate::messages::front::FrontendMessage;
use crate::protocol::conn_params::ConnParams;
use crate::messages::back::BackMessage;
use crate::messages::front_builder::StartupMessageBuilder;

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

    pub fn auth(&mut self, conn_params: &ConnParams) -> Result<BackMessage> {
        let mut message = StartupMessageBuilder::new()
            .user(&conn_params.user())
            .database(&conn_params.database())
            .build();

        self.stream.write_all(message.to_vecu8())?;
        Ok(BackMessage::from_buf(&mut self.stream)?)
    }
}

