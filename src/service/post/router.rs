// Importing the necessary modules and functions.
use axum::routing::{delete, get, post, put};
use crate::{config::BeContext, service::post::service::PostService};


// The `PostRouter` struct which is responsible for routing HTTP requests to the appropriate handlers.
pub struct PostRouter;


// Implementation of the `PostRouter` struct.
impl PostRouter {
    // Function to create a new `PostRouter`.
    pub fn new() -> axum::Router<BeContext> {
        // Create a new `Router` and define the routes.
        axum::Router::new()
            // Route for fetching all posts.
            .route("/posts", get(PostService::fetch_posts))
            // Route for creating a new post.
            .route("/posts", post(PostService::create_post))
            // Route for fetching a specific post.
            .route("/posts/:post_id", get(PostService::fetch_post))
            // Route for updating a specific post.
            .route("/posts/:post_id", put(PostService::update_post))
            // Route for deleting a specific post.
            .route("/posts/:post_id", delete(PostService::delete_post))
            // Route for liking a specific post.
            .route("/posts/:post_id/like", post(PostService::like_post))
            // Route for unliking a specific post.
            .route("/posts/:post_id/like", delete(PostService::unlike_post))
            // Route for creating a comment on a specific post.
            .route("/posts/:post_id/comments", post(PostService::create_comment))
            // Route for fetching all comments on a specific post.
            .route("/posts/:post_id/comments", get(PostService::get_comments))
            // Route for deleting a specific comment on a specific post.
            .route("/posts/:post_id/comments/:comment_id", delete(PostService::delete_comment))
    }
}