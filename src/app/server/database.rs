cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::env;
        use leptos::ServerFnError;
        use crate::app::models::profile::{
            Experience,
            Portfolio,
            Profile,
            Skill,
            Contact,
            Education,
            Language,
        };
        use surrealdb::engine::any::Any;
        use surrealdb::opt::auth::Root;
        use surrealdb::{ Surreal, Error };
        use once_cell::sync::Lazy;

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

        pub async fn server_fetch_profile() -> Result<Option<Profile>, ServerFnError> {
            use crate::app::models::server;
            let _ = open_db_connection().await;
            let query = DB.query(
                "
                SELECT *,
                    (SELECT * FROM skill ORDER BY level DESC) AS skills, 
                    (SELECT * FROM experience ORDER BY start_date DESC) AS experiences,
                    (SELECT * FROM portfolio ORDER BY index ASC ) AS portfolios,
                    (SELECT * FROM education ORDER BY graduated_year ASC ) AS educations ,
                    (SELECT * FROM language ORDER BY level DESC ) AS languages ,
                    (SELECT * FROM contact ORDER BY use_link ASC ) AS contacts 
                FROM profile 
                LIMIT 1;
            "
            ).bind(("table", "profile")).await;
            let _ = DB.invalidate().await;

            match query {
                Ok(mut res) => {
                    // Create a temporary struct with the proper derive
                    // Since SurrealDB's ID is of type `Thing`, not a `String`, we need to convert it to a `String`.
                    // We can't use the `Profile` model directly, so we use a temporary model and convert it afterward.

                    let found = res.take::<Option<server::ThingProfile>>(0);

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
                                pdf: temp_profile.pdf,
                                about: temp_profile.about,
                                avatar: temp_profile.avatar,
                                address: temp_profile.address,
                                id: Some(temp_profile.id.id.to_string()), // Convert Thing to String
                                skills: temp_profile.skills,
                                experiences: temp_profile.experiences,
                                portfolios: temp_profile.portfolios,
                                contacts: temp_profile.contacts,
                                languages: temp_profile.languages,
                                educations: temp_profile.educations,
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

        pub async fn server_update_all_tables(
            profile: Profile,
            _is_update_skill: bool,
            _is_update_portfolio: bool,
            _is_update_experience: bool,
            _is_update_language: bool,
            _is_update_education: bool,
            _is_update_contact: bool
        ) -> Result<bool, ServerFnError> {
            let _ = open_db_connection().await;

            if _is_update_skill {
                let _skill_result = server_update_skill(
                    profile.skills.clone().expect("REASON")
                ).await;
                // println!("_skill_result: {:?}", _skill_result);
            }

            if _is_update_experience {
                let _experience_result = server_update_experience(
                    profile.experiences.clone().expect("REASON")
                ).await;
                // println!("_experience_result: {:?}", _experience_result);
            }

            if _is_update_portfolio {
                let _portfolio_result = server_update_profile_api(
                    profile.portfolios.clone().expect("REASON")
                ).await;
                // println!("_portfolio_result: {:?}", _portfolio_result);
            }
            if _is_update_education {
                let _education_result = server_update_education(
                    profile.educations.clone().expect("REASON")
                ).await;
                // println!("_contact_result: {:?}", _contact_result);
            }
            if _is_update_language {
                let _language_result = server_update_language(
                    profile.languages.clone().expect("REASON")
                ).await;
                // println!("_contact_result: {:?}", _contact_result);
            }
            if _is_update_contact {
                let _contact_result = server_update_contact(
                    profile.contacts.clone().expect("REASON")
                ).await;
                // println!("_contact_result: {:?}", _contact_result);
            }
            let mut update_profile_clone = profile.clone();
            update_profile_clone.skills = None;
            update_profile_clone.experiences = None;
            update_profile_clone.portfolios = None;
            update_profile_clone.contacts = None;
            update_profile_clone.educations = None;
            update_profile_clone.languages = None;
            update_profile_clone.id = None;

            let res: Result<Option<Profile>, Error> = DB.update((
                "profile",
                profile.id.clone().unwrap(),
            )).content(update_profile_clone).await;
            let _ = DB.invalidate().await;

            match res {
                Ok(_user) => Ok(true),
                Err(e) => {
                    let error_string = e.to_string();
                    if
                        error_string.contains("failed to deserialize") &&
                        error_string.contains(
                            "expected a string, found $surrealdb::private::sql::Thing"
                        )
                    {
                        return Ok(true);
                    }
                    Err(ServerFnError::from(e)) // Re-throw other errors
                }
            }
        }

        pub async fn server_update_skill(skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
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
        pub async fn server_update_experience(
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
        pub async fn server_update_profile_api(
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
        pub async fn server_update_education(
            educations: Vec<Education>
        ) -> Result<Vec<Education>, ServerFnError> {
            let _ = open_db_connection().await;
            let delete_all_records: Result<Vec<Education>, Error> = DB.delete("education").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&educations).unwrap();
                    let insert_records: Result<Vec<Education>, Error> = DB.insert(
                        "education"
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
        pub async fn server_update_language(
            languages: Vec<Language>
        ) -> Result<Vec<Language>, ServerFnError> {
            let _ = open_db_connection().await;
            let delete_all_records: Result<Vec<Language>, Error> = DB.delete("language").await;
            match delete_all_records {
                Ok(_deleted) => {
                    let json_value = serde_json::to_value(&languages).unwrap();
                    let insert_records: Result<Vec<Language>, Error> = DB.insert(
                        "language"
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
        pub async fn server_update_contact(
            contacts: Vec<Contact>
        ) -> Result<Vec<Contact>, ServerFnError> {
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
