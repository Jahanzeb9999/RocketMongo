use std::result;

use futures::TryStreamExt;
use mongodb::{
    Collection,
    Database,
    bson::{doc, Document, oid::ObjectId},
    error::Error as MongoError,
    options::FindOneOptions,
};
use bson::to_document;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::models::{User, Item};

#[derive(Debug, Error)]
pub enum DbError {
    #[error("MongoDb error: {0}")]
    MongoError(#[from] MongoError),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Document not found")]
    NotFound,
    #[error("Invalid format")]
    InvalidId,
}

// Helper functions to work with MongoDB
pub async fn find_user_by_username(db: &Database, username: &str) -> Result<Option<user>, DbError> {
    let collection = db.collection::<User>("users");
    let filter = doc! {"username": username};
    let result = collection.find_one(filter, None).await?;
    Ok(result)
}


pub async fn insert_user(db: &Database, user: &User) -> Result<String, DbError> {
    let collection = db.collection::<User>("users");
    let result = collection.insert_one(user.clone(), None).await?;

    match result.inserted_id.as_object_id() {
        Some(id) => Ok(id.to_hex()),
        None => Err(DbError::SerializationError("Failed to get inserted ID".to_string()))
    }
}

pub async fn find_item_by_id(db: &Database, item_id: &str) -> Result<Option<Item>, DbError> {
    let collection = db.collection::<Item>("items");
    let object_id = ObjectId::parse_str(item_id).map_err(|_| DbError::InvalidId)?;
    let filter = doc!{"_id": object_id};
    let result = collection.find_one(filter, None).await?;

    Ok(result)
}

pub async fn find_items_by_users_id(db: &Database, user_id: &str) -> Result<Vec<Item>, DbError> {
    let collection = db.collection::<Item>("items");
    let filter = doc! {"user_id": user_id};
    let cursor = collection.find(filter, None).await?;
    let items = cursor.try_collect().await?;

    Ok(result)


}

pub async fn update_item(db: &Database, item_id: &str, updates: Document) -> Result<bool, DbError> {
    let collection = db.collection::<Item>(name);
    let object_id = ObjectId::parse_str(item_id).map_err(|_| DbError::InvalidId)?;
    let filter = doc!{"_id": object_id};
    let update = doc! {"$set": updates};

    let result = collection.update_one(filter, update, None).await?;
    Ok(result.modified_count > 0);

}

pub async fn delete_item(db: &Database, item_id: &str) -> Result<bool, DbError> {
    let collection = db.collection::<Item>("items");
    let object_id = ObjectId::parse_str(item_id).map_err(|_| DbError::InvalidId);
    let filter = doc!{"_id": object_id};

    let result = collection.delete_one(filter, None).await?;
    Ok(result.deleted_count > 0)
}
