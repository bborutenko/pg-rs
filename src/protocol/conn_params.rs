use std::net::Ipv4Addr;


pub struct ConnParams<'a> {
    user: &'a str,
    password: Option<&'a str>,
    host: Ipv4Addr,
    port: u32,
    database: &'a str,
}

pub struct ConnParamsBuilder<'a> {
    user: &'a str,
    password: Option<&'a str>,
    host: Ipv4Addr,
    port: u32,
    database: &'a str,

}

// todo: impl for conn and unit test

impl<'a> ConnParamsBuilder<'a> {
    pub fn new() -> ConnParamsBuilder<'a> {
        ConnParamsBuilder { 
            user: "postgres", 
            password: None, 
            host: Ipv4Addr::new(127, 0, 0, 1), // localhost 
            port: 5432, 
            database: "postgres", 
        }
    }

    pub fn user(
        &'a mut self, username: &'a str,
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.user = username;
        self
    }

    pub fn password(
        &'a mut self, password: Option<&'a str>,
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.password = password;
        self
    }

    pub fn host(
        &'a mut self, host_nums: (u8, u8, u8, u8),
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.host = Ipv4Addr::new(host_nums.0, host_nums.1, host_nums.2, host_nums.3);
        self
    }

    pub fn port(
        &'a mut self, port: u32
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.port = port;
        self
    }

    pub fn database(
        &'a mut self, database: &'a str
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.database = database;
        self
    }

    pub fn build(&'a self) -> ConnParams<'a> {
        ConnParams{
            user: self.user, 
            password: self.password, 
            host: self.host, 
            port: self.port, 
            database: self.database, 

        }
    }
}


#[cfg(test)]
mod conn_params_tests {
    use super::*;

    #[test]
    fn test_new() {
        let user = "test";
        let password = "test";
        let host: (u8, u8, u8, u8) = (0, 0, 0, 0); 
        let port = 8000;
        let database = "test";
        
        let conn_params = ConnParamsBuilder::new()
            .user(user)
            .password(Some(password))
            .host(host)
            .port(port)
            .database(database)
            .build();

        let conn_params_default = ConnParamsBuilder::new()
            .build();

        assert_ne(conn_params.)
    }
}
