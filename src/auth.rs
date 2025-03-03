use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use rocket::time::Duration;
use core::hash;
use std::env;

use crate::models::{TokenClaims, User};


static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").unwrap_or_else(|_| "your_default_jwt_secret_key".to_string())
});

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}


pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}



pub fn generate_token(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> { 
    let now = Utc::now();
    let expires_at = now + Duration::hours(24);

    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: expires_at.timestamp(),
    };

    encode(
        &Header,
        &claims, 
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

pub fn validate_token(token: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    let decoded = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default()
    );

    Ok(decoded);
}
