# Ryoxhi - Backend - Axum
An online forum backend using axum and prisma.

## API Documentation

View files in the folder `api_document`:

- Ryoxhi.postman.json
- Openapi.json ( Auto generated )
- Openapi.yaml  ( Auto generated )

## Functionality

1. Post
   - Fetch posts
     - Base on author.
     - Base on likes.
     - Base on followers.
   - Fetch specific post base on `post_id`
   - Like posts
   - Unlike posts
   - Comment on posts
     - Create comment.
     - Delete comment.
2. User
   - Create users
     - username
     - email
     - avatar
     - intro
     - avatar
3. Profile
   - Follow profile
   - Unfollow profile
   - Block profile
   - Unblock profile

## Preparation
1. Install prisma client
   ```bash
    cd prisma
    cargo run
   ```
2. Prepare database
   ```bash
    docker-compose up -d
    cargo run db push
   ```
3. Generate prisma client
   ```bash
    cd prisma
    cargo run generate
   ```
4. Config .env
   - Copy .env.example to .env
     - ```bash
         cp .env.example .env
         cp /prisma/.env.example /prisma/.env
         ```
   - Config .env
5. Build and develop
```bash
cargo run
```

## Config .env
__Note:__ You can use .env.example as a template. All variant explained in .env.example

__Note:__ You must sync database url in both .env and /prisma/.env 

- RUST_LOG="\<your config\>"
  - Minimum log level
  - RUST_LOG: `"debug", "info", "warn", "error", "trace"`
- BACKEND_PORT=\<your config\>
  - Port of backend
  - BACKEND_PORT: 0 ~ 65535
- TZ_EAST_OFFSET_IN_HOURS=\<your config\>
  - Timezone __east__ offset 
  - TZ_EAST_OFFSET_IN_HOURS: -12 ~ 12
- JWT_SECRET=\<your config\>
  - Secret of jwt
- JWT_EXPIRATION_VALUE=\<your config\>
  - Expiration value of jwt
- JWT_EXPIRATION_UNIT=\<your config\>
  - Expiration unit of jwt
  - JWT_EXPIRATION_UNIT: `"seconds", "minutes", "hours", "days", "weeks", "months", "years"`
- DATABASE_URL=\<your config\>
  - Database url
  - Example: `postgresql://<username>:<password>@<host>:<port>/<database>?options=-c%20TimeZone%3D<your_timezone>`

## Router

```
        axum::Router::new()
            .route("/", axum::routing::get("Hello Rust!"))
            // user
            .route("/users", get(UsersService::fetch_user))
            .route("/users", post(UsersService::login_user))
            .route("/users", put(UsersService::update_user))
            .route("/users/create", post(UsersService::create_user))
            // profile
            .route("/profiles/:username", get(ProfilesService::fetch_profile))
            .route("/profiles/:username/follow", post(ProfilesService::follow_profile))
            .route("/profiles/:username/follow", delete(ProfilesService::unfollow_profile))
            .route("/profiles/:username/block", post(ProfilesService::block_profile))
            .route("/profiles/:username/block", delete(ProfilesService::unblock_profile))
            // post
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
