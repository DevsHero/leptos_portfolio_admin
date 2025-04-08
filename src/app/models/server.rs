use serde::{ Deserialize, Serialize };
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::models::profile::{
            Experience,
            Portfolio,
            Skill,
            Contact,
            PDF,
            Education,
            Language,
        };
        use surrealdb::sql::Thing;
        #[derive(Debug, Deserialize)]
        pub struct ThingProfile {
            pub first_name: String,
            pub last_name: String,
            pub nick_name: String,
            pub gender: String,
            pub birth_date: String,
            pub role: String,
            pub nationality: String,
            pub about: String,
            pub avatar: String,
            pub address: String,
            pub pdf: PDF,
            pub id: Thing, // Using Thing instead of String
            pub skills: Option<Vec<Skill>>,
            pub experiences: Option<Vec<Experience>>,
            pub portfolios: Option<Vec<Portfolio>>,
            pub contacts: Option<Vec<Contact>>,
            pub educations: Option<Vec<Education>>,
            pub languages: Option<Vec<Language>>,
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Verification {
    pub verify: bool,
    pub restrict: bool,
}
