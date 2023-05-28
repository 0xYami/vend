use anyhow::Result;
use dotenvy::dotenv;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};

pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
}

pub struct Config {
    pub addr: SocketAddr,
    pub db: DBConfig,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;

        let port = env::var("PORT")?.parse::<u16>().unwrap();
        let host = env::var("HOST")?.parse::<IpAddr>().unwrap();
        let database_url = env::var("DATABASE_URL")?;
        let max_connections = env::var("MAX_CONNECTIONS")?.parse::<u32>().unwrap();

        Ok(Self {
            addr: SocketAddr::from((host, port)),
            db: DBConfig {
                url: database_url,
                max_connections,
            },
        })
    }
}
