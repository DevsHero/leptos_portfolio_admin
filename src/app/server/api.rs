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
    use gotenberg_pdf::{ Client, WebOptions, PaperFormat, LinearDimention, Unit };
    use crate::app::utils::pdf_template::generate_html_string;
    use leptos::logging;
    let html_string = match generate_html_string(&profile) {
        Ok(html) => html,
        Err(e) => {
            logging::error!("Failed to generate HTML string: {}", e);
            return Err(ServerFnError::ServerError(format!("HTML generation failed: {}", e)));
        }
    };

    let gotenberg_url = std::env
        ::var("GOTENBERG_URL")
        .unwrap_or_else(|_| "http://localhost:4000".to_string());
    logging::log!("Using Gotenberg URL: {}", gotenberg_url);
    let client = Client::new(&gotenberg_url);

    let mut options = WebOptions::default();
    options.set_paper_format(PaperFormat::A4);

    options.margin_bottom = Some(LinearDimention::new(0.5, Unit::Cm));
    options.margin_top = Some(LinearDimention::new(0.5, Unit::Cm));
    options.margin_right = Some(LinearDimention::new(0.0, Unit::Cm));
    options.margin_left = Some(LinearDimention::new(0.0, Unit::Cm));

    logging::log!("Sending HTML to Gotenberg...");

    // --- Call Gotenberg ---
    // Pass a slice (&) of the String to pdf_from_html
    let pdf_bytes = match client.pdf_from_html(&html_string, options).await {
        //                                         ^ Use &html_string here
        Ok(bytes) => bytes,
        Err(e) => {
            logging::error!("Gotenberg PDF generation failed: {}", e);
            return Err(ServerFnError::ServerError(format!("PDF generation failed: {}", e)));
        }
    };

    logging::log!("PDF generated successfully ({} bytes), encoding to base64...", pdf_bytes.len());

    let encoded_pdf = STANDARD.encode(&pdf_bytes);
    Ok(encoded_pdf)
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
