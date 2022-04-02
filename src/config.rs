use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    db: Database,
    srv: Server,
}

impl Config {
    pub fn new() -> Self {
        ron::from_str(&fs::read_to_string("Config.ron").unwrap()).unwrap()
    }

    pub fn db_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db.protocol,
            self.db.user,
            self.db.password,
            self.db.host,
            self.db.port,
            self.db.database
        )
    }

    pub fn srv_url(&self) -> String {
        format!("{}:{}", self.srv.host, self.srv.port)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Database {
    protocol: String,
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Server {
    host: String,
    port: u16,
}