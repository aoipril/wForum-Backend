// Importing the necessary modules and functions.
use axum::routing::{get, post, put};
use crate::service::user::service::UsersService;


// The `UsersRouter` struct which is responsible for routing HTTP requests to the appropriate handlers.
pub struct UsersRouter;


// Implementation of the `UsersRouter` struct.
impl UsersRouter {
    // Function to create a new `UsersRouter`.
    pub fn new() -> axum::Router<crate::config::BeContext> {
        // Create a new `Router` and define the routes.
        axum::Router::new()
            // Route for fetching the current user's details.
            .route("/users", get(UsersService::fetch_user))
            // Route for logging in a user.
            .route("/users", post(UsersService::login_user))
            // Route for updating the current user's details.
            .route("/users", put(UsersService::update_user))
            // Route for creating a new user.
            .route("/users/create", post(UsersService::create_user))
    }
}