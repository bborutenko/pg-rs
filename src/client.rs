use std::io;
use crate::messages::back::BackTypes;
use crate::messages::front_builder::{QueryMessageBuilder, StartupMessageBuilder};
use crate::protocol::connection::Connection;
use crate::protocol::conn_params::ConnParams;

pub struct Client {
    conn: Connection,
    conn_params: ConnParams,
}

impl Client {
    pub fn connect(conn_params: ConnParams) -> io::Result<Client> {
        let mut conn = Connection::new(&conn_params).unwrap();
        
        let mut auth_message = StartupMessageBuilder::new()
            .user(&conn_params.user())
            .database(&conn_params.database())
            .build();

        conn.send_message(&mut auth_message)?;

        Ok(Client { conn, conn_params })
    }

    pub fn connect_from_dsn(dsn: &str) -> io::Result<Client> {
        let conn_params = ConnParams::from_dsn(&dsn)
            .unwrap();
        Client::connect(conn_params)
    }

    pub fn execute(&mut self, sql: &str) -> Result<(), String> {
        let mut message = QueryMessageBuilder::new()
            .query(sql)
            .build();

        let response = self.conn.send_message(&mut message).unwrap();

        match response.msg_type() {
            BackTypes::CommandComplete => return Ok(()),
            BackTypes::ErrorResponse => return Err(
                format!("Error response during sql query; error: {}", response.data())),
            _ => return Err("Unsupported backend message type".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_from_dsn() -> io::Result<()> {
        let dsn = "postgresql://postgres:test@127.0.0.1:5432/postgres";
        Client::connect_from_dsn(&dsn)?;
        Ok(())
    }

    #[test]
    fn test_send_query() -> io::Result<()> {
        let mut client = Client::connect_from_dsn("postgresql://postgres:test@127.0.0.1:5432/postgres")?;

        client.execute("SHOW hot_standby;").unwrap();
        Ok(())
    }
}

