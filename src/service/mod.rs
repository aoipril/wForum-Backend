// Importing the necessary modules and services.
pub mod post;
pub mod profile;
pub mod user;
pub mod utils;

// The `Router` struct.
// This struct is responsible for routing HTTP requests to the appropriate handlers.
pub struct Router;

// Implementation of the `Router` struct.
impl Router {
    // Function to create a new `Router`.
    // This function defines the routes for the application.
    pub fn new() -> axum::Router<crate::config::BeContext> {
        // Create a new `Router` and define the routes.
        axum::Router::new()
            // Route for the root path ("/").
            // This route returns the string "Hello Rust!" when accessed with a GET request.
            .route("/", axum::routing::get("Hello Rust!"))
            // Nested route for the "/api" path.
            // This route forwards requests to the `PostRouter`.
            .nest("/api", post::PostRouter::new())
            // Nested route for the "/api" path.
            // This route forwards requests to the `ProfilesRouter`.
            .nest("/api", profile::ProfilesRouter::new())
            // Nested route for the "/api" path.
            // This route forwards requests to the `UsersRouter`.
            .nest("/api", user::UsersRouter::new())

    }
}