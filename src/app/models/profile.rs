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
pub struct Language {
    pub name: String,
    pub level: String,
}
impl Default for Language {
    fn default() -> Self {
        Self {
            name: String::from("English"),
            level: String::from("Native"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct PDF {
    pub use_pdf: bool,
    pub use_generate: bool,
    pub pdf_link: Option<String>,
    pub use_about_pdf_version: bool,
    pub use_avatar_pdf_version: bool,
    pub about_pdf_data: Option<String>,
    pub avatar_pdf_url: Option<String>,
    pub show_contact: bool,
    pub show_language: bool,
    pub show_about: bool,
    pub show_education: bool,
    pub show_experience: bool,
    pub show_portfolio: bool,
    pub show_skill: bool,
    pub show_profile: bool,
    pub show_avatar: bool,
}
impl Default for PDF {
    fn default() -> Self {
        Self {
            use_pdf: bool::from(false),
            use_generate: bool::from(false),
            pdf_link: None,
            use_about_pdf_version: bool::from(false),
            about_pdf_data: None,
            use_avatar_pdf_version: bool::from(false),
            avatar_pdf_url: None,
            show_about: bool::from(true),
            show_education: bool::from(true),
            show_contact: bool::from(true),
            show_experience: bool::from(true),
            show_language: bool::from(true),
            show_portfolio: bool::from(true),
            show_profile: bool::from(true),
            show_skill: bool::from(true),
            show_avatar: bool::from(true),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Contact {
    pub contact_icon: String,
    pub use_link: bool,
    pub contact_value: String,
    pub contact_title: Option<String>,
}
impl Default for Contact {
    fn default() -> Self {
        Self {
            contact_icon: String::from("MdiEmail"),
            use_link: bool::from(true),
            contact_value: String::from("developer@portfolio.com"),
            contact_title: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Portfolio {
    pub portfolio_name: String,
    pub portfolio_link: String,
    pub is_opensource: bool,
    pub portfolio_detail: String,
    pub portfolio_icon_url: String,
    pub stacks: Vec<String>,
    pub screenshots_url: Vec<String>,
    pub index: u8,
    pub use_portfolio_detail_pdf_version: bool,
    pub portfolio_detail_pdf_data: Option<String>,
}
impl Default for Portfolio {
    fn default() -> Self {
        Self {
            index: u8::from(1),
            portfolio_name: String::from("Leptos Portfolio Admin"),
            portfolio_link: String::from("https://github.com/zelda2003/leptos_portfolio_admin"),
            is_opensource: bool::from(false),
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
            use_portfolio_detail_pdf_version: bool::from(false),
            portfolio_detail_pdf_data: None,
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
    pub company_address: String,
    pub use_describe_pdf_version: bool,
    pub describe_pdf_data: Option<String>,
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
            company_address: String::from("Hollywood USA"),
            use_describe_pdf_version: bool::from(false),
            describe_pdf_data: None,
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
pub struct Education {
    pub institute_name: String,
    pub institute_logo_url: String,
    pub graduated_year: String,
    pub degree: String,
    pub institute_address: String,
    pub major: String,
    pub gpa: String,
}
impl Default for Education {
    fn default() -> Self {
        Self {
            institute_name: String::from("Stanford University"),
            institute_logo_url: String::from(
                "https://identity.stanford.edu/wp-content/uploads/sites/3/2020/07/SU_SealColor_web3.png"
            ),
            graduated_year: String::from("2010"),
            degree: String::from("bachelor's degree"),
            institute_address: String::from("CA USA"),
            major: String::from("computer science"),
            gpa: String::from("4.00"),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Profile {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub nick_name: String,
    pub gender: String,
    pub birth_date: String,
    pub pdf: PDF,
    pub role: String,
    pub nationality: String,
    pub about: String,
    pub avatar: String,
    pub address: String,
    pub skills: Option<Vec<Skill>>,
    pub experiences: Option<Vec<Experience>>,
    pub portfolios: Option<Vec<Portfolio>>,
    pub contacts: Option<Vec<Contact>>,
    pub languages: Option<Vec<Language>>,
    pub educations: Option<Vec<Education>>,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            id: None,
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            gender: String::from("Male"),
            birth_date: String::from("2000-01-01"),
            nationality: String::from("USA"),
            pdf: PDF::default(),
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
            languages: vec![Language::default()].into(),
            educations: vec![Education::default()].into(),
        }
    }
}
