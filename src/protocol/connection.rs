use std::{io::Result, net::TcpStream};
use std::io::Write;
use crate::protocol::conn_params::ConnParams;
use crate::protocol::buffer::Buffer;

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

    pub fn auth(&mut self, conn_params: &ConnParams) -> Result<()>{
        let mut buf = Buffer::new();
        let message = buf.auth_message(
            conn_params.user(), 
            conn_params.database()
        );

        self.stream.write_all(&message)?;
        Ok(())
    }
}

//todo write tests
 
