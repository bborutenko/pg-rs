use crate::client::Client;
use crate::protocol::conn_params::ConnParamsBuilder;

pub struct ClientBuilder<'a> {
   user: &'a str,
   password: Option<&'a str>,
   host: (u8, u8, u8, u8),
   port: u32,
   database: &'a str,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> ClientBuilder<'a> {
        ClientBuilder {
            user: "postgres",
            password: None,
            host: (127, 0, 0, 1),
            port: 5432,
            database: "postgres",
        }
    }

    pub fn user(mut self, user: &'a str) -> ClientBuilder<'a> {
        self.user = user;
        self
    }

    pub fn password(mut self, password: &'a str) -> ClientBuilder<'a> {
        self.password = Some(password);
        self
    }

    pub fn host(mut self, host: &'a str) -> ClientBuilder<'a> {
        self.host = parse_ipv4(host).unwrap();
        self
    }

    pub fn port(mut self, port: u32) -> ClientBuilder<'a> {
        self.port = port;
        self
    }

    pub fn database(mut self, database: &'a str) -> ClientBuilder<'a> {
        self.database = database;
        self
    }

    pub fn connect(self) -> Client {
        if self.password.is_some() {
            let conn_params = ConnParamsBuilder::new()
                .user(self.user)
                .password(self.password.unwrap())
                .host(self.host)
                .port(self.port)
                .database(self.database)
                .build();

            Client::connect(conn_params).unwrap()
        } else {
            let conn_params = ConnParamsBuilder::new()
                .user(self.user)
                .host(self.host)
                .port(self.port)
                .database(self.database)
                .build();

            Client::connect(conn_params).unwrap()
        }
    }
}

pub fn parse_ipv4(ip_str: &str) -> Result<(u8, u8, u8, u8), &'static str> {
    if ip_str.to_lowercase() == "localhost" {
        return Ok((127, 0, 0, 1));
    }

    let parts: Vec<&str> = ip_str.split('.').collect();
    
    if parts.len() != 4 {
        return Err("IPv4 address must have exactly 4 octets");
    }

    let mut octets = [0u8; 4];
    for (i, part) in parts.iter().enumerate() {
        octets[i] = part.parse().map_err(|_| "Invalid octet value")?;
    }

    Ok((octets[0], octets[1], octets[2], octets[3]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Result;

    #[test]
    fn test_client_connection() -> Result<()> {
        ClientBuilder::new()
          .user("postgres")
          .host("localhost")
          .port(5432)
          .database("postgres")
          .connect(); 

        Ok(())
    }
}
