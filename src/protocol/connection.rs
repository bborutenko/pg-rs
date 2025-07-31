use std::net::TcpListener;


pub struct Connection<'a> {
    url: &'a String,
    connection: TcpListener,
}


impl<'a> Connection<'a> {
    pub fn connect(url: &'a str) -> Self {
        Connection { url }
    }   
}

