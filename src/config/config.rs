// Importing the necessary modules and functions.
use std::env;
use dotenv::dotenv;

use crate::service::utils::helper::Helper;


// The `BeConfig` struct which contains the configuration for the backend.
#[derive(Debug, Clone)]
pub struct BeConfig {
    // The log level for the application.
    pub log_level: String,
    // The port on which the backend will run.
    pub backend_port: u16,
    // The timezone offset in hours.
    pub tz_east_offset_in_hours: i32,
    // The configuration for JWT.
    pub jwt_config: JwtConfig,
    // The URL for the database.
    pub database_url: String,
}

// The `JwtConfig` struct which contains the configuration for JWT.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    // The secret key for JWT.
    pub jwt_secret: String,
    // The expiration value for JWT.
    pub jwt_exp_value: i64,
}


// Implementation of the `BeConfig` struct.
impl BeConfig {
    // Function to initialize the `BeConfig` struct.
    pub fn init() -> Self {
        Self {
            // Get the log level from the environment variable or default to "info".
            log_level: get_env("RUST_LOG"),
            // Get the backend port from the environment variable.
            backend_port: get_env("BACKEND_PORT").parse().unwrap(),
            // Get the timezone offset from the environment variable.
            tz_east_offset_in_hours: get_env("TZ_EAST_OFFSET_IN_HOURS").parse().unwrap(),
            // Initialize the `JwtConfig` struct.
            jwt_config: JwtConfig {
                // Get the JWT secret from the environment variable.
                jwt_secret: get_env("JWT_SECRET"),
                // Get the JWT expiration value from the environment variable and convert it to seconds.
                jwt_exp_value: Helper::value_to_seconds(
                    get_env("JWT_EXPIRATION_VALUE").parse().unwrap(),
                    get_env("JWT_EXPIRATION_UNIT")
                )
            },
            // Get the database URL from the environment variable.
            database_url: get_env("DATABASE_URL"),
        }
    }
}


// Function to get the value of an environment variable.
pub fn get_env(key: &str) -> String {
    // Load the environment variables from the .env file.
    dotenv().ok();
    // Get the value of the environment variable or panic if it is not set.
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}