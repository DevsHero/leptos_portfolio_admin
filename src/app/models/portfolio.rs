use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Skill {
    pub name: String,
    pub level: String,
}
impl Default for Skill {
    fn default() -> Self {
        Self {
            name: String::from("Javascript"),
            level: String::from("Expert"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Contact {
    pub contact_icon: String,
    pub is_href: bool,
    pub contact_value: String,
}
impl Default for Contact {
    fn default() -> Self {
        Self {
            contact_icon: String::from("MdiEmail"),
            is_href: bool::from(true),
            contact_value: String::from("developer@portfolio.com"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Portfolio {
    pub portfolio_name: String,
    pub portfolio_link: String,
    pub is_private: bool,
    pub portfolio_detail: String,
    pub portfolio_icon_url: String,
    pub stacks: Vec<String>,
    pub screenshots_url: Vec<String>,
}
impl Default for Portfolio {
    fn default() -> Self {
        Self {
            portfolio_name: String::from("Leptos Portfolio Admin"),
            portfolio_link: String::from("https://github.com/zelda2003/leptos_portfolio_admin"),
            is_private: bool::from(false),
            portfolio_detail: String::from("Fullstack rust portfolio project with admin system"),
            portfolio_icon_url: String::from("https://cdn-icons-png.flaticon.com/512/25/25231.png"),
            screenshots_url: vec![
                String::from(
                    "https://149842033.v2.pressablecdn.com/wp-content/uploads/2019/03/breed2-free-portfolio-website-templates.jpg"
                ),
                String::from("https://themewagon.com/wp-content/uploads/2021/11/html.design.jpg")
            ],
            stacks: vec![
                String::from("Rust"),
                String::from("Leptos"),
                String::from("Actix Web"),
                String::from("Tailwind"),
                String::from("SurrealDB")
            ],
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Experience {
    pub company_name: String,
    pub company_logo_url: String,
    pub position_name: String,
    pub start_date: String,
    pub end_date: String,
    pub describe: String,
    pub company_url: String,
}
impl Default for Experience {
    fn default() -> Self {
        Self {
            company_name: String::from("Avengers Team"),
            company_logo_url: String::from(
                "https://seeklogo.com/images/A/avengers-logo-5B0A68AFB3-seeklogo.com.png"
            ),
            position_name: String::from("Spider Man"),
            company_url: String::from("https://en.wikipedia.org/wiki/List_of_Avengers_members"),
            start_date: String::from("2000-01-01"),
            end_date: String::from("Now"),
            describe: String::from(
                "Assisted in retrieving and securing dangerous alien technology (Captain America: Civil War).
Engaged in high-stakes urban combat during Battle of New York (Infinity War).
Participated in intergalactic rescue missions; fought Thanos’ army on Titan.
Blipped out of existence for five years, then returned to help in the final battle against Thanos (Endgame).
Provided support in rebuilding efforts post-Blip, maintaining neighborhood security."
            ),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub nick_name: String,
    pub gender: String,
    pub birth_date: String,
    pub role: String,
    pub nationality: String,
    pub about: String,
    pub avatar: String,
    pub id: String,
    pub address: String,
    pub skills: Option<Vec<Skill>>,
    pub experiences: Option<Vec<Experience>>,
    pub portfolios: Option<Vec<Portfolio>>,
    pub contacts: Option<Vec<Contact>>,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            id: String::from("portfolio_id"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            gender: String::from("Male"),
            birth_date: String::from("2000-01-01"),
            nationality: String::from("USA"),
            about: String::from("I'm Developer"),
            avatar: String::from(
                "https://raw.githubusercontent.com/marwin1991/profile-technology-icons/refs/heads/main/icons/github.png"
            ),
            nick_name: String::from("Rookie"),
            role: String::from("Developer"),
            skills: vec![Skill::default()].into(),
            address: String::from("In the world"),
            experiences: vec![Experience::default()].into(),
            portfolios: vec![Portfolio::default()].into(),
            contacts: vec![Contact::default()].into(),
        }
    }
}
