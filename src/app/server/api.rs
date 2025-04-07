use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, SiteConfig };
use std::env;

#[server(GetProfile, "/api")]
pub async fn get_profile_api() -> Result<Profile, ServerFnError> {
    let data = retrieve_profile_api().await;
    match data {
        Ok(Some(profile)) => Ok(profile),
        Ok(None) => Err(ServerFnError::ServerError("No profile found".to_string())),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(Verify, "/api")]
pub async fn verify_password_api(password: String) -> Result<bool, ServerFnError> {
    let verify = verify_password(password).await;
    Ok(verify?)
}

#[server(CreatePDF, "/api")]
pub async fn pdf_create_api(profile: Profile) -> Result<String, ServerFnError> {
    let data = generate_pdf(profile).await;
    match data {
        Ok(encode) => Ok(encode),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(ReadPDF, "/api")]
pub async fn get_pdf_file_api() -> Result<String, ServerFnError> {
    let data = read_pdf_file().await;
    match data {
        Ok(encode) => Ok(encode.expect("REASON")),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(CheckPDF, "/api")]
pub async fn check_pdf_exits_api() -> Result<bool, ServerFnError> {
    let pdf_file = get_pdf_dir().await;
    Ok(pdf_file?.exists())
}
#[server(SiteConfigs, "/api")]
pub async fn site_config_api() -> Result<SiteConfig, ServerFnError> {
    let title = std::env::var("SITE_TITLE").unwrap();
    let config = SiteConfig { title };
    Ok(config)
}

#[server(UpdatePortfolio, "/api")]
pub async fn update_profile_api(
    profile: Profile,
    _is_update_skill: bool,
    _is_update_portfolio: bool,
    _is_update_experience: bool,
    _is_update_language: bool,
    _is_update_education: bool,
    _is_update_contact: bool
) -> Result<bool, ServerFnError> {
    let updated = update_profile(
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
        use super::database;
        use super::redis::{ update_cache, get_cache, check_rate_limit };
        use crate::app::{
            constants::constant::{
                PROFILE_CACHE_KEY,
                CACHE_TTL,
                PDF_FILE_NAME,
                PDF_DIR,
                PDF_FULL_PATH,
            },
        };
        use std::path::PathBuf;
        use std::fs;
        use leptos::{ use_context, logging };
        use base64::{ engine::general_purpose::STANDARD, Engine as _ };

        pub async fn retrieve_profile_api() -> Result<Option<Profile>, ServerFnError> {
            let cached: Option<String> = get_cache(PROFILE_CACHE_KEY).await;
            if let Some(cached_json) = cached {
                match serde_json::from_str::<Option<Profile>>(&cached_json) {
                    Ok(profile_opt) => {
                        println!("Get profile from cache");
                        return Ok(profile_opt);
                    }
                    Err(e) => {
                        eprintln!(
                            "Redis cache deserialization failed for key '{}': {}. Fetching fresh data.",
                            PROFILE_CACHE_KEY,
                            e
                        );
                    }
                }
            } else {
                println!("Cache miss for key: {}", PROFILE_CACHE_KEY);
            }

            // --- Cache Miss or Deserialization Failure ---
            println!("Fetching profile from database.");
            let data = database::server_fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _ = update_cache(PROFILE_CACHE_KEY, &data_json, CACHE_TTL).await;
                }
                Err(e) => {
                    eprintln!("Failed to serialize profile for caching: {}", e); // Log serialization error but continue
                }
            }

            Ok(data)
        }
        pub async fn update_profile(
            profile: Profile,
            _is_update_skill: bool,
            _is_update_portfolio: bool,
            _is_update_experience: bool,
            _is_update_language: bool,
            _is_update_education: bool,
            _is_update_contact: bool
        ) -> Result<bool, ServerFnError> {
            let _ = database::server_update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_language,
                _is_update_education,
                _is_update_contact
            ).await;
            // --- Update Profile Cache---
            let data = database::server_fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _ = update_cache(PROFILE_CACHE_KEY, &data_json, CACHE_TTL).await;
                    let _ = fs::remove_file(PDF_FULL_PATH);
                    Ok(true)
                }
                Err(e) => {
                    Err(ServerFnError::ServerError(format!("Failed to update redis: {}", e)))
                }
            }
        }
        pub async fn generate_pdf(profile: Profile) -> Result<String, ServerFnError> {
            use crate::app::utils::pdf_template::{ generate_html_string, generate_pdf };

            println!("Generate new pdf");
            let html_string = match generate_html_string(&profile) {
                Ok(html) => html,
                Err(e) => {
                    return Err(
                        ServerFnError::ServerError(format!("HTML generation failed: {}", e))
                    );
                }
            };

            let _pdf_bytes_result = generate_pdf(&html_string);
            #[cfg(not(feature = "ssr"))]
            let _pdf_bytes_result: Result<Vec<u8>, String> = Err(
                "PDF generation is only available on the server.".to_string()
            );

            match _pdf_bytes_result {
                Ok(pdf_bytes) => {
                    let encoded_pdf = STANDARD.encode(&pdf_bytes);
                    let _ = store_pdf_file(pdf_bytes.clone()).await;
                    Ok(encoded_pdf)
                }
                Err(e) => { Err(ServerFnError::ServerError(e)) }
            }
        }

        pub async fn get_pdf_dir() -> Result<PathBuf, ServerFnError> {
            let public_dir = PathBuf::from(PDF_DIR);
            let target_file_path = public_dir.join(PDF_FILE_NAME);
            Ok(target_file_path)
        }
        async fn store_pdf_file(data: Vec<u8>) -> Result<bool, ServerFnError> {
            use std::path::PathBuf;
            let public_dir = PathBuf::from(PDF_DIR);
            let target_file_path = public_dir.join(PDF_FILE_NAME);
            use std::fs;
            let _ = fs::create_dir_all(PDF_DIR);
            match fs::write(&target_file_path, &data) {
                Ok(_) => {
                    println!("Store pdf file success");
                    Ok(true)
                }
                Err(_e) => { Ok(false) }
            }
        }
        pub async fn read_pdf_file() -> Result<Option<String>, ServerFnError> {
            let _pdf_bytes_result = fs::read(PDF_FULL_PATH.to_string()); // Use ? for cleaner error propagation
            #[cfg(not(feature = "ssr"))]
            let _pdf_bytes_result: Result<Vec<u8>, String> = Err(
                "PDF generation is only available on the server.".to_string()
            );
            match _pdf_bytes_result {
                Ok(pdf_bytes) => {
                    let encoded_pdf = STANDARD.encode(&pdf_bytes);
                    Ok(Some(encoded_pdf))
                }
                Err(_e) => { Ok(None) }
            }
        }
        pub async fn verify_password(password: String) -> Result<bool, ServerFnError> {
            use actix_web::HttpRequest;
            use argon2::Argon2;
            use password_hash::{ PasswordHash, PasswordVerifier, Error as PwHashError };
            use base64::{ Engine as _, engine::general_purpose };

            let http_req = use_context::<HttpRequest>();
            let ip_address_string = if let Some(req) = http_req {
                req.headers()
                    .get("X-Forwarded-For")
                    .and_then(|h| h.to_str().ok())
                    .and_then(|s| s.split(',').next())
                    .map(|s| s.trim().to_string())
                    .or_else(|| { req.peer_addr().map(|addr| addr.ip().to_string()) })
                    .unwrap_or_else(|| "unknown_ip".to_string())
            } else {
                logging::warn!("HttpRequest not found in context for rate limiting.");
                "unknown_ip_context".to_string()
            };

            let allowed = check_rate_limit("admin_verify", &ip_address_string, 5, 60).await?;
            if !allowed {
                logging::warn!(
                    "Rate limit exceeded for admin verify from IP: {}",
                    ip_address_string
                );
                return Err(
                    ServerFnError::ServerError(
                        "Too many attempts. Please try again later.".to_string()
                    )
                );
            }

            // First try to get the encoded hash (for Docker environments)
            let stored_hash = match std::env::var("ADMIN_PASSWORD_HASH_ENCODED") {
                Ok(encoded) => {
                    // Decode the Base64 encoded hash
                    match general_purpose::STANDARD.decode(encoded) {
                        Ok(decoded_bytes) => {
                            match String::from_utf8(decoded_bytes) {
                                Ok(decoded_hash) => {
                                    logging::log!(
                                        "Using decoded hash from ADMIN_PASSWORD_HASH_ENCODED"
                                    );
                                    decoded_hash
                                }
                                Err(e) => {
                                    logging::error!(
                                        "Error converting decoded hash to string: {}",
                                        e
                                    );
                                    return Err(
                                        ServerFnError::ServerError(
                                            "Server configuration error.".to_string()
                                        )
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            logging::error!("Error decoding Base64 hash: {}", e);
                            return Err(
                                ServerFnError::ServerError(
                                    "Server configuration error.".to_string()
                                )
                            );
                        }
                    }
                }
                Err(_) => {
                    // Fall back to the original hash (for non-Docker environments)
                    match std::env::var("ADMIN_PASSWORD_HASH") {
                        Ok(h) => {
                            logging::log!("Using original ADMIN_PASSWORD_HASH");
                            h
                        }
                        Err(_) => {
                            logging::error!(
                                "Neither ADMIN_PASSWORD_HASH nor ADMIN_PASSWORD_HASH_ENCODED environment variables are set!"
                            );
                            return Err(
                                ServerFnError::ServerError(
                                    "Server configuration error.".to_string()
                                )
                            );
                        }
                    }
                }
            };

            logging::log!("DEBUG: Using hash: '{}'", stored_hash);

            let verify_result = actix_web::rt::task::spawn_blocking(move || {
                let password_bytes = password.as_bytes();

                let parsed_hash = match PasswordHash::new(&stored_hash) {
                    Ok(hash) => {
                        logging::log!("DEBUG: Successfully parsed hash");
                        hash
                    }
                    Err(e) => {
                        logging::error!("FATAL: Stored hash is invalid: {}", e);
                        logging::error!("Hash that failed to parse: '{}'", stored_hash);
                        return Err(PwHashError::Password);
                    }
                };

                // Use explicit parameters matching those used for generation
                let argon2 = Argon2::new(
                    argon2::Algorithm::Argon2id,
                    argon2::Version::V0x13,
                    argon2::Params::new(19456, 2, 1, None).unwrap()
                );

                argon2.verify_password(password_bytes, &parsed_hash)
            }).await;

            let is_correct = match verify_result {
                Ok(Ok(())) => true,
                Ok(Err(e)) => {
                    if matches!(e, PwHashError::Password) {
                        logging::log!(
                            "Incorrect admin password attempt (Argon2 mismatch) from IP {}",
                            ip_address_string
                        );
                    } else {
                        logging::error!("Argon2 verification processing error: {}", e);
                    }
                    false
                }
                Err(e) => {
                    logging::error!("Blocking task join error during Argon2 verification: {}", e);
                    false
                }
            };

            if is_correct {
                logging::log!("Successful admin login from IP {}", ip_address_string);
            }

            Ok(is_correct)
        }
    }
}
