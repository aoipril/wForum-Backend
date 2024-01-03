// Importing the necessary modules and functions.
use serde::{Deserialize, Serialize};
use prisma_client_rust::chrono::{DateTime, FixedOffset, TimeZone};

use crate::config::CONTEXT;
use crate::service::profile::model::Profile;
use crate::prisma::prisma::{platform_posts, post_comments};


// The `PostContent` struct which represents the content of a post.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostContent<T> {
    // The post content.
    pub post: T
}

// The `PostsBody` struct which represents the body of a post.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostsBody<T> {
    // The posts in the body.
    pub posts: Vec<T>,
    // The limit of posts.
    pub post_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryBody<T> {
    // The posts in the body.
    pub posts: Vec<T>,
    // The time of user's viewing history.
    pub time_vec: Vec<DateTime<FixedOffset>>,
    // The limit of posts.
    pub post_count: usize,
}

// The `CommentContent` struct which represents the content of a comment.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentContent<T> {
    // The comment content.
    pub comment: T
}

// The `CommentsContent` struct which represents the content of comments.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsContent<T> {
    // The comments content.
    pub comments: Vec<T>
}

// The `CreatePostPost` struct which represents the data for creating a post.
#[derive(Debug, Deserialize)]
pub struct CreatePostPost {
    // The title of the post.
    pub title: String,
    // The description of the post.
    pub description: String,
    // The content of the post.
    pub content: String
}

// The `UpdatePostPost` struct which represents the data for updating a post.
#[derive(Debug, Deserialize)]
pub struct UpdatePostPost {
    // The new title of the post.
    pub title: Option<String>,
    // The new description of the post.
    pub description: Option<String>,
    // The new content of the post.
    pub content: Option<String>,
}

// The `ListPostQuery` struct which represents the query parameters for listing posts.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostQuery {
    // The author of the posts.
    pub author: Option<String>,
    // The user who liked the posts.
    pub liked_by: Option<String>,
    // The limit of posts to list.
    pub limit: Option<i64>,
    // The offset for listing posts.
    pub offset: Option<i64>,
    // Whether to list posts from following users.
    pub following: Option<bool>,
}

// The `CommentCreateInput` struct which represents the input for creating a comment.
#[derive(Debug, Deserialize)]
pub struct CommentCreateInput {
    // The content of the comment.
    pub content: String,
}

// The `Post` struct which represents a post.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    // The ID of the post.
    pub post_id: i32,
    // The title of the post.
    pub title: String,
    // The description of the post.
    pub description: String,
    // The content of the post.
    pub content: String,
    // The creation timestamp of the post.
    pub created_at: DateTime<FixedOffset>,
    // Whether the post is liked.
    pub liked: bool,
    // The count of likes on the post.
    pub liked_count: i32,
    // The author of the post.
    pub author: Profile,
}

// The `Comment` struct which represents a comment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    // The ID of the comment.
    pub comment_id: i32,
    // The content of the comment.
    pub content: String,
    // The creation timestamp of the comment.
    pub created_at: DateTime<FixedOffset>,
    // The user who made the comment.
    pub user: Profile,
}


// Implementation of the `History` struct.
impl<T> HistoryBody<T> {
    pub fn with_timezone_offset(self) -> HistoryBody<T> {
        HistoryBody {
            posts: self.posts,
            time_vec:  self.time_vec.iter().map(|datetime| {
                FixedOffset::east_opt(3600 * CONTEXT.config.tz_east_offset_in_hours)
                    .unwrap().from_utc_datetime(&datetime.naive_utc())
            }).collect(),
            post_count: self.post_count,
        }
    }
}


// Implementation of the `platform_posts::Data` struct.
impl platform_posts::Data {
    // Function to convert `platform_posts::Data` into a `Post`.
    pub fn to_post(self, like: bool, followed: bool, following: bool, blocked:bool, blocking:bool,) -> Post {
        Post {
            post_id: self.post_id,
            title: self.title,
            description: self.description,
            content: self.content,
            created_at: FixedOffset::east_opt(3600 * CONTEXT.config.tz_east_offset_in_hours)
                .unwrap().from_utc_datetime(&self.created_at.naive_utc()),
            liked: like, liked_count: self.like_count,
            author: self.author.unwrap().to_profile(followed, following, blocked, blocking),
        }
    }
}


// Implementation of the `post_comments::Data` struct.
impl post_comments::Data {
    // Function to convert `post_comments::Data` into a `Comment`.
    pub fn to_comment(self, followed: bool, following: bool, blocked:bool, blocking:bool,) -> Comment {
        Comment {
            comment_id: self.comment_id,
            content: self.content,
            created_at: FixedOffset::east_opt(3600 * CONTEXT.config.tz_east_offset_in_hours)
                .unwrap().from_utc_datetime(&self.created_at.naive_utc()),
            user: self.user.unwrap().to_profile(followed, following, blocked, blocking),
        }
    }
}