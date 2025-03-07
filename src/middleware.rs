use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request, State,
};
use mongodb::Database;
use std::sync::Arc;

use crate::auth::validate_token;
use crate::models::User;
use crate::db::find_user_by_id;

pub struct AuthenticatedUser(pub user);


#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the authorization header
        let token = match request.headers().get_one("Authorization") {
            Some(token) => {
                // Remove "Bearer " from the start of the token
                if token.starts_with("Bearer ") {
                    token.trim_start_matches("Bearer ").trim()
                } else {
                    return Outcome::Failure((Status::Unauthorized, ()));
                }
            }
            None => return Outcome::Failure((Status::Unauthorized, ())),
        };

        // Get the database connection
        let db = match request.rocket().state::<Database>() {
            Some(db) => db,
            None => return Outcome::Failure((Status::InternalServerError, ())),
        };

        // Validate the token
        match validate_token(token) {
            Ok(claims) => {
                // Get the user from the database
                match find_user_by_id(db, &claims.sub).await {
                    Ok(Some(user)) => Outcome::Success(AuthenticatedUser(user)),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}