// Importing the necessary modules and functions.
use crate::error::EError;
use crate::prisma::prisma;
use crate::prisma::prisma::{PrismaClient, user_blocks, user_follows, user_like_posts};


// Type alias for the Prisma client.
type PRISMA = axum::Extension<std::sync::Arc<PrismaClient>>;


// The `Checker` struct.
// This struct contains methods for checking various conditions in the application.
pub struct Checker;


// Implementation of the `Checker` struct.
impl Checker {

    // Function to check if a user is following another user.
    // It takes the Prisma client, the ID of the follower and the ID of the followed user as parameters.
    // It returns a `Result` with a `bool` indicating whether the user is following the other user or not.
    pub async fn check_following(
        prisma: &PRISMA,
        follower_id: i32,
        followed_id: i32,
    ) -> Result<bool, EError> {

        // Query the database to find a follow relationship between the two users.
        let following = prisma
            .user_follows()
            .find_unique(user_follows::follower_id_followed_id(
                follower_id, followed_id,
            ))
            .exec().await?;

        // Return `true` if the follow relationship exists, `false` otherwise.
        Ok(following.is_some())
    }

    // Function to check if a user has blocked another user.
    // It takes the Prisma client, the ID of the blocker and the ID of the blocked user as parameters.
    // It returns a `Result` with a `bool` indicating whether the user has blocked the other user or not.
    pub async fn check_blocked(
        prisma: &PRISMA,
        blocker_id: i32,
        blocked_id: i32,
    ) -> Result<bool, EError> {

        // Query the database to find a block relationship between the two users.
        let blocked = prisma
            .user_blocks()
            .find_unique(user_blocks::blocker_id_blocked_id(
                blocker_id,blocked_id
            ))
            .exec().await?;

        // Return `true` if the block relationship exists, `false` otherwise.
        Ok(blocked.is_some())
    }

    // Function to check if a user is the author of an article.
    // It takes the ID of the user and a reference to the article as parameters.
    // It returns a `Result` with a `bool` indicating whether the user is the author of the article or not.
    pub async fn check_author(
        user_id: i32,
        article: &prisma::platform_posts::Data,
    ) -> Result<bool, EError> {

        // If the user is the author of the article, return `true`.
        if article.author_id == user_id {
            return Ok(true);
        }

        // If the user is not the author of the article, return an error.
        Err(EError::BadRequest(String::from(
            "You are not the author of this article",
        )))
    }

    // Function to check if a user has liked an article.
    // It takes the Prisma client, the ID of the reader and the ID of the article as parameters.
    // It returns a `Result` with a `bool` indicating whether the user has liked the article or not.
    pub async fn check_liked(
        prisma: &PRISMA,
        reader_id: i32,
        article_id: i32,
    ) -> Result<bool, EError> {

        // Query the database to find a like relationship between the user and the article.
        let data = prisma
            .user_like_posts()
            .find_unique(
                user_like_posts::UniqueWhereParam::UserIdPostIdEquals(
                    reader_id, article_id,
                ))
            .exec().await?;

        // Return `true` if the like relationship exists, `false` otherwise.
        Ok(data.is_some())
    }
}