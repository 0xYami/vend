use anyhow::Result;
use axum::http::{HeaderName, HeaderValue, Method};
use dotenvy::dotenv;
use std::{
    env,
    net::{IpAddr, SocketAddr},
};

#[derive(Clone)]
pub struct DBConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DBConfig {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            url: env::var("DATABASE_URL")?,
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")?.parse::<u32>()?,
        })
    }
}

#[derive(Clone)]
pub struct CorsConfig {
    pub allowed_origin: HeaderValue,
    pub allowed_methods: Vec<Method>,
    pub allowed_headers: Vec<HeaderName>,
}

impl CorsConfig {
    fn try_from_env() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            allowed_origin: env::var("CORS_ALLOWED_ORIGIN")?.parse::<HeaderValue>()?,
            allowed_methods: env::var("CORS_ALLOWED_METHODS")?
                .split(',')
                .map(|m| m.parse::<Method>().unwrap())
                .collect(),
            allowed_headers: env::var("CORS_ALLOWED_HEADERS")?
                .split(',')
                .map(|h| h.parse::<HeaderName>().unwrap())
                .collect(),
        })
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct S3BucketConfig {
    pub base_url: String,
}

impl S3BucketConfig {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;
        Ok(Self {
            base_url: env::var("S3_BUCKET_BASE_URL").unwrap(),
        })
    }
}

#[derive(Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub db: DBConfig,
    pub cors: CorsConfig,
    pub jwt: JwtConfig,
    pub s3_bucket: S3BucketConfig,
}

impl Config {
    pub fn try_from_env() -> Result<Self> {
        dotenv()?;

        let port = env::var("PORT")?.parse::<u16>().unwrap();
        let host = env::var("HOST")?.parse::<IpAddr>().unwrap();

        Ok(Self {
            addr: SocketAddr::from((host, port)),
            db: DBConfig::try_from_env().unwrap(),
            cors: CorsConfig::try_from_env().unwrap(),
            jwt: JwtConfig::try_from_env().unwrap(),
            s3_bucket: S3BucketConfig::try_from_env().unwrap(),
        })
    }
}
