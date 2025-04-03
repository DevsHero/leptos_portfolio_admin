cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::env;
        use redis::AsyncCommands;
        use std::sync::OnceLock;
        use leptos::ServerFnError;
        use crate::app::models::Profile; // Assuming Profile is defined here
        use crate::app::server::database; // Assuming database::fetch_profile exists

        // Static OnceLock to hold the initialized Redis client
        static REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();

        /// Initializes the Redis client using environment variables.
        /// Panics if the client cannot be created.
        pub fn init_redis() -> redis::Client {
            let redis_url = env
                ::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into());
            println!("Attempting to connect to Redis at: {}", redis_url); // Added for debugging
            redis::Client::open(redis_url).expect("Failed to create Redis client") // Panics on failure
        }
    }
}
