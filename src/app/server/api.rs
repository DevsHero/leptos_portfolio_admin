use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, Skill };
#[server(GetProfile, "/api")]
pub async fn get_profile() -> Result<Vec<Profile>, ServerFnError> {
    let data = retrieve_profile_api().await;
    // println!("Query result: {:?}", data);
    match data {
        Ok(result) => Ok(result.into_iter().collect()),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
// #[server(GetSkill, "/api")]
// pub async fn get_skill() -> Result<Vec<Skill>, ServerFnError> {
//     let data = retrieve_skill_api().await;
//     println!("Query result: {:?}",data);
//     match data {
//         Ok(result) => Ok(result.into_iter().collect()),
//         Err(e) => Err(ServerFnError::from(e)),
//     }
// }
#[server(UpdatePortfolio, "/api")]
pub async fn update_portfolio(
    profile: Profile,
    is_update_skill: bool,
    skills: Vec<Skill>
) -> Result<Option<Profile>, ServerFnError> {
    let updated = update_portfolio_api(profile).await;
    match updated {
        Ok(updated_result) => Ok(updated_result),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(UpdateSkill, "/api")]
pub async fn update_skill(skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
    let updated = update_skill_api(skills).await;
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
        pub async fn retrieve_skill_api() -> Result<Option<Skill>, ServerFnError> {
            database::fetch_skill().await
        }
        pub async fn update_portfolio_api(
            profile: Profile
        ) -> Result<Option<Profile>, ServerFnError> {
            database::update_profile(profile).await
        }
        pub async fn update_skill_api(skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            database::update_skill(skills).await
        }
    }
}
