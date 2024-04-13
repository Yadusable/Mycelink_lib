use std::net::SocketAddr;
use std::path::PathBuf;

pub struct Config {
    pub fcp_endpoint: SocketAddr,
    pub database_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fcp_endpoint: "127.0.0.1:9481".parse().unwrap(),
            database_path: "./mycelink.sqlite3".into(),
        }
    }
}
