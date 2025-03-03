use crate::app::models::Experience;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::ServerFnError;
        use crate::app::models::{ Profile, Skill };
        use surrealdb::engine::remote::ws::{ Client, Wss };
        use surrealdb::opt::auth::Root;
        use surrealdb::{ Surreal, Error };
        use once_cell::sync::Lazy;
        static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);
        pub async fn open_db_connection() {
            use std::env;
            let host = env::var("SURREAL_HOST").unwrap_or("127.0.0.1:8000".to_string());
            let username = env::var("SURREAL_USER").unwrap_or("root".to_string());
            let password = env::var("SURREAL_PASS").unwrap_or("root".to_string());
            let ns = env::var("SURREAL_NAMESPACE").unwrap_or("surreal".to_string());
            let db_name = env::var("SURREAL_DB").unwrap_or("portfolio".to_string());
            DB.connect::<Wss>(host).await;
            DB.signin(Root {
                username: username.as_str(),
                password: password.as_str(),
            }).await;
            let _ = DB.use_ns(ns).use_db(db_name).await;
        }
        pub async fn fetch_profile() -> Result<Option<Profile>, ServerFnError> {
            open_db_connection().await;
            let query = DB.query(
                "
            SELECT *,
                (SELECT * FROM skill  ) AS skills, 
                (SELECT * FROM experience  ) AS experiences 
            FROM profile 
            LIMIT 1;
        "
            ).bind(("table", "profile")).await;
            let _ = DB.invalidate().await;
            // println!("Query result: {:?}",query);
            match query {
                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(result) => Ok(result),
                        Err(e) => Err(ServerFnError::from(e)),
                    }
                }
                Err(e) => Err(ServerFnError::from(e)),
            }
        }
        // pub async fn fetch_profile_skills() -> Result<Option<ProfileWithSkills>, ServerFnError> {
        //     open_db_connection().await;
        //     let query = DB.query(
        //         "RETURN {
        //              profile: (SELECT * FROM profile LIMIT 1)[0],
        //              skill: (SELECT * FROM skill)
        //          }"
        //     ).await;
        //     let _ = DB.invalidate().await;
        //     match query {
        //         Ok(mut res) => {
        //             let found = res.take(0);
        //             match found {
        //                 Ok(result) => Ok(result),
        //                 Err(e) => Err(ServerFnError::from(e))
        //             }
        //         },
        //         Err(e) => Err(ServerFnError::from(e))
        //     }
        // }
        pub async fn fetch_skill() -> Result<Option<Skill>, ServerFnError> {
            open_db_connection().await;
            let query = DB.query("SELECT * FROM skill;").await;
            let _ = DB.invalidate().await;
            println!("Query result: {:?}", query);
            match query {
                Ok(mut res) => {
                    let found = res.take(0);
                    match found {
                        Ok(result) => Ok(result),
                        Err(e) => Err(ServerFnError::Args(e.to_string())),
                    }
                }
                Err(e) => Err(ServerFnError::Args(e.to_string())),
            }
        }

        pub async fn update_profile(profile: Profile) -> Result<Option<Profile>, ServerFnError> {
            open_db_connection().await;

            // Extract the profile ID
            let profile_id = profile.id.clone();

            // Create a map of the fields to update
            let mut update_data = std::collections::HashMap::new();
            update_data.insert("first_name", profile.first_name);
            update_data.insert("last_name", profile.last_name);
            update_data.insert("nick_name", profile.nick_name);
            update_data.insert("gender", profile.gender);
            update_data.insert("birth_date", profile.birth_date);
            update_data.insert("role", profile.role);
            update_data.insert("nationality", profile.nationality);
            update_data.insert("about", profile.about);
            update_data.insert("avatar", profile.avatar);

            let skill_result = update_skill(profile.skills.clone().expect("REASON")).await;
            // println!("skill_result: {:?}", skill_result);
            let experience_result = update_experience(
                profile.experiences.clone().expect("REASON")
            ).await;
            // println!("experience_result: {:?}", experience_result);
            let updated_user: Result<Option<Profile>, Error> = DB.update((
                profile.id.tb.clone(),
                profile.id.id.to_string(),
            )).merge(update_data).await;
            let _ = DB.invalidate().await;
            match updated_user {
                Ok(returned_user) => Ok(returned_user),
                Err(e) => Err(ServerFnError::from(e)),
            }
        }
        pub async fn update_skill(skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            open_db_connection().await;
            let delete_all_records: Result<Vec<Skill>, Error> = DB.delete("skill").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&skills).unwrap();
                    let insert_records: Result<Vec<Skill>, Error> = DB.insert("skill").content(
                        json_value
                    ).await;
                    // println!("Query result: {:?}",insert_records);
                    match insert_records {
                        Ok(inserted) => Ok(inserted),
                        // let _ = DB.invalidate().await;
                        Err(e) => Err(ServerFnError::from(e)),
                    }
                }
                Err(e) => Err(ServerFnError::from(e)),
            }
        }
        pub async fn update_experience(
            experiences: Vec<Experience>
        ) -> Result<Vec<Experience>, ServerFnError> {
            open_db_connection().await;
            let delete_all_records: Result<Vec<Experience>, Error> = DB.delete("experience").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&experiences).unwrap();
                    let insert_records: Result<Vec<Experience>, Error> = DB.insert(
                        "experience"
                    ).content(json_value).await;
                    // println!("Query result: {:?}",insert_records);
                    match insert_records {
                        Ok(inserted) => Ok(inserted),
                        // let _ = DB.invalidate().await;
                        Err(e) => Err(ServerFnError::from(e)),
                    }
                }
                Err(e) => Err(ServerFnError::from(e)),
            }
        }
    }
}
