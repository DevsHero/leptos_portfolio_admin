use serde::{ Deserialize, Serialize };
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct SiteConfig {
    pub title: String,
}
impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: String::from("Fullstack rust ssr site"),
        }
    }
}
