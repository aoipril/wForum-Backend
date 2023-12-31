// Importing the necessary modules and functions.
use crate::error::EError;
use crate::service::post::Post;
use crate::service::utils::checker::Checker;
use crate::prisma::prisma::{platform_posts, user_details, PrismaClient};


// Type alias for the Prisma client.
type PRISMA = axum::Extension<std::sync::Arc<PrismaClient>>;


// The `Helper` struct.
// This struct contains helper methods used throughout the application.
pub struct Helper;

// Implementation of the `Helper` struct.
impl Helper {

    // Function to convert a value to seconds based on the provided unit.
    // It takes a value and a unit as parameters and returns the value in seconds.
    pub fn value_to_seconds(value: i64, unit: String) -> i64 {

        match unit.as_str() {
            "seconds" => value,
            "minutes" => value * 60,
            "hours" => value * 3600,
            "days" => value * 86400,
            "weeks" => value * 604800,
            "months" => value * 2592000,
            "years" => value * 31536000,
            _ => panic!("Invalid unit"),
        }
    }

    // Function to get a user by their ID.
    // It takes the Prisma client and the user's ID as parameters.
    // It returns a `Result` with the user's details or an error.
    pub async fn get_user_by_id(
        prisma: &PRISMA,
        user_id: i32,
    ) -> Result<user_details::Data, EError> {

        let data = prisma
            .user_details()
            .find_unique(user_details::user_id::equals(user_id))
            .exec().await?;

        match data {
            Some(data) => Ok(data),
            None => Err(EError::NotFound(String::from("User not found"))),
        }
    }

    // Function to get a user by their username.
    // It takes the Prisma client and the username as parameters.
    // It returns a `Result` with the user's details or an error.
    pub async fn get_user_by_name(
        prisma: &PRISMA,
        username: String,
    ) -> Result<user_details::Data, EError> {

        let data = prisma
            .user_details()
            .find_unique(user_details::username::equals(username))
            .exec().await?;

        match data {
            Some(data) => Ok(data),
            None => Err(EError::NotFound(String::from("User not found"))),
        }
    }

    // Function to fetch a post by its ID.
    // It takes the Prisma client and the post's ID as parameters.
    // It returns a `Result` with the post's details or an error.
    pub async fn fetch_post(
        prisma: &PRISMA,
        post_id: String,
    ) -> Result<platform_posts::Data, EError> {

        let post_id: i32 = post_id.parse()
            .map_err(|_| EError::BadRequest(String::from("Invalid post id")))?;

        let data = prisma
            .platform_posts()
            .find_unique(platform_posts::post_id::equals(post_id))
            .with(platform_posts::author::fetch())
            .exec().await?;

        match data {
            Some(data) => Ok(data),
            None => Err(EError::NotFound(String::from("Post not found"))),
        }
    }

    // Function to fetch multiple posts based on provided filters.
    // It takes the Prisma client, a vector of filters, a limit and an offset as parameters.
    // It returns a `Result` with a vector of posts or an error.
    pub async fn fetch_posts(
        prisma: &PrismaClient,
        filter: Vec<platform_posts::WhereParam>,
        query_limit: Option<i64>,
        query_offset: Option<i64>,
    ) -> Result<Vec<platform_posts::Data>, EError> {

        let posts = prisma
            .platform_posts()
            .find_many(filter)
            .with(platform_posts::author::fetch())
            .take(query_limit.unwrap_or(20))
            .skip(query_offset.unwrap_or(0))
            .order_by(platform_posts::created_at::order(prisma_client_rust::Direction::Desc))
            .exec().await
            .map_err(|_| EError::InternalServerError(String::from("Failed to fetch posts")))?;

        Ok(posts)
    }

    // Function to add a post to a vector of posts.
    // It takes the Prisma client, a mutable reference to a vector of posts, a reference to a post and a user ID as parameters.
    // It checks if the user has liked the post and if the user is following the author of the post, then adds the post to the vector.
    pub async fn push_post(
        prisma: &PRISMA,
        posts: &mut Vec<Post>,
        post: &platform_posts::Data,
        user_id: i32,
    ) -> Result<(), EError> {

        let like = Checker::check_liked(&prisma, user_id, post.post_id).await?;
        let followed =
            Checker::check_following(&prisma, post.author_id, user_id,).await?;
        let following =
            Checker::check_following(&prisma, user_id, post.author_id).await?;
        let blocked =
            Checker::check_blocked(&prisma, post.author_id, user_id,).await?;
        let blocking =
            Checker::check_blocked(&prisma, user_id, post.author_id).await?;

        Ok(posts.push(post.clone().to_post(like, followed, following, blocked, blocking)))
    }

}