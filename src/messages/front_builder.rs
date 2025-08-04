use super::front::{
    StartupMessage,
    QueryMessage,
};


pub struct StartupMessageBuilder<'a> {
    user: &'a str,
    database: &'a str,
}

pub struct QueryMessageBuilder<'a> {
    query: &'a str,
}

impl<'a> StartupMessageBuilder<'a> {
    pub fn new() -> StartupMessageBuilder<'a> {
        StartupMessageBuilder { 
            user: "postgres", 
            database: "postgres",
        }
    }

    pub fn user(mut self, user: &'a str) -> StartupMessageBuilder<'a> {
        self.user = user;
        self
    }

    pub fn database(mut self, database: &'a str) -> StartupMessageBuilder<'a> {
        self.database = database;
        self
    }

    pub fn build(self) -> StartupMessage {
        StartupMessage::new(self.user, self.database)
    }
}

impl<'a> QueryMessageBuilder<'a> {
    pub fn new() -> QueryMessageBuilder<'a> {
        QueryMessageBuilder { 
            query:  "SELECT 1;",
        }
    }

    pub fn query(
        mut self, query: &'a str
    ) -> QueryMessageBuilder<'a> {
        self.query = query;
        self
    }

    pub fn build(self) -> QueryMessage {
        QueryMessage::new(self.query)
    }
}
