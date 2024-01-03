// The `profile` module.
pub mod model;
pub mod service;


// Importing the necessary modules and functions.
use axum::routing::{delete, get, post};
use crate::{config::BeContext, service::profile::service::ProfilesService};


// The `ProfilesRouter` struct which is responsible for routing HTTP requests to the appropriate handlers.
pub struct ProfilesRouter;


// Implementation of the `ProfilesRouter` struct.
impl ProfilesRouter {
    // Function to create a new `ProfilesRouter`.
    pub fn new() -> axum::Router<BeContext> {
        // Create a new `Router` and define the routes.
        axum::Router::new()
            // Route for fetching a specific profile.
            .route("/profiles/:username", get(ProfilesService::fetch_profile))
            // Route for following a specific profile.
            .route("/profiles/:username/follow", post(ProfilesService::follow_profile))
            // Route for unfollowing a specific profile.
            .route("/profiles/:username/follow", delete(ProfilesService::unfollow_profile))
            // Route for blocking a specific profile.
            .route("/profiles/:username/block", post(ProfilesService::block_profile))
            // Route for unblocking a specific profile.
            .route("/profiles/:username/block", delete(ProfilesService::unblock_profile))
    }
}