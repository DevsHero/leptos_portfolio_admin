use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::env;
        use std::sync::OnceLock;
        use redis::Client as RedisClient;
        use redis::AsyncCommands;
        use leptos::ServerFnError;

        static REDIS_CLIENT: OnceLock<RedisClient> = OnceLock::new();
        fn init_redis() -> RedisClient {
            let redis_url = if cfg!(debug_assertions) {
                env::var("REDIS_URL_DEV").unwrap_or_else(|_| "redis://localhost:6379".to_string())
            } else {
                env::var("REDIS_URL_PROD").unwrap_or_else(|_| "redis://redis:6379".to_string())
            };

            println!("Attempting to connect to Redis at: {}", redis_url);
            RedisClient::open(redis_url).expect("Failed to create Redis client")
        }
        pub fn get_redis_client() -> &'static RedisClient {
            REDIS_CLIENT.get_or_init(init_redis)
        }
        pub async fn update_cache(
            key: &str,
            data: &String,
            ttl: u64
        ) -> Result<bool, ServerFnError> {
            let client = get_redis_client();
            let mut redis_client = client
                .get_multiplexed_async_connection().await
                .map_err(|e| -> ServerFnError {
                    eprintln!("Redis connection failed: {}", e);
                    ServerFnError::ServerError(format!("Redis connection failed: {}", e))
                })?;

            println!("Updating redis cache");
            let set_result: Result<(), redis::RedisError> = redis_client.set_ex::<
                &str,
                &String,
                ()
            >(key, &data, ttl).await;

            if let Err(e) = set_result {
                eprintln!("Redis SET failed for key '{}': {}", key, e);
            }
            Ok(true)
        }
    }
}
