use std::io::Result;
use crate::messages::front_builder::StartupMessageBuilder;
use crate::protocol::connection::Connection;
use crate::protocol::conn_params::ConnParams;

pub struct Client {
    conn: Connection,
    conn_params: ConnParams,
}

impl Client {
    pub fn connect(conn_params: ConnParams) -> Result<Client> {
        let mut conn = Connection::new(&conn_params).unwrap();
        
        let mut auth_message = StartupMessageBuilder::new()
            .user(&conn_params.user())
            .database(&conn_params.database())
            .build();

        conn.send_message(&mut auth_message)?;

        Ok(Client { conn, conn_params })
    }

    pub fn connect_from_dsn(dsn: &str) -> Client {
        let conn_params = ConnParams::from_dsn(&dsn)
            .unwrap();
        let conn = Connection::new(&conn_params).unwrap();

        Client { conn, conn_params }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_from_dsn() {
        let dsn = "postgresql://postgres:test@127.0.0.1:5432/postgres";
        Client::connect_from_dsn(&dsn);
    }
}

