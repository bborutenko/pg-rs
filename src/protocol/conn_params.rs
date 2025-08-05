use std::net::Ipv4Addr;

pub struct ConnParams {
    user: String,
    password: Option<String>,
    host: Ipv4Addr,
    port: u32,
    database: String,
}

pub struct ConnParamsBuilder<'a> {
    user: &'a str,
    password: Option<&'a str>,
    host: Ipv4Addr,
    port: u32,
    database: &'a str,

}

impl ConnParams {
    #[inline]
    pub fn user(&self) -> &String {
        &self.user
    }

    #[inline]
    pub fn password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    #[inline]
    pub fn host(&self) -> &Ipv4Addr {
        &self.host
    }

    #[inline]
    pub fn port(&self) -> &u32 {
        &self.port
    }

    #[inline]
    pub fn database(&self) -> &String {
        &self.database
    }

    pub fn to_string(&self) -> String {
        let mut url = format!("postgresql://{}", self.user);

        url = match &self.password {
            Some(pass) => format!("{}:{}", url, pass),
            None => url,
        };

        url = format!(
            "{}@{}:{}/{}",
            url,
            self.host.to_string(),
            self.port.to_string(),
            self.database,
        );

        url
    }
}


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
        &'a mut self, password: &'a str,
    ) -> &'a mut ConnParamsBuilder<'a> {
        self.password = Some(password);
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

    pub fn build(&'a self) -> ConnParams {
        ConnParams {
            user: self.user.to_string(), 
            password: match self.password {
                Some(pass) => Some(pass.to_string()),
                None => None,
            }, 
            host: self.host, 
            port: self.port, 
            database: self.database.to_string(), 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let host: (u8, u8, u8, u8) = (0, 0, 0, 0); 
        
        let conn_params = ConnParamsBuilder::new()
            .user("test")
            .password("test")
            .host(host)
            .port(8000)
            .database("test")
            .build();

        let conn_params_default = ConnParamsBuilder::new()
            .build();

        assert_ne!(conn_params.user(), conn_params_default.user());
        assert_ne!(conn_params.password(), conn_params_default.password());
        assert_ne!(conn_params.host(), conn_params_default.host());
        assert_ne!(conn_params.port(), conn_params_default.port());
        assert_ne!(conn_params.database(), conn_params_default.database());
    }

    #[test]
    fn test_to_string_default() {
        let conn_params = ConnParamsBuilder::new()
            .build();

        assert_eq!(
            conn_params.to_string(),
            "postgresql://postgres@127.0.0.1:5432/postgres".to_string(),
        );
    }

    #[test]
    fn test_to_string_with_password() {
        let conn_params = ConnParamsBuilder::new()
            .password("test")
            .build();
    
        assert_eq!(
            conn_params.to_string(),
            "postgresql://postgres:test@127.0.0.1:5432/postgres".to_string(),
        );
    }
}
