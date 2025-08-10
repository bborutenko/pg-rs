# PG-RS

A Postgres adapter for Rust programming laguage. 

[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)  

---

## Features

- Connection for PosgreSQL without password
- Supported Backend message types:
1. AuthenticationOk
2. CommandComplete
3. ErrorResponse
- Supported Frontend message types:
1. StartupMessage
2. QueryMessage
- Executing simple queries

---

## Code example 

```Rust
use std::io;
use pg_rs::client::Client;

fn main() -> io::Result<()> {
    let dsn = "postgresql://postgres:test@127.0.0.1:5432/postgres";
    let mut client = Client::connect_from_dsn(&dsn)?;

    client.execute("SELECT 1;").unwrap();
}
```


