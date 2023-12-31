// The `config` module.
pub mod config;

// Importing the `BeConfig` struct from the `config` module.
use config::BeConfig;
// Importing the `lazy_static` macro for single initialization of global variables.
use lazy_static::lazy_static;


// The `BeContext` struct which contains a thread-safe reference-counted `BeConfig`.
#[derive(Clone)]
pub struct BeContext {
    // The `config` field is an `Arc` (Atomic Reference Count) which ensures thread safety.
    pub config: std::sync::Arc<BeConfig>,
}


// Using the `lazy_static` macro to initialize global variables only once.
lazy_static! {
    // The `CONFIG` variable is a global instance of `BeConfig` initialized once.
    pub static ref CONFIG: BeConfig = BeConfig::init();
    // The `CONTEXT` variable is a global instance of `BeContext` initialized once.
    // It contains a thread-safe reference-counted `BeConfig`.
    pub static ref CONTEXT: BeContext = BeContext {
        // Ensure safe concurrency by wrapping `CONFIG` in an `Arc`.
        config: std::sync::Arc::new(CONFIG.clone()),
    };
}