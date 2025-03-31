use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, SiteConfig, Skill };
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
    use crate::app::utils::pdf_template::{ generate_html_string, generate_pdf_with_html2pdf };
    use leptos::logging;
    let html_string = match generate_html_string(&profile) {
        Ok(html) => html,
        Err(e) => {
            logging::error!("Failed to generate HTML string: {}", e);
            return Err(ServerFnError::ServerError(format!("HTML generation failed: {}", e)));
        }
    };
    #[cfg(feature = "ssr")]
    let _pdf_bytes_result = generate_pdf_with_html2pdf(&html_string);
    #[cfg(not(feature = "ssr"))]
    let _pdf_bytes_result: Result<Vec<u8>, String> = Err(
        "PDF generation is only available on the server.".to_string()
    );

    // --- Process result ---
    match _pdf_bytes_result {
        Ok(pdf_bytes) => {
            logging::log!(
                "PDF generated successfully via html2pdf ({} bytes), encoding...",
                pdf_bytes.len()
            );
            // Encode to Base64
            let encoded_pdf = STANDARD.encode(&pdf_bytes);
            Ok(encoded_pdf)
        }
        Err(e) => {
            logging::error!("html2pdf PDF generation failed: {}", e);
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
) -> Result<Option<Profile>, ServerFnError> {
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
        Ok(updated_result) => Ok(updated_result),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(UpdateSkill, "/api")]
pub async fn update_skill(_skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
    let updated = update_skill_api(_skills).await;
    match updated {
        Ok(updated_result) => Ok(updated_result),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::server::database;
        pub async fn retrieve_profile_api() -> Result<Option<Profile>, ServerFnError> {
            database::fetch_profile().await
        }

        pub async fn update_portfolio_api(
            profile: Profile,
            _is_update_skill: bool,
            _is_update_portfolio: bool,
            _is_update_experience: bool,
            _is_update_language: bool,
            _is_update_education: bool,
            _is_update_contact: bool
        ) -> Result<Option<Profile>, ServerFnError> {
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
        pub async fn update_skill_api(_skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            database::update_skill(_skills).await
        }
    }
}
