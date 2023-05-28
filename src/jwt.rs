use crate::config::JwtConfig;
use anyhow::Result;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Jwt {
    /// secret key
    secret: String,
    /// expiration in seconds
    expiration: usize,
}

impl Jwt {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            secret: config.secret,
            expiration: config.expiration,
        }
    }

    pub fn generate(&self, subject: String) -> Result<String> {
        let claims = Claims {
            sub: subject,
            exp: self.expiration,
        };
        self.encode(claims)
    }

    pub fn validate(&self, token: String) -> Result<String> {
        let claims = self.decode(token)?;
        Ok(claims.sub)
    }

    fn encode(&self, claims: Claims) -> Result<String> {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;
        Ok(token)
    }

    fn decode(&self, token: String) -> Result<Claims> {
        let token_data = jsonwebtoken::decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}
