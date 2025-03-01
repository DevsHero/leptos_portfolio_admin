use serde::{ Deserialize, Serialize };
use surrealdb::sql::Thing;
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Skill {
    pub name: String,
    pub level: String,
}
impl Default for Skill {
    fn default() -> Self {
        Self {
            name: String::from("Javascript"),
            level: String::from("basic"),
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
    pub id: Thing,
    pub skills: Vec<Skill>,
    pub experiences: Vec<Experience>,
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            id: Thing::from(("profile", "0")),
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
            skills: vec![Skill::default()],
            experiences: vec![Experience::default()],
        }
    }
}
