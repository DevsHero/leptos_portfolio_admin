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

#[server(CreatePDF, "/api")]
pub async fn pdf_create(profile: Profile) -> Result<String, ServerFnError> {
    let data = generate_pdf(profile).await;
    match data {
        Ok(encode) => Ok(encode),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(ReadPDF, "/api")]
pub async fn get_pdf_file() -> Result<String, ServerFnError> {
    let data = read_pdf_file().await;
    match data {
        Ok(encode) => Ok(encode.expect("REASON")),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(CheckPDF, "/api")]
pub async fn check_pdf_exits() -> Result<bool, ServerFnError> {
    let pdf_file = get_pdf_dir().await;

    Ok(pdf_file?.exists())
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
        use super::database;
        use super::redis::{ update_cache, get_cache };
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
            let data = database::fetch_profile().await?;
            match serde_json::to_string(&data) {
                Ok(data_json) => {
                    let _redis_update = update_cache(
                        PROFILE_CACHE_KEY,
                        &data_json,
                        CACHE_TTL
                    ).await;
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
                    let _redis_update = update_cache(
                        PROFILE_CACHE_KEY,
                        &data_json,
                        CACHE_TTL
                    ).await;
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
                Err(e) => { Ok(None) }
            }
        }
    }
}
