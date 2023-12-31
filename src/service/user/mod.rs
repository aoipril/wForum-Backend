// The `user` module.
pub mod router;
pub mod service;


// Importing the necessary modules and functions.
use serde::{Deserialize, Serialize};
use prisma_client_rust::chrono::{FixedOffset, TimeZone};

use crate::config::CONTEXT;
use crate::prisma::prisma::user_details;


// The `UserBody` struct which represents the body of a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    // The user in the body.
    pub user: T
}

// The `CreateUserPost` struct which represents the data for creating a user.
#[derive(Debug, Deserialize)]
pub struct CreateUserPost {
    // The email of the user.
    pub email: String,
    // The username of the user.
    pub username: String,
    // The password of the user.
    pub password: String,
}

// The `UpdateUserPost` struct which represents the data for updating a user.
#[derive(Debug, Deserialize)]
pub struct UpdateUserPost {
    // The new email of the user.
    pub email: Option<String>,
    // The new introduction of the user.
    pub intro: Option<String>,
    // The new avatar of the user.
    pub avatar: Option<String>,
    // The new username of the user.
    pub username: Option<String>,
    // The new password of the user.
    pub password: Option<String>,
}

// The `LoginUserPost` struct which represents the data for logging in a user.
#[derive(Debug, Deserialize)]
pub struct LoginUserPost {
    // The email of the user.
    pub email: String,
    // The password of the user.
    pub password: String,
}

// The `User` struct which represents a user.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // The ID of the user.
    pub user_id: i32,
    // The introduction of the user.
    pub intro: Option<String>,
    // The avatar of the user.
    pub avatar: Option<String>,
    // The email of the user.
    pub email: String,
    // The username of the user.
    pub username: String,
    // The creation timestamp of the user.
    pub created_at: prisma_client_rust::chrono::DateTime<FixedOffset>,
    // The token of the user.
    pub token: Option<String>,
}


// Implementation of the `User` struct.
impl User {
    // Function to set the token of the user.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}


// Implementation of the `From` trait for `User`.
impl From<user_details::Data> for User {
    // Function to convert `user_details::Data` into a `User`.
    fn from(data: user_details::Data) -> Self {
        Self {
            user_id: data.user_id,
            intro: data.intro,
            avatar: data.avatar,
            email: data.email,
            username: data.username,
            // Convert the creation timestamp to the configured timezone.
            created_at: FixedOffset::east_opt(3600 * CONTEXT.config.tz_east_offset_in_hours)
                .unwrap().from_utc_datetime(&data.created_at.naive_utc()),
            token: None,
        }
    }
}