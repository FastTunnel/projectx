use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, exp: usize) -> Self {
        Self { sub, exp }
    }
    pub fn is_expired(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize
    }
    pub fn can_refresh(&self) -> bool {
        self.exp < chrono::Utc::now().timestamp() as usize + 60 * 60
    }
    pub fn current_user(&self) -> &String {
        &self.sub
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "subject:{}", self.sub)
    }
}
