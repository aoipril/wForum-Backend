// Importing the necessary modules and services.
use axum::Json;
use axum::extract::Path;

// Importing the application's modules.
use crate::service::utils::helper::Helper;
use crate::service::utils::checker::Checker;
use crate::service::profile::model::{Profile, ProfileBody};
use crate::error::EError;
use crate::extractor::extractor::{AuthUser, OptionalAuthUser};
use crate::prisma::prisma::{user_blocks, user_details, user_follows, PrismaClient};


// Type alias for the Prisma client.
type PRISMA = axum::Extension<std::sync::Arc<PrismaClient>>;


// The `ProfilesService` struct.
// This struct contains methods for handling HTTP requests related to profiles.
pub struct ProfilesService;


// Implementation of the `ProfilesService` struct.
impl ProfilesService {

    // Function to fetch a profile by its username.
    // It takes an optional authenticated user, the Prisma client and the username as parameters.
    // It returns a `Result` with a JSON response containing the profile's details or an error.
    pub async fn fetch_profile(
        Path(username): Path<String>,
        auth_user: OptionalAuthUser,
        prisma: PRISMA,
    ) -> Result<Json<ProfileBody<Profile>>, EError> {

        tracing::info!("Fetching profile: username: {}", username);

        let visited_user = Helper::get_user_by_name(&prisma, username).await?;

        return match auth_user.0 {
            Some(user) => {
                let followed =
                    Checker::check_following(&prisma, visited_user.user_id, user.user_id,).await?;
                let following =
                    Checker::check_following(&prisma, user.user_id, visited_user.user_id).await?;
                let blocked =
                    Checker::check_blocked(&prisma, visited_user.user_id, user.user_id).await?;
                let blocking =
                    Checker::check_blocked(&prisma, user.user_id, visited_user.user_id,).await?;
                Ok(Json::from(ProfileBody {
                    profile: visited_user.to_profile(followed, following, blocking, blocked),
                }))
            }
            None => Ok(Json::from(ProfileBody {
                profile: visited_user.to_profile(false, false,
                                                 false, false),
            })),
        };
    }


    // Function to follow a profile.
    // It takes an authenticated user, the Prisma client and the username as parameters.
    // It returns a `Result` with a JSON response containing the followed profile's details or an error.
    pub async fn follow_profile(
        Path(username): Path<String>,
        auth_user: AuthUser,
        prisma: PRISMA,
    ) -> Result<Json<ProfileBody<Profile>>, EError> {

        let current_user = Helper::get_user_by_id(&prisma, auth_user.user_id).await?;

        if current_user.username == username {
            return Err(EError::BadRequest(String::from("You cannot follow yourself")));
        }

        let followed_user = Helper::get_user_by_name(&prisma, username).await?;

        tracing::info!("Following profile: username: {} to {}",
            current_user.username, followed_user.username);

        if Checker::check_following(&prisma, auth_user.user_id, followed_user.user_id).await? {
            return Err(EError::BadRequest(String::from("You are already following this user")));
        }

        if Checker::check_blocked(&prisma, followed_user.user_id, auth_user.user_id,).await? {
            return Err(EError::BadRequest(String::from("Current user has been blocked")));
        }

        if Checker::check_blocked(&prisma, auth_user.user_id, followed_user.user_id).await? {
            return Err(EError::BadRequest(String::from("Current user are blocking this user")));
        }

        prisma
            .user_follows()
            .upsert(
                user_follows::follower_id_followed_id(current_user.user_id, followed_user.user_id),
                user_follows::create(
                    user_details::user_id::equals(current_user.user_id),
                    user_details::user_id::equals(followed_user.user_id),
                    vec![],
                ),
                vec![],
            )
            .exec()
            .await?;

        let followed =
            Checker::check_following(&prisma, followed_user.user_id, auth_user.user_id,).await?;

        Ok(Json::from(ProfileBody {
            profile: followed_user.to_profile(followed, true,
                                              false, false),
        }))
    }


    // Function to unfollow a profile.
    // It takes an authenticated user, the Prisma client and the username as parameters.
    // It returns a `Result` with a JSON response containing the unfollowed profile's details or an error.
    pub async fn unfollow_profile(
        Path(username): Path<String>,
        auth_user: AuthUser,
        prisma: PRISMA,
    ) -> Result<Json<ProfileBody<Profile>>, EError> {

        let current_user = Helper::get_user_by_id(&prisma, auth_user.user_id).await?;

        if current_user.username == username {
            return Err(EError::BadRequest(String::from("You cannot unfollow yourself")));
        }

        let followed_user = Helper::get_user_by_name(&prisma, username).await?;

        tracing::info!("Unfollowing profile: username: {} to {}",
            current_user.username, followed_user.username);

        if !Checker::check_following(&prisma, auth_user.user_id, followed_user.user_id).await? {
            return Err(EError::BadRequest(String::from("Current user did not follow")));
        }

        let _ = prisma
            .user_follows()
            .delete(user_follows::follower_id_followed_id(
                current_user.user_id,
                followed_user.user_id,
            ))
            .exec().await.is_ok();

        let followed =
            Checker::check_following(&prisma, followed_user.user_id, auth_user.user_id,).await?;

        Ok(Json::from(ProfileBody {
            profile: followed_user.to_profile(followed, false,
                                              false, false),
        }))
    }


    // Function to block a profile.
    // It takes an authenticated user, the Prisma client and the username as parameters.
    // It returns a `Result` with a JSON response containing the blocked profile's details or an error.
    pub async fn block_profile(
        Path(username): Path<String>,
        auth_user: AuthUser,
        prisma: PRISMA,
    ) -> Result<Json<ProfileBody<Profile>>, EError> {

        let current_user = Helper::get_user_by_id(&prisma, auth_user.user_id).await?;

        if current_user.username == username {
            return Err(EError::BadRequest(String::from("You cannot block yourself", )));
        }

        let blocked_user = Helper::get_user_by_name(&prisma, username).await?;

        tracing::info!("Blocking profile: username: {} to {}",
            current_user.username, blocked_user.username);

        if Checker::check_blocked(&prisma, auth_user.user_id, blocked_user.user_id).await? {
            return Err(EError::BadRequest(String::from("User has already been blocked")));
        }

        let _ = prisma
            .user_follows()
            .delete(user_follows::follower_id_followed_id(
                current_user.user_id,
                blocked_user.user_id,
            ))
            .exec().await.is_ok();

        let _ = prisma
            .user_follows()
            .delete(user_follows::follower_id_followed_id(
                blocked_user.user_id,
                current_user.user_id,
            ))
            .exec().await.is_ok();

        let _ = prisma
            .user_blocks()
            .upsert(
                user_blocks::blocker_id_blocked_id(current_user.user_id, blocked_user.user_id ),
                user_blocks::create(
                    user_details::user_id::equals(current_user.user_id),
                    user_details::user_id::equals(blocked_user.user_id),
                    vec![],
                ),
                vec![],
            )
            .exec().await?;

        let blocked =
            Checker::check_blocked(&prisma, blocked_user.user_id, auth_user.user_id,).await?;

        Ok(Json::from(ProfileBody {
            profile: blocked_user.to_profile(false, false,
                                             blocked, true),
        }))
    }


    // Function to unblock a profile.
    // It takes an authenticated user, the Prisma client and the username as parameters.
    // It returns a `Result` with a JSON response containing the unblocked profile's details or an error.
    pub async fn unblock_profile(
        Path(username): Path<String>,
        auth_user: AuthUser,
        prisma: PRISMA,
    ) -> Result<Json<ProfileBody<Profile>>, EError> {

        let current_user = Helper::get_user_by_id(&prisma, auth_user.user_id).await?;

        if current_user.username == username {
            return Err(EError::BadRequest(String::from("You cannot unblock yourself", )));
        }

        let blocked_user = Helper::get_user_by_name(&prisma, username).await?;

        tracing::info!("Unblocking profile: username: {} to {}",
            current_user.username, blocked_user.username);

        if !Checker::check_blocked(&prisma, auth_user.user_id, blocked_user.user_id).await? {
            return Err(EError::BadRequest(String::from("Current user did not block")));
        }

        let _ = prisma
            .user_blocks()
            .delete(user_blocks::blocker_id_blocked_id(
                current_user.user_id,
                blocked_user.user_id,
            ))
            .exec().await.is_ok();

        let blocked =
            Checker::check_blocked(&prisma, blocked_user.user_id, auth_user.user_id,).await?;

        Ok(Json::from(ProfileBody {
            profile: blocked_user.to_profile(false, false,
                                             blocked, false),
        }))
    }
}
