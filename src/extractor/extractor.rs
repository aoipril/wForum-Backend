// Importing the necessary modules and functions.
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::{header::AUTHORIZATION, request::Parts, HeaderValue};
use prisma_client_rust::chrono;

use crate::config::BeContext;
use crate::error::EError;


// Constant for the authorization header scheme.
const AUTH_HEADER_SCHEME: &str = "Bearer ";


// The `AuthUser` struct which represents an authenticated user.
#[derive(Debug, Clone)]
pub struct AuthUser {
    // The user's ID.
    pub user_id: i32,
}

// The `OptionalAuthUser` struct which represents an optional authenticated user.
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<AuthUser>);

// The `AuthUserClaims` struct which represents the claims in a JWT.
#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    // The user's ID.
    user_id: i32,
    // The expiration timestamp of the JWT.
    exp: i64,
}


// Implementation of the `AuthUser` struct.
impl AuthUser {

    // Function to generate a JWT for the user.
    pub fn gen_jwt(&self, ctx: &BeContext) -> String {
        let key = jsonwebtoken::EncodingKey::from_secret(ctx.config.jwt_config.jwt_secret.as_ref());
        let claims = AuthUserClaims {
            user_id: self.user_id,
            exp: chrono::Utc::now().timestamp() + ctx.config.jwt_config.jwt_exp_value,
        };

        jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &key)
            .expect("Failed to generate JWT")
    }

    // Function to create an `AuthUser` from an authorization header.
    fn from_authorization(ctx: &BeContext, auth_header: &HeaderValue) -> Result<Self, EError> {

        let auth_header = auth_header.to_str().map_err(|_| {
            tracing::info!("Authorization header is not UTF-8");
            EError::Unauthorized(String::from("Authorization header is not UTF-8"))
        })?;

        if !auth_header.starts_with(AUTH_HEADER_SCHEME) {
            tracing::info!(
                "Authorization header is using the wrong scheme: {:?}",auth_header
            );
            return Err(EError::Unauthorized(String::from(
                "Authorization header is using the wrong scheme",
            )));
        }

        let token = &auth_header[AUTH_HEADER_SCHEME.len()..];

        tracing::debug!("Incoming token: {:?}", token);

        let jwt = jsonwebtoken::decode::<AuthUserClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(ctx.config.jwt_config.jwt_secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        )
            .map_err(|e| {
                tracing::info!("JWT validation failed: {:?}", e);
                EError::Unauthorized(String::from("JWT validation failed"))
            })?;

        let jsonwebtoken::TokenData { header, claims } = jwt;

        if header.alg != jsonwebtoken::Algorithm::HS256 {
            tracing::info!("JWT is using the wrong algorithm: {:?}", header.alg);
            return Err(EError::Unauthorized(String::from(
                "JWT is using the wrong algorithm",
            )));
        }

        if claims.exp < prisma_client_rust::chrono::Utc::now().timestamp() {
            tracing::info!("JWT is expired");
            return Err(EError::Unauthorized(String::from("JWT is expired")));
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}


// Implementation of the `From` trait for `OptionalAuthUser`.
impl From<OptionalAuthUser> for Option<AuthUser> {

    // Function to convert an `OptionalAuthUser` into an `Option<AuthUser>`.
    fn from(optional_auth_user: OptionalAuthUser) -> Self {
        optional_auth_user.0
    }
}


// Implementation of the `FromRequestParts` trait for `AuthUser`.
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
    where
        S: Send + Sync,
        BeContext: FromRef<S>,
{
    type Rejection = EError;

    // Function to create an `AuthUser` from request parts.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {

        let ctx: BeContext = BeContext::from_ref(state);

        let auth_header = parts
            .headers.get(AUTHORIZATION)
            .ok_or(EError::Unauthorized(String::from(
                "Missing Authorization header",
            )))?;

        Self::from_authorization(&ctx, auth_header)
    }
}


// Implementation of the `FromRequestParts` trait for `OptionalAuthUser`.
#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
    where
        S: Send + Sync,
        BeContext: FromRef<S>,
{
    type Rejection = EError;

    // Function to create an `OptionalAuthUser` from request parts.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx: BeContext = BeContext::from_ref(state);

        Ok(Self(
            parts
                .headers.get(AUTHORIZATION)
                .map(|auth_header| AuthUser::from_authorization(&ctx, auth_header).ok())
                .flatten(),
        ))
    }
}
