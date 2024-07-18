# wForum - Backend

An online forum backend using axum and prisma.

## Prerequisites

- Rust / Docker / PostgreSQL

## Functionality

1. User
   - Get current  / Login / Update / Delete user.
     - Avatar / Email / Username / Intro.
2. Profile
   - Follow / Unfollow / Block / Unblock profile.
3. Post
   - Filter posts base on post id / author / liked / followers.
   - Create / Update / Delete / Like / Unlike posts.
   - Get / Create / Delete comment on posts.

## Starting the backend

- Copy `.env.example` to `.env` and configure it.

1. `just push` : Sync database with the Prisma schema.
2. `just generate` : Generate the Prisma Client.
3. `just run`: Run the application using `cargo run`.
4. `just watch`: Use `cargo watch` to automatically reload the application on file changes.

## API Documentation

- wForum.postman.json ( export from postman)
- Openapi.json ( automatic conversion )
- Openapi.yaml  ( automatic conversion )

## Router Overview

```
        axum::Router::new()
            .route("/", axum::routing::get("Hello Rust!"))
            // user service
            .route("/users", get(UsersService::fetch_user))
            .route("/users", post(UsersService::login_user))
            .route("/users", put(UsersService::update_user))
            .route("/users/create", post(UsersService::create_user))
            // profile service
            .route("/profiles/:username", get(ProfilesService::fetch_profile))
            .route("/profiles/:username/follow", post(ProfilesService::follow_profile))
            .route("/profiles/:username/follow", delete(ProfilesService::unfollow_profile))
            .route("/profiles/:username/block", post(ProfilesService::block_profile))
            .route("/profiles/:username/block", delete(ProfilesService::unblock_profile))
            // post service
            .route("/posts", get(PostService::fetch_posts))
            .route("/posts", post(PostService::create_post))
            .route("/posts/:post_id", get(PostService::fetch_post))
            .route("/posts/:post_id", put(PostService::update_post))
            .route("/posts/:post_id", delete(PostService::delete_post))
            .route("/posts/:post_id/like", post(PostService::like_post))
            .route("/posts/:post_id/like", delete(PostService::unlike_post))
            .route("/posts/:post_id/comments", post(PostService::create_comment))
            .route("/posts/:post_id/comments", get(PostService::get_comments))
            .route("/posts/:post_id/comments/:comment_id", delete(PostService::delete_comment))
```

## Reference

Realworld project
