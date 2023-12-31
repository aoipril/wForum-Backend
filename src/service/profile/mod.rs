// The `profile` module.
pub mod router;
pub mod service;


// Importing the necessary modules and functions.
use serde::{Serialize, Deserialize};
use crate::prisma::prisma::user_details;


// The `ProfileBody` struct which represents the body of a profile.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileBody<T> {
    // The profile in the body.
    pub profile: T
}

// The `Profile` struct which represents a profile.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    // The username of the profile.
    pub username: String,
    // The introduction of the profile.
    pub intro: Option<String>,
    // The avatar of the profile.
    pub avatar: Option<String>,
    // Whether the profile is followed.
    pub followed: bool,
    // Whether the profile is following.
    pub following: bool,
    // Whether the profile is blocked.
    pub blocked: bool,
    // Whether the profile is blocking.
    pub blocking: bool,
}


// Implementation of the `user_details::Data` struct.
impl user_details::Data {
    // Function to convert `user_details::Data` into a `Profile`.
    pub fn to_profile(self, followed: bool, following: bool, blocked:bool, blocking:bool,) -> Profile {
        Profile {
            username: self.username,
            intro: self.intro,
            avatar: self.avatar,
            following, followed,
            blocking, blocked
        }
    }
}


// Implementation of the `From` trait for `Profile`.
impl From<user_details::Data> for Profile {
    // Function to convert `user_details::Data` into a `Profile`.
    fn from(data: user_details::Data) -> Self {
        Self {
            username: data.username,
            intro: data.intro,
            avatar: data.avatar,
            following: false, followed: false,
            blocking: false, blocked: false,
        }
    }
}