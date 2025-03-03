use serde::{Deserialize, Serialize};
use bson::{oid::ObjectId, DateTime};
use chrono::{Utc};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,

}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        User {
            id: None,
            username,
            password_hash,
            email,
            created_at,
            updated_at

        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Item {
    pub fn new(user_id: String, title: String, description: String) -> Self {
        let now = DateTime::from_chrono(Utc::now());
        Item {
            id: None,
            user_id,
            title,
            description,
            created_at: now,
            updated_at: now
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItemRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItemRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}
