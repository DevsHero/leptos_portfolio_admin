use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, SiteConfig };
use std::env;

#[server(GetProfile, "/api")]
pub async fn get_profile() -> Result<Profile, ServerFnError> {
    let data = retrieve_profile_api().await;
    match data {
        Ok(Some(profile)) => Ok(profile),
        Ok(None) => Err(ServerFnError::ServerError("No profile found".to_string())),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(Verify, "/api")]
pub async fn verify(password: String) -> Result<bool, ServerFnError> {
    let admin_password = std::env::var("ADMIN_MODE_PASSWORD").unwrap_or("admin".to_string());
    Ok(admin_password == password)
}

#[server(PdfExport, "/api")]
pub async fn pdf_export(profile: Profile) -> Result<String, ServerFnError> {
    use base64::{ engine::general_purpose::STANDARD, Engine as _ };
    use crate::app::utils::pdf_template::{ generate_html_string, generate_pdf };
    use leptos::logging;
    let html_string = match generate_html_string(&profile) {
        Ok(html) => html,
        Err(e) => {
            logging::error!("Failed to generate HTML string: {}", e);
            return Err(ServerFnError::ServerError(format!("HTML generation failed: {}", e)));
        }
    };
    #[cfg(feature = "ssr")]
    let _pdf_bytes_result = generate_pdf(&html_string);
    #[cfg(not(feature = "ssr"))]
    let _pdf_bytes_result: Result<Vec<u8>, String> = Err(
        "PDF generation is only available on the server.".to_string()
    );

    // --- Process result ---
    match _pdf_bytes_result {
        Ok(pdf_bytes) => {
            logging::log!("PDF generated successfully  ({} bytes), encoding...", pdf_bytes.len());
            // Encode to Base64
            let encoded_pdf = STANDARD.encode(&pdf_bytes);
            Ok(encoded_pdf)
        }
        Err(e) => {
            logging::error!("generate pdf failed: {}", e);
            Err(ServerFnError::ServerError(e))
        }
    }
}
#[server(SiteConfigs, "/api")]
pub async fn site_config() -> Result<SiteConfig, ServerFnError> {
    let title = std::env::var("SITE_TITLE").unwrap();
    let config = SiteConfig { title };
    Ok(config)
}

#[server(UpdatePortfolio, "/api")]
pub async fn update_portfolio(
    profile: Profile,
    _is_update_skill: bool,
    _is_update_portfolio: bool,
    _is_update_experience: bool,
    _is_update_language: bool,
    _is_update_education: bool,
    _is_update_contact: bool
) -> Result<bool, ServerFnError> {
    let updated = update_portfolio_api(
        profile,
        _is_update_skill,
        _is_update_portfolio,
        _is_update_experience,
        _is_update_language,
        _is_update_education,
        _is_update_contact
    ).await;

    match updated {
        Ok(_updated_result) => Ok(_updated_result),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use redis::AsyncCommands;
        use std::sync::OnceLock;
        use crate::app::server::database;
        static REDIS_CLIENT: OnceLock<redis::Client> = OnceLock::new();

        fn init_redis() -> redis::Client {
            let redis_url = env
                ::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".into());
            println!("Attempting to connect to Redis at: {}", redis_url);
            redis::Client::open(redis_url).expect("Failed to create Redis client")
        }

        pub async fn retrieve_profile_api() -> Result<Option<Profile>, ServerFnError> {
            let client = REDIS_CLIENT.get_or_init(init_redis);
            let mut conn = client
                .get_multiplexed_async_connection().await
                .map_err(|e| -> ServerFnError {
                    ServerFnError::ServerError(format!("Redis connection failed: {}", e))
                })?;

            const CACHE_KEY: &str = "profile";
            const CACHE_TTL: usize = 3600;

            // Try to get cached profile
            let cached: Option<String> = conn.get(CACHE_KEY).await.map_err(|e| -> ServerFnError {
                eprintln!("Redis GET failed: {}. Proceeding as cache miss.", e);
                ServerFnError::ServerError(format!("Redis GET failed: {}", e))
            })?;

            if let Some(cached_json) = cached {
                // Attempt to deserialize from cache
                match serde_json::from_str::<Option<Profile>>(&cached_json) {
                    Ok(profile_opt) => {
                        println!("Fetching profile from cache"); // Debug log
                        return Ok(profile_opt); // Return deserialized profile (or None if stored as None)
                    }
                    Err(e) => {
                        // Log deserialization error, maybe invalidate cache, then proceed to fetch
                        eprintln!(
                            "Redis cache deserialization failed for key '{}': {}. Fetching fresh data.",
                            CACHE_KEY,
                            e
                        );
                    }
                }
            } else {
                println!("Cache miss for key: {}", CACHE_KEY); // Debug log
            }

            // --- Cache Miss or Deserialization Failure ---

            // Fetch from database
            println!("Fetching profile from database."); // Debug log
            let profile_opt = database::fetch_profile().await?;
            match serde_json::to_string(&profile_opt) {
                Ok(profile_opt_json) => {
                    println!("Setting cache for key: {} with TTL: {}", CACHE_KEY, CACHE_TTL); // Debug log
                    let set_result: Result<(), redis::RedisError> = conn.set_ex::<
                        &str,
                        &String,
                        ()
                    >(CACHE_KEY, &profile_opt_json, CACHE_TTL as u64).await;
                    if let Err(e) = set_result {
                        eprintln!("Redis SET failed for key '{}': {}", CACHE_KEY, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize profile for caching: {}", e);
                }
            }

            Ok(profile_opt) // Return the fetched profile (or None)
        }
        pub async fn update_portfolio_api(
            profile: Profile,
            _is_update_skill: bool,
            _is_update_portfolio: bool,
            _is_update_experience: bool,
            _is_update_language: bool,
            _is_update_education: bool,
            _is_update_contact: bool
        ) -> Result<bool, ServerFnError> {
            database::update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_language,
                _is_update_education,
                _is_update_contact
            ).await
        }
    }
}
