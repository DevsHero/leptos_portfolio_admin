use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::env;
        use std::sync::OnceLock;
        use redis::Client as RedisClient;
        use redis::AsyncCommands;
        use leptos::{ ServerFnError, logging };
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

        pub async fn get_cache(key: &str) -> Option<String> {
            let client = REDIS_CLIENT.get_or_init(init_redis);
            let mut redis_client = match client.get_multiplexed_async_connection().await {
                Ok(conn) => conn,
                Err(_e) => {
                    return None;
                }
            };
            match redis_client.get::<&str, Option<String>>(key).await {
                Ok(Some(value)) => { Some(value) }
                Ok(None) => { None }
                Err(_e) => { None }
            }
        }
        pub async fn update_cache(
            key: &str,
            data: &String,
            ttl: u64
        ) -> Result<bool, ServerFnError> {
            let client = REDIS_CLIENT.get_or_init(init_redis);
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
        pub async fn check_rate_limit(
            action_key: &str, // e.g., "admin_login"
            identifier: &str, // e.g., IP address
            limit: usize, // Max attempts allowed
            window_secs: i64 // Time window in seconds
        ) -> Result<bool, ServerFnError> {
            let client = REDIS_CLIENT.get_or_init(init_redis);
            let mut conn = client.get_multiplexed_async_connection().await?;
            let key = format!("rate_limit:{}:{}", action_key, identifier);
            let count: isize = conn.incr(&key, 1).await?;
            if count == 1 {
                let _: () = conn.expire(&key, window_secs).await?;
                logging::log!("Set expiration for key '{}' to {} seconds", key, window_secs);
            }
            if (count as usize) > limit {
                logging::log!(
                    "Rate limit exceeded for key '{}'. Count: {}, Limit: {}",
                    key,
                    count,
                    limit
                );
                Ok(false) // Rate limited
            } else {
                Ok(true) // Allowed
            }
        }
    }
}
