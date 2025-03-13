use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, SiteConfig, Skill };
use std::env;

#[server(GetProfile, "/api")]
pub async fn get_profile() -> Result<Vec<Profile>, ServerFnError> {
    let data = retrieve_profile_api().await;
    // println!("Query result: {:?}", data);
    match data {
        Ok(result) => Ok(result.into_iter().collect()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}

#[server(Verify, "/api")]
pub async fn verify(password: String) -> Result<bool, ServerFnError> {
    let admin_password = std::env::var("ADMIN_MODE_PASSWORD").unwrap_or("admin".to_string());
    Ok(admin_password == password)
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
    _is_update_contact: bool
) -> Result<Option<Profile>, ServerFnError> {
    let updated = update_portfolio_api(
        profile,
        _is_update_skill,
        _is_update_portfolio,
        _is_update_experience,
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
            _is_update_contact: bool
        ) -> Result<Option<Profile>, ServerFnError> {
            database::update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_contact
            ).await
        }
        pub async fn update_skill_api(_skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            database::update_skill(_skills).await
        }
    }
}
