use std::vec;
// Importing the necessary modules and services.
use rand::rngs::OsRng;
use axum::{extract::State, Json};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

// Importing the application's modules.
use crate::error::EError;
use crate::service::user::model::*;
use crate::config::BeContext;
use crate::extractor::extractor::AuthUser;
use crate::prisma::prisma::{platform_posts, post_comments, PrismaClient, user_blocks, user_details, user_follows, user_history, user_like_posts, user_password};


// Type alias for the Prisma client.
type PRISMA = axum::Extension<std::sync::Arc<PrismaClient>>;


// The `UsersService` struct.
// This struct contains methods for handling HTTP requests related to users.
pub struct UsersService;


// Implementation of the `UsersService` struct.
impl UsersService {

    // Function to fetch a user by their ID.
    // It takes an authenticated user, the application context, and the Prisma client as parameters.
    // It returns a `Result` with a JSON response containing the user's details or an error.
    pub async fn fetch_user(
        auth_user: AuthUser,
        ctx: State<BeContext>,
        prisma: PRISMA,
    ) -> Result<Json<UserBody<User>>, EError> {

        tracing::info!("Fetching user: user_id:{}", auth_user.user_id);

        let data = prisma
            .user_details().find_unique(user_details::user_id::equals(auth_user.user_id))
            .exec().await.unwrap();

        match data {
            Some(data) => {
                let mut user: User = data.into();
                user.set_token(auth_user.gen_jwt(&ctx));
                Ok(Json::from(UserBody { user }))
            }
            None => Err(EError::NotFound(String::from("User not found"))),
        }
    }


    // Function to log in a user.
    // It takes the Prisma client, the application context, and the user's login data as parameters.
    // It returns a `Result` with a JSON response containing the logged-in user's details or an error.
    pub async fn login_user(
        prisma: PRISMA,
        ctx: State<BeContext>,
        Json(input): Json<UserBody<LoginUserPost>>,
    ) -> Result<Json<UserBody<User>>, EError> {

        let UserBody {
            user: LoginUserPost { email, password },
        } = input;

        tracing::info!("Logging in user: email: {}", email);

        let user_data = prisma
            .user_details().find_unique(user_details::email::equals(email))
            .exec().await?;

        let user_data = match user_data {
            Some(user_data) => user_data,
            None => return Err(EError::NotFound(String::from("User not found"))),
        };

        let password_data = prisma
            .user_password().find_unique(user_password::user_id::equals(user_data.user_id))
            .exec().await?.unwrap();

        match Self::verify_password(password.as_str(),
                                    password_data.hash_password.as_str()) {
            Ok(_) => (),
            Err(_) => return Err(EError::Unauthorized(String::from("Invalid password"))),
        };

        let mut user: User = user_data.into();

        let token = AuthUser { user_id: user.user_id }.gen_jwt(&ctx);
        user.set_token(token);

        Ok(Json::from(UserBody { user }))
    }


    // Function to update a user's details.
    // It takes the Prisma client, an authenticated user, the application context, and the new user data as parameters.
    // It returns a `Result` with a JSON response containing the updated user's details or an error.
    pub async fn update_user(
        prisma: PRISMA,
        auth_user: AuthUser,
        ctx: State<BeContext>,
        Json(input): Json<UserBody<UpdateUserPost>>,
    ) -> Result<Json<UserBody<User>>, EError> {

        tracing::info!("Updating user: user_id: {}", auth_user.user_id);

        let UserBody {
            user:
            UpdateUserPost {
                email,
                intro,
                avatar,
                username,
                password,
            },
        } = input;

        let user_data = prisma
            .user_details().find_unique(user_details::user_id::equals(auth_user.user_id))
            .exec().await?;

        let user_data = match user_data {
            Some(user_data) => user_data,
            None => return Err(EError::NotFound(String::from("User not found"))),
        };

        let user_data = prisma
            .user_details()
            .update(
                user_details::user_id::equals(auth_user.user_id),
                vec![
                    match intro {
                        Some(intro) => user_details::intro::set(Some(intro)),
                        None => user_details::intro::set(user_data.intro),
                    },
                    match avatar {
                        Some(avatar) => user_details::avatar::set(Some(avatar)),
                        None => user_details::avatar::set(user_data.avatar),
                    },
                    match email {
                        Some(email) => user_details::email::set(email),
                        None => user_details::email::set(user_data.email),
                    },
                    match username {
                        Some(username) => user_details::username::set(username),
                        None => user_details::username::set(user_data.username),
                    },
                ],
            )
            .exec().await?;

        if let Some(password) = password {
            let password_data = prisma
                .user_password().find_unique(user_password::user_id::equals(auth_user.user_id))
                .exec().await?.unwrap();

            let _ = prisma
                .user_password()
                .update(
                    user_password::user_id::equals(password_data.user_id),
                    vec![
                        user_password::hash_password::set(
                            Self::hash_password(password.as_str()).unwrap(),
                        )
                    ]
                ).exec().await?;
        }

        let mut user: User = user_data.into();

        let token = AuthUser { user_id: user.user_id }.gen_jwt(&ctx);
        user.set_token(token);

        Ok(Json::from(UserBody { user }))
    }


    // Function to create a new user.
    // It takes the Prisma client, the application context, and the new user data as parameters.
    // It returns a `Result` with a JSON response containing the created user's details or an error.
    pub async fn create_user(
        prisma: PRISMA,
        ctx: State<BeContext>,
        Json(input): Json<UserBody<CreateUserPost>>,
    ) -> Result<Json<UserBody<User>>, EError> {

        let UserBody {
            user:
            CreateUserPost {
                email, username, password,
            },
        } = input;

        tracing::info!("Creating user: email: {}", email);

        let user_data = prisma
            .user_details()
            .create(
                email, username, vec![],
            ).exec().await?;

        let _ = prisma.user_password()
            .create(
            Self::hash_password(password.as_str()).unwrap(),
            user_details::user_id::equals(user_data.user_id),
            vec![]
            ).exec().await?;

        let token = AuthUser { user_id: user_data.user_id }.gen_jwt(&ctx);

        let mut user: User = user_data.into();
        user.set_token(token);

        Ok(Json::from(UserBody { user }))
    }


    // Function to delete a user.
    // It takes the Prisma client and an authenticated user as parameters.
    // It returns a `Result` with a JSON response containing the deleted user's details or an error.
    pub async fn delete_user(
        prisma: PRISMA,
        auth_user: AuthUser,
    ) -> Result<String, EError> {

        let user_data = prisma
            .user_details().find_unique(user_details::user_id::equals(auth_user.user_id))
            .exec().await?;

        let _ = match user_data {
            Some(user_data) => user_data,
            None => return Err(EError::NotFound(String::from("User not found"))),
        };

        tracing::info!("Deleting user: user_id: {}", auth_user.user_id);

        // Delete user's follows
        let _ = prisma
            .user_follows()
            .delete_many(vec![user_follows::follower_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's blocks
        let _ = prisma
            .user_blocks()
            .delete_many(vec![user_blocks::blocker_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's comments
        let _ = prisma
            .post_comments()
            .delete_many(vec![post_comments::user_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's likes
        let _ = prisma
            .user_like_posts()
            .delete_many(vec![user_like_posts::user_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's history
        let _ = prisma
            .user_history()
            .delete_many(vec![user_history::user_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's posts
        let _ = prisma
            .platform_posts()
            .delete_many(vec![platform_posts::author_id::equals(auth_user.user_id)])
            .exec().await?;

        // Delete user's password
        let _ = prisma
            .user_password()
            .delete(user_password::user_id::equals(auth_user.user_id))
            .exec().await?;

        // Delete user
        let _ = prisma
            .user_details()
            .delete(user_details::user_id::equals(auth_user.user_id))
            .exec().await?;

        Ok("User deleted".to_string())
    }


    // Utility functions for the `UsersService` struct.

    // Function to hash a password.
    // It takes a password as a parameter.
    // It returns a `Result` with a `String` containing the hashed password or an error.
    fn hash_password(password: &str) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| anyhow::anyhow!("failed to hash password"))?;

        Ok(password_hash.to_string())
    }


    // Function to verify a password against a hashed password.
    // It takes a password and a hashed password as parameters.
    // It returns a `Result` indicating whether the password is valid or an error.
    fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<()> {
        let argon2 = Argon2::default();
        // Parse password hash from PHC string
        let password_hash = PasswordHash::new(password_hash)
            .map_err(|_| anyhow::anyhow!("failed to parse password hash from PHC string"))?;
        // Verify password against hash
        argon2
            .verify_password(password.as_bytes(), &password_hash)
            .map_err(|_| anyhow::anyhow!("failed to verify password"))?;
        Ok(())
    }
}
