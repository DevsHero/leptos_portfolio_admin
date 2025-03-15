cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::env;
        use leptos::ServerFnError;
        use crate::app::models::portfolio::{ Experience, Portfolio, Profile, Skill, Contact };
        use surrealdb::engine::any::Any;
        use surrealdb::opt::auth::Root;
        use surrealdb::{ Surreal, Error };
        use once_cell::sync::Lazy;
        use surrealdb::sql::Thing;
        static DB: Lazy<Surreal<Any>> = Lazy::new(Surreal::init);
        pub async fn open_db_connection() -> Result<(), Error> {
            let host = env
                ::var("SURREAL_PROTOCOL_HOST")
                .unwrap_or("http:127.0.0.1:8000".to_string());
            let username = env::var("SURREAL_USER").unwrap_or("root".to_string());
            let password = env::var("SURREAL_PASS").unwrap_or("root".to_string());
            let ns = env::var("SURREAL_NAMESPACE").unwrap_or("surreal".to_string());
            let db_name = env::var("SURREAL_DB").unwrap_or("portfolio".to_string());
            let _ = DB.connect(host).await;

            let _ = DB.signin(Root {
                username: username.as_str(),
                password: password.as_str(),
            }).await;
            let _ = DB.use_ns(ns).use_db(db_name).await;

            Ok(())
        }

        pub async fn fetch_profile() -> Result<Option<Profile>, ServerFnError> {
            use serde::Deserialize;

            let _ = open_db_connection().await;
            let query = DB.query(
                "
                SELECT *,
                    (SELECT * FROM skill) AS skills, 
                    (SELECT * FROM experience) AS experiences,
                    (SELECT * FROM portfolio) AS portfolios,
                    (SELECT * FROM contact) AS contacts 
                FROM profile 
                LIMIT 1;
            "
            ).bind(("table", "profile")).await;
            let _ = DB.invalidate().await;

            match query {
                Ok(mut res) => {
                    // Create a temporary struct with the proper derive
                    #[derive(Debug, Deserialize)]
                    struct TempProfile {
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
                        pub id: Thing, // Using Thing instead of String
                        pub skills: Option<Vec<Skill>>,
                        pub experiences: Option<Vec<Experience>>,
                        pub portfolios: Option<Vec<Portfolio>>,
                        pub contacts: Option<Vec<Contact>>,
                    }

                    // First deserialize to the temporary struct
                    let found = res.take::<Option<TempProfile>>(0);

                    match found {
                        Ok(Some(temp_profile)) => {
                            // Convert to the actual Profile type with String id
                            let profile = Profile {
                                first_name: temp_profile.first_name,
                                last_name: temp_profile.last_name,
                                nick_name: temp_profile.nick_name,
                                gender: temp_profile.gender,
                                birth_date: temp_profile.birth_date,
                                role: temp_profile.role,
                                nationality: temp_profile.nationality,
                                about: temp_profile.about,
                                avatar: temp_profile.avatar,
                                address: temp_profile.address,
                                id: temp_profile.id.id.to_string(), // Convert Thing to String
                                skills: temp_profile.skills,
                                experiences: temp_profile.experiences,
                                portfolios: temp_profile.portfolios,
                                contacts: temp_profile.contacts,
                            };

                            Ok(Some(profile))
                        }
                        Ok(None) => Ok(None),
                        Err(e) => Err(ServerFnError::from(e)),
                    }
                }
                Err(e) => Err(ServerFnError::from(e)),
            }
        }

        pub async fn update_all_tables(
            profile: Profile,
            _is_update_skill: bool,
            _is_update_portfolio: bool,
            _is_update_experience: bool,
            _is_update_contact: bool
        ) -> Result<Option<Profile>, ServerFnError> {
            let _ = open_db_connection().await;

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
            update_data.insert("address", profile.address);
            update_data.insert("avatar", profile.avatar);
            if _is_update_skill {
                let _skill_result = update_skill(profile.skills.clone().expect("REASON")).await;
                // println!("_skill_result: {:?}", _skill_result);
            }

            if _is_update_experience {
                let _experience_result = update_experience(
                    profile.experiences.clone().expect("REASON")
                ).await;
                // println!("_experience_result: {:?}", _experience_result);
            }

            if _is_update_portfolio {
                let _portfolio_result = update_portfolio(
                    profile.portfolios.clone().expect("REASON")
                ).await;
                // println!("_portfolio_result: {:?}", _portfolio_result);
            }
            if _is_update_contact {
                let _contact_result = update_contact(
                    profile.contacts.clone().expect("REASON")
                ).await;
                // println!("_contact_result: {:?}", _contact_result);
            }

            let res: Result<Option<Profile>, Error> = DB.update((
                "profile",
                profile.id.clone(),
            )).content(update_data.clone()).await;
            let _ = DB.invalidate().await;
            println!("updated_user: {:?}", res);
            match res {
                Ok(user) => Ok(user),
                // let _ = DB.invalidate().await;
                Err(e) => Err(ServerFnError::from(e)),
            }
        }

        pub async fn update_skill(skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            let _ = open_db_connection().await;
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
            let _ = open_db_connection().await;
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
        pub async fn update_portfolio(
            portfolios: Vec<Portfolio>
        ) -> Result<Vec<Portfolio>, ServerFnError> {
            let _ = open_db_connection().await;
            let delete_all_records: Result<Vec<Portfolio>, Error> = DB.delete("portfolio").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&portfolios).unwrap();
                    let insert_records: Result<Vec<Portfolio>, Error> = DB.insert(
                        "portfolio"
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
        pub async fn update_contact(contacts: Vec<Contact>) -> Result<Vec<Contact>, ServerFnError> {
            let _ = open_db_connection().await;
            let delete_all_records: Result<Vec<Contact>, Error> = DB.delete("contact").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&contacts).unwrap();
                    let insert_records: Result<Vec<Contact>, Error> = DB.insert("contact").content(
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
    }
}
