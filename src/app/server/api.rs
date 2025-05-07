use leptos::{ server, ServerFnError };
use crate::app::models::{ server::WSSignedConfig, Profile, SiteConfig, Verification };
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
pub async fn verify_password_api(password: String) -> Result<Verification, ServerFnError> {
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
 
 
#[server(GetWSSignedConfig, "/api")]
pub async fn get_ws_signed_config_api() -> Result<WSSignedConfig, ServerFnError> {
   let config: WSSignedConfig = ws_signing_key().await?;
  

    Ok(config)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use super::database;
        use super::redis::{ update_cache, get_cache, check_rate_limit };
        use chrono::Utc;
        use hmac::{ Hmac, Mac };
        use sha2::Sha256;
        use hex;
        use crate::app::{
            constants::constant::{
                PROFILE_CACHE_KEY,
                CACHE_TTL,
                PDF_FILE_NAME,
                PDF_DIR,
                PDF_FULL_PATH,
            },
        };
        use rand_core::{ RngCore, OsRng };
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

            println!("Fetching profile from database.");
            let data = database::server_fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _ = update_cache(PROFILE_CACHE_KEY, &data_json, CACHE_TTL).await;
                }
                Err(e) => {
                    eprintln!("Failed to serialize profile for caching: {}", e);  
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
            let _pdf_bytes_result = fs::read(PDF_FULL_PATH.to_string()); 
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
        pub async fn verify_password(password: String) -> Result<Verification, ServerFnError> {
            use actix_web::{ HttpRequest, rt::time };
            use argon2::Argon2;
            use password_hash::{ PasswordHash, PasswordVerifier };
            use std::time::{ Duration, Instant };
            use subtle::ConstantTimeEq;

            let start = Instant::now();
            let ip_address_string = {
                let http_req = use_context::<HttpRequest>();
                if let Some(req) = http_req {
                    req.headers()
                        .get("X-Forwarded-For")
                        .and_then(|h| h.to_str().ok())
                        .and_then(|s| s.split(',').next())
                        .map(|s| s.trim().to_string())
                        .or_else(|| req.peer_addr().map(|addr| addr.ip().to_string()))
                        .unwrap_or_else(|| "unknown_ip".to_string())
                } else {
                    logging::warn!("HttpRequest not found in context for rate limiting.");
                    "unknown_ip_context".to_string()
                }
            };

            let allowed = check_rate_limit("admin_verify", &ip_address_string, 5, 300).await?;
            let is_rate_limited = (!allowed as u8).ct_eq(&1u8).into();

            if is_rate_limited {
                let sleep_time = get_random_in_range(50, 150);
                time::sleep(Duration::from_millis(sleep_time)).await;

                logging::warn!(
                    "Rate limit exceeded for admin verify from IP: {}",
                    ip_address_string
                );
                return Ok(Verification {
                    verify: false,
                    restrict: true,
                });
            }

             let stored_hash = match get_stored_hash().await {
                Ok(hash) => hash,
                Err(e) => {
                    time::sleep(Duration::from_millis(get_random_in_range(100, 200))).await;
                    return Err(e);
                }
            };

             let verify_result = actix_web::rt::task::spawn_blocking(move || {
                let parsed_hash = match PasswordHash::new(&stored_hash) {
                    Ok(hash) => hash,
                    Err(e) => {
                        logging::error!("FATAL: Stored hash is invalid: {}", e);
                        logging::error!("Hash that failed to parse: '{}'", stored_hash);
                        return 0u8;
                    }
                };

                let argon2 = Argon2::new(
                    argon2::Algorithm::Argon2id,
                    argon2::Version::V0x13,
                    argon2::Params::new(19456, 2, 1, None).unwrap()
                );
                match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(()) => 1u8,
                    Err(_) => 0u8,
                }
            }).await;

            let verification_byte = match verify_result {
                Ok(result) => result,
                Err(e) => {
                    logging::error!("Verification task error: {}", e);
                    0u8
                }
            };

            let is_correct = verification_byte.ct_eq(&1u8).into();
            if is_correct {
                logging::log!("Successful admin login from IP {}", ip_address_string);
            } else {
                logging::log!("Failed admin login attempt from IP {}", ip_address_string);
            }

            let base_delay = 100;
            let jitter = get_random_in_range(20, 80);
            let elapsed = start.elapsed().as_millis() as u64;
            let target_min_time = base_delay + jitter;

            if elapsed < target_min_time {
                time::sleep(Duration::from_millis(target_min_time - elapsed)).await;
            }

            Ok(Verification {
                verify: is_correct,
                restrict: false,
            })
        }

        fn get_random_in_range(min: u64, max: u64) -> u64 {
            let mut rng = OsRng;
            let range = max - min;
            let mut buf = [0u8; 8];
            rng.fill_bytes(&mut buf);
            let random_u64 = u64::from_ne_bytes(buf);
            min + (random_u64 % (range + 1))
        }

        async fn get_stored_hash() -> Result<String, ServerFnError> {
            use base64::{ Engine as _, engine::general_purpose };
            let _encoded_hash = match std::env::var("ADMIN_PASSWORD_HASH_ENCODED") {
                Ok(encoded) => {
                    match general_purpose::STANDARD.decode(encoded) {
                        Ok(decoded_bytes) => {
                            match String::from_utf8(decoded_bytes) {
                                Ok(decoded_hash) => {
                                    logging::log!(
                                        "Using decoded hash from ADMIN_PASSWORD_HASH_ENCODED"
                                    );
                                    return Ok(decoded_hash);
                                }
                                Err(e) => {
                                    logging::error!(
                                        "Error converting decoded hash to string: {}",
                                        e
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            logging::error!("Error decoding Base64 hash: {}", e);
                        }
                    }
                }
                Err(_) => {}
            };

            if let Ok(hash) = std::env::var("ADMIN_PASSWORD_HASH") {
                logging::log!("Using original ADMIN_PASSWORD_HASH");
                return Ok(hash);
            }

            Err(ServerFnError::ServerError("Server configuration error.".to_string()))
        }
        pub async fn ws_signing_key( ) -> Result<WSSignedConfig, ServerFnError> {
            let host = std::env::var("WS_HOST")
                .ok().filter(|h| !h.is_empty())
                .unwrap_or_else(|| {
                    logging::warn!("WS_HOST not set, defaulting to ws://localhost:4000");
                    "ws://localhost:4000".to_string()
                });
            let secret = std::env::var("WS_API_KEY")
                .unwrap_or_else(|_| "".to_string());

            let ts = Utc::now().timestamp().to_string();

            let sig = {
                let mut mac: Hmac<Sha256> =
                 Hmac::new_from_slice(secret.as_bytes()).expect("HMAC valid key");
                mac.update(ts.as_bytes());
                hex::encode(mac.finalize().into_bytes())
            };

            Ok(WSSignedConfig {
                host,
                ts,
                sig,
            })
        }
    }
   
}
