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
    let data = generate_pdf(profile).await;
    match data {
        Ok(pdf_bytes) => Ok(pdf_bytes),

        Err(e) => Err(ServerFnError::from(e)),
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
        use super::database;
        use super::redis::{ get_redis_client, update_cache };
        const CACHE_KEY: &str = "profile";
        const CACHE_TTL: u64 = 2592000;

        pub async fn retrieve_profile_api() -> Result<Option<Profile>, ServerFnError> {
            let client = get_redis_client();
            let mut redis_client = client
                .get_multiplexed_async_connection().await
                .map_err(|e| -> ServerFnError {
                    eprintln!("Redis connection failed: {}", e); // Log error
                    ServerFnError::ServerError(format!("Redis connection failed: {}", e))
                })?;

            // Try to get cached profile
            let cached: Option<String> = redis_client
                .get(CACHE_KEY).await
                .map_err(|e| -> ServerFnError {
                    eprintln!(
                        "Redis GET failed for key '{}': {}. Proceeding as cache miss.",
                        CACHE_KEY,
                        e
                    );

                    ServerFnError::ServerError(format!("Redis GET failed: {}", e))
                })?;

            if let Some(cached_json) = cached {
                match serde_json::from_str::<Option<Profile>>(&cached_json) {
                    Ok(profile_opt) => {
                        println!("Cache hit for key: {}", CACHE_KEY);
                        return Ok(profile_opt);
                    }
                    Err(e) => {
                        eprintln!(
                            "Redis cache deserialization failed for key '{}': {}. Fetching fresh data.",
                            CACHE_KEY,
                            e
                        );
                    }
                }
            } else {
                println!("Cache miss for key: {}", CACHE_KEY);
            }

            // --- Cache Miss or Deserialization Failure ---
            println!("Fetching profile from database.");
            let data = database::fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _redis_update = update_cache(CACHE_KEY, &data_json, CACHE_TTL).await;
                }
                Err(e) => {
                    eprintln!("Failed to serialize profile for caching: {}", e); // Log serialization error but continue
                }
            }

            Ok(data)
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
            let _ = database::update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_language,
                _is_update_education,
                _is_update_contact
            ).await;
            // --- Update Profile Cache---
            let data = database::fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _redis_update = update_cache(CACHE_KEY, &data_json, CACHE_TTL).await;
                    Ok(true)
                }
                Err(e) => {
                    Err(ServerFnError::ServerError(format!("Failed to update redis: {}", e)))
                }
            }
        }
        pub async fn generate_pdf(profile: Profile) -> Result<String, ServerFnError> {
            use base64::{ engine::general_purpose::STANDARD, Engine as _ };
            use crate::app::utils::pdf_template::{ generate_html_string, generate_pdf };
            use leptos::logging;
            let html_string = match generate_html_string(&profile) {
                Ok(html) => html,
                Err(e) => {
                    logging::error!("Failed to generate HTML string: {}", e);
                    return Err(
                        ServerFnError::ServerError(format!("HTML generation failed: {}", e))
                    );
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
                    logging::log!(
                        "PDF generated successfully  ({} bytes), encoding...",
                        pdf_bytes.len()
                    );
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
    }
}
