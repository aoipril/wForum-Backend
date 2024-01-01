// Importing the necessary modules and services.
use axum::Json;
use axum::extract::{Path, Query};

// Importing the application's modules.
use crate::error::EError;
use crate::service::post::*;
use crate::service::utils::helper::Helper;
use crate::service::utils::checker::Checker;
use crate::extractor::extractor::{AuthUser, OptionalAuthUser};
use crate::prisma::prisma::{
    platform_posts, post_comments, user_details, user_follows, user_like_posts, PrismaClient
};

// Type alias for the Prisma client.
type PRISMA = axum::Extension<std::sync::Arc<PrismaClient>>;

// The `PostService` struct.
// This struct contains methods for handling HTTP requests related to posts.
pub struct PostService;

// Implementation of the `PostService` struct.
impl PostService {

    // Function to fetch a post by its ID.
    // It takes an optional authenticated user, the Prisma client and the post's ID as parameters.
    // It returns a `Result` with a JSON response containing the post's details or an error.
    pub async fn fetch_post(
        auth_user: OptionalAuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
    ) -> Result<Json<PostContent<Post>>, EError> {

        tracing::debug!("Fetching Post: post_id: {}",post_id);

        let post = Helper::fetch_post(&prisma, post_id).await?;

        if let Some(user) = auth_user.0 {
            let liked = Checker::check_liked(&prisma, user.user_id, post.post_id).await?;
            let followed =
                Checker::check_following(&prisma, post.author_id, user.user_id,).await?;
            let following =
                Checker::check_following(&prisma, user.user_id, post.author_id).await?;
            let blocked =
                Checker::check_blocked(&prisma, post.author_id, user.user_id,).await?;
            let blocking =
                Checker::check_blocked(&prisma, user.user_id, post.author_id).await?;

            return Ok(Json::from(PostContent {
                post: post.to_post(liked, followed, following, blocked, blocking),
            }));
        }

        Ok(Json::from(PostContent {
            post: post.to_post(false, false, false,
                               false, false),
        }))
    }


    // Function to fetch multiple posts based on provided filters.
    // It takes an optional authenticated user, the Prisma client and the query parameters as parameters.
    // It returns a `Result` with a JSON response containing a list of posts or an error.
    pub async fn fetch_posts(
        user: OptionalAuthUser,
        prisma: PRISMA,
        Query(query): Query<ListPostQuery>,
    ) -> Result<Json<PostsBody<Post>>, EError> {

        tracing::debug!("Fetching Posts");

        let mut filter: Vec<platform_posts::WhereParam> = Vec::new();

        if let Some(author) = query.author {
            filter.push(platform_posts::author::is(
                vec![user_details::username::equals(author)]))
        }

        if let Some(liked_by) = query.liked_by {
            filter.push(platform_posts::liked_by_users::some(vec![
                user_like_posts::user::is(vec![user_details::username::equals(liked_by)])]))
        }

        if let Some(true) = query.following {
            if let Some(auth_user) = user.clone().0 {
                // Get all users that the current user is following
                let followed_users = prisma.user_follows()
                    .find_many(vec![user_follows::follower_id::equals(auth_user.user_id)])
                    .exec().await?;

                // Get the user_ids of the followed users
                let followed_user_ids: Vec<i32> = followed_users
                    .iter()
                    .map(|follow| follow.followed_id)
                    .collect();

                // Add the posts of the followed users to the filter
                filter.push(platform_posts::author_id::in_vec(followed_user_ids));
            }else {
                return Err(EError::Unauthorized(String::from("Login to filter following author's post")));
            }
        }

        let _post = Helper::fetch_posts(&prisma, filter.clone(), query.limit, query.offset).await?;

        let limit = prisma.platform_posts().count(filter).exec().await?;

        let mut posts: Vec<Post> = Vec::new();

        if let Some(auth_user) = user.0 {
            for post in _post.iter() {
                Helper::push_post(&prisma, &mut posts, post, auth_user.user_id).await?;
            }
        } else {
            posts = _post
                .iter()
                .map(|post| post.clone().to_post(false, false, false,
                                                 false, false))
                .collect();
        }

        Ok(Json::from(PostsBody {
            posts,
            limit: limit as usize,
        }))
    }


    // Function to create a new post.
    // It takes an authenticated user, the Prisma client and the post data as parameters.
    // It returns a `Result` with a JSON response containing the created post's details or an error.
    pub async fn create_post(
        auth_user: AuthUser,
        prisma: PRISMA,
        Json(input): Json<PostContent<CreatePostPost>>,
    ) -> Result<Json<PostContent<Post>>, EError> {

        tracing::debug!("Creating post: user_id: {}", auth_user.user_id);

        let PostContent {
            post:
            CreatePostPost {
                title,
                description,
                content,
            },
        } = input;

        let post_data = prisma
            .platform_posts()
            .create(
                title,
                description,
                content,
                user_details::user_id::equals(auth_user.user_id),
                vec![],
            )
            .with(platform_posts::author::fetch())
            .exec().await?;

        Ok(Json::from(PostContent {
            post: post_data.to_post(false, false, false,
                                    false, false),
        }))
    }


    // Function to update a post.
    // It takes an authenticated user, the Prisma client, the post's ID and the new post data as parameters.
    // It returns a `Result` with a JSON response containing the updated post's details or an error.
    pub async fn update_post(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
        Json(input): Json<PostContent<UpdatePostPost>>,
    ) -> Result<Json<PostContent<Post>>, EError> {

        tracing::debug!("Updating Post: user_id: {}, post_id: {}", auth_user.user_id, post_id);

        let PostContent {
            post:
            UpdatePostPost {
                title,
                description,
                content,
            },
        } = input;

        let post = prisma
            .platform_posts()
            .find_unique(platform_posts::post_id::equals(post_id.parse().unwrap()))
            .with(platform_posts::author::fetch())
            .exec().await?
            .ok_or(EError::NotFound(String::from("Post not found")))?;

        Checker::check_author(auth_user.user_id, &post).await?;

        let updated_post = prisma
            .platform_posts()
            .update(
                platform_posts::post_id::equals(post_id.parse().unwrap()),
                vec![
                    match title {
                        Some(title) => platform_posts::title::set(title),
                        None => platform_posts::title::set(post.title),
                    },
                    match description {
                        Some(description) => platform_posts::description::set(description),
                        None => platform_posts::description::set(post.description),
                    },
                    match content {
                        Some(content) => platform_posts::content::set(content),
                        None => platform_posts::content::set(post.content),
                    },
                ],
            )
            .with(platform_posts::author::fetch())
            .exec().await?;

        Ok(Json::from(PostContent {
            post: updated_post.to_post(false, false, false,
                                       false, false),
        }))
    }


    // Function to delete a post.
    // It takes an authenticated user, the Prisma client and the post's ID as parameters.
    // It returns a `Result` with a JSON response containing a success message or an error.
    pub async fn delete_post(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
    ) -> Result<Json<String>, EError> {

        tracing::debug!("Deleting post: user_id: {}, post_id: {}", auth_user.user_id, post_id);

        let post = Helper::fetch_post(&prisma, post_id.parse().unwrap()).await?;

        Checker::check_author(auth_user.user_id, &post).await?;

        let _ = prisma
            .platform_posts()
            .delete(platform_posts::post_id::equals(post_id.parse().unwrap()),)
            .exec().await?;

        Ok(Json::from("Post deleted".to_string()))
    }


    // Function to like a post.
    // It takes an authenticated user, the Prisma client and the post's ID as parameters.
    // It returns a `Result` with a JSON response containing the liked post's details or an error.
    pub async fn like_post(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
    ) -> Result<Json<PostContent<Post>>, EError> {

        tracing::info!("Liking Post: user_id: {}, post_id: {}", auth_user.user_id, post_id);

        let post_data = Helper::fetch_post(&prisma, post_id.parse().unwrap()).await?;

        if Checker::check_blocked(&prisma, post_data.author_id, auth_user.user_id).await? {
            return Err(EError::Forbidden(String::from(
                "You are blocked by the author of this post",
            ))); }

        if Checker::check_liked(&prisma, auth_user.user_id, post_data.post_id).await? {
            return Err(EError::BadRequest(String::from(
                "You have already liked this post",
            ))); }

        let _ = prisma
            .user_like_posts()
            .create(
                user_details::user_id::equals(auth_user.user_id),
                platform_posts::post_id::equals(post_data.post_id),
                vec![],
            )
            .exec().await?;

        let post = prisma
            .platform_posts()
            .update(
                platform_posts::post_id::equals(post_id.parse().unwrap()),
                vec![platform_posts::like_count::increment(1)],
            )
            .with(platform_posts::author::fetch())
            .exec().await?;

        let followed =
            Checker::check_following(&prisma, post.author_id, auth_user.user_id,).await?;
        let following =
            Checker::check_following(&prisma, auth_user.user_id, post.author_id).await?;
        let blocking =
            Checker::check_blocked(&prisma, auth_user.user_id, post.author_id).await?;

        Ok(Json::from(PostContent {
            post: post.to_post(true, followed, following, false, blocking),
        }))
    }


    // Function to unlike a post.
    // It takes an authenticated user, the Prisma client and the post's ID as parameters.
    // It returns a `Result` with a JSON response containing the unliked post's details or an error.
    pub async fn unlike_post(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
    ) -> Result<Json<PostContent<Post>>, EError> {

        tracing::info!("Unliking Post: user_id: {}, post_id: {}", auth_user.user_id, post_id);

        let post = Helper::fetch_post(&prisma, post_id.parse().unwrap()).await?;

        if Checker::check_blocked(&prisma, post.author_id, auth_user.user_id).await? {
            return Err(EError::Forbidden(String::from(
                "You are blocked by the author of this post",
            ))); }

        if !Checker::check_liked(&prisma, auth_user.user_id, post.post_id).await? {
            return Err(EError::BadRequest(String::from(
                "You have not liked this post",
            ))); }

        let _ = prisma
            .user_like_posts()
            .delete(user_like_posts::user_id_post_id(
                auth_user.user_id,
                post.post_id,
            ))
            .exec().await?;

        let post = prisma
            .platform_posts()
            .update(
                platform_posts::post_id::equals(post_id.parse().unwrap()),
                vec![platform_posts::like_count::decrement(1)],
            )
            .with(platform_posts::author::fetch())
            .exec().await?;

        let followed =
            Checker::check_following(&prisma, post.author_id, auth_user.user_id,).await?;
        let following =
            Checker::check_following(&prisma, auth_user.user_id, post.author_id).await?;
        let blocking =
            Checker::check_blocked(&prisma, auth_user.user_id, post.author_id).await?;

        Ok(Json::from(PostContent {
            post: post.to_post(false, followed, following, false, blocking),
        }))
    }


    // Function to fetch all comments on a post.
    // It takes an optional authenticated user, the Prisma client and the post's ID as parameters.
    // It returns a `Result` with a JSON response containing a list of comments or an error.
    pub async fn get_comments(
        auth_user: OptionalAuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
    ) -> Result<Json<CommentsContent<Comment>>, EError> {

        tracing::info!("Getting comments: post_id: {}", post_id);

        let post = Helper::fetch_post(&prisma, post_id.parse().unwrap()).await?;

        let comments = prisma
            .post_comments()
            .find_many(vec![
                post_comments::post_id::equals(post.post_id),
            ])
            .with(post_comments::user::fetch())
            .exec().await?;

        let mut comments: Vec<Comment> = comments
            .iter()
            .map(|comment| comment.clone().to_comment(false, false,
                                                      false, false))
            .collect();

        if let Some(user) = auth_user.0 {
            for comment in comments.iter_mut() {
                let followed =
                    Checker::check_following(&prisma, user.user_id, post.author_id).await?;

                comment.user.following = followed;
            }
        }

        Ok(Json::from(CommentsContent { comments }))
    }


    // Function to create a new comment on a post.
    // It takes an authenticated user, the Prisma client, the post's ID and the comment data as parameters.
    // It returns a `Result` with a JSON response containing the created comment's details or an error.
    pub async fn create_comment(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path(post_id): Path<String>,
        Json(input): Json<CommentContent<CommentCreateInput>>,
    ) -> Result<Json<CommentContent<Comment>>, EError> {

        tracing::info!("Creating comment: user_id: {}, post_id: {}", auth_user.user_id, post_id);

        let CommentContent {
            comment: CommentCreateInput { content: body },
        } = input;

        let post = Helper::fetch_post(&prisma, post_id.parse().unwrap()).await?;

        if Checker::check_blocked(&prisma, post.author_id, auth_user.user_id).await? {
            return Err(EError::Forbidden(String::from(
                "You are blocked by the author of this post",
            )));}

        let comment = prisma
            .post_comments()
            .create(
                body,
                user_details::user_id::equals(auth_user.user_id),
                platform_posts::post_id::equals(post.post_id),
                vec![],
            )
            .with(post_comments::user::fetch())
            .exec().await?;

        let blocking =
            Checker::check_blocked(&prisma, auth_user.user_id, post.author_id).await?;

        Ok(Json::from(CommentContent {
            comment: comment.to_comment(false, false, false, blocking),
        }))
    }


    // Function to delete a comment on a post.
    // It takes an authenticated user, the Prisma client and the post's ID and comment's ID as parameters.
    // It returns a `Result` with a JSON response containing a success message or an error.
    pub async fn delete_comment(
        auth_user: AuthUser,
        prisma: PRISMA,
        Path((_post_id, comment_id)): Path<(String, i32)>,
    ) -> Result<Json<String>, EError> {

        tracing::info!("Deleting comment: user_id: {}, post_id: {}, comment_id: {}"
            , auth_user.user_id, _post_id, comment_id);

        let comment = prisma
            .post_comments()
            .find_unique(post_comments::comment_id::equals(comment_id))
            .with(post_comments::user::fetch())
            .exec().await?
            .ok_or(EError::NotFound(String::from("Comment not found")))?;

        if comment.user_id != auth_user.user_id {
            return Err(EError::BadRequest(String::from(
                "You are not the author of this comment",
            ))); }

        let _ = prisma
            .post_comments()
            .delete(
                post_comments::comment_id::equals(comment_id),
            )
            .exec().await?;

        Ok(Json::from("Comment deleted".to_string()))
    }
}
