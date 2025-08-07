use std::net::Ipv4Addr;
use crate::client_builder::parse_ipv4;

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

    pub fn from_dsn(dsn: &str) -> Result<ConnParams, &'static str> {
        if !dsn.starts_with("postgresql://") {
            return Err("DSN must start with 'postgresql://'");
        }

        let mut params = ConnParamsBuilder::new();
        let rest = &dsn["postgresql://".len()..];

        let parts: Vec<&str> = rest.splitn(2, '?').collect();
        let auth_and_host = parts[0];

        if !auth_and_host.is_empty() {
            let user_pass_host_port_db: Vec<&str> = auth_and_host.splitn(2, '@').collect();
            let has_auth = user_pass_host_port_db.len() == 2;
            
            if has_auth && !user_pass_host_port_db[0].is_empty() {
                let user_pass_parts: Vec<&str> = user_pass_host_port_db[0].splitn(2, ':').collect();
                params.user = &user_pass_parts[0];
                if let Some(pass) = user_pass_parts.get(1) {
                    params.password = Some(pass);
                }
            }

            // Определяем часть с host:port/dbname
            let host_port_db = if has_auth {
                user_pass_host_port_db[1]
            } else {
                user_pass_host_port_db[0]
            };

            if !host_port_db.is_empty() {
                let mut remaining = host_port_db;
                
                // Извлекаем dbname если есть /
                if let Some(pos) = remaining.find('/') {
                    params.database = &remaining[pos+1..];
                    remaining = &remaining[..pos];
                }
                
                // Извлекаем порт если есть :
                if let Some(pos) = remaining.find(':') {
                    let addr = parse_ipv4(&remaining[..pos]).unwrap();

                    params.host = Ipv4Addr::new(addr.0, addr.1, addr.2, addr.3);
                    params.port = remaining[pos+1..].parse().map_err(|_| "Invalid port number")?;
                }
            }
        }

        Ok(ConnParams {
            user: params.user.to_string(), 
            password: match params.password {
                Some(pass) => Some(pass.to_string()),
                None => None,
            }, 
            host: params.host, 
            port: params.port, 
            database: params.database.to_string(), 
        })

    }

    pub fn to_dsn(&self) -> String {
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
    fn test_to_dsn_default() {
        let conn_params = ConnParamsBuilder::new()
            .build();

        assert_eq!(
            conn_params.to_dsn(),
            "postgresql://postgres@127.0.0.1:5432/postgres".to_string(),
        );
    }

    #[test]
    fn test_to_dsn_with_password() {
        let conn_params = ConnParamsBuilder::new()
            .password("test")
            .build();
    
        assert_eq!(
            conn_params.to_dsn(),
            "postgresql://postgres:test@127.0.0.1:5432/postgres".to_string(),
        );
    }

    #[test]
    fn test_from_dsn() {
        let conn_params = ConnParams::from_dsn(
            "postgresql://postgres:test@127.0.0.1:5432/postgres"
            ).unwrap();

        assert_eq!(
            conn_params.to_dsn(),
            "postgresql://postgres:test@127.0.0.1:5432/postgres".to_string()
        )
    }

    #[test]
    fn test_from_dsn_without_host_port() {
        let conn_params = ConnParams::from_dsn(
            "postgresql://postgres:test@"
        ).unwrap();

        assert_eq!(
            conn_params.to_dsn(),
            "postgresql://postgres:test@127.0.0.1:5432/postgres".to_string()
        )
    }

    #[test]
    fn test_from_dsn_without_auth() {
        let conn_params = ConnParams::from_dsn(
            "postgresql://127.0.0.1:5432/postgres"
        ).unwrap();

        assert_eq!(
            conn_params.to_dsn(),
            "postgresql://postgres@127.0.0.1:5432/postgres".to_string()
        )
    }
}
