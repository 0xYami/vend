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

impl DBConfig {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            url: env::var("DATABASE_URL")?,
            max_connections: env::var("MAX_CONNECTIONS")?.parse::<u32>()?,
        })
    }
}

pub struct JwtConfig {
    /// secret key
    pub secret: String,
    /// expiration in hours
    pub expiration: usize,
}

impl JwtConfig {
    fn try_from_env() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            secret: env::var("JWT_SECRET")?,
            expiration: env::var("JWT_EXPIRATION")?.parse::<usize>()?,
        })
    }
}

pub struct Config {
    pub addr: SocketAddr,
    pub db: DBConfig,
    pub jwt: JwtConfig,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;

        let port = env::var("PORT")?.parse::<u16>().unwrap();
        let host = env::var("HOST")?.parse::<IpAddr>().unwrap();

        Ok(Self {
            addr: SocketAddr::from((host, port)),
            db: DBConfig::try_from_env().unwrap(),
            jwt: JwtConfig::try_from_env().unwrap(),
        })
    }
}
