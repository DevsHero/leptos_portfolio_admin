use leptos::{ server, ServerFnError };
use crate::app::models::{ Profile, SiteConfig, Skill };
use std::env;

#[server(GetProfile, "/api")]
pub async fn get_profile() -> Result<Profile, ServerFnError> {
    let data = retrieve_profile_api().await;
    match data {
        Ok(Some(profile)) => Ok(profile),
        Ok(None) => Err(ServerFnError::ServerError("No profile found".to_string())),
        Err(e) => Err(ServerFnError::from(e)),
    }
}
#[server(Verify, "/api")]
pub async fn verify(password: String) -> Result<bool, ServerFnError> {
    let admin_password = std::env::var("ADMIN_MODE_PASSWORD").unwrap_or("admin".to_string());
    Ok(admin_password == password)
}

#[server(PdfExport, "/api")]
pub async fn pdf_export() -> Result<String, ServerFnError> {
    use base64::{ engine::general_purpose::STANDARD, Engine as _ };
    use gotenberg_pdf::{ Bytes, Client, WebOptions };
    use std::error::Error;

    let client = Client::new("http://localhost:4000");

    let html_content =
        r#"
   <!DOCTYPE html>
<html lang='en'>
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>Beautiful Resume</title>
    <style>
        body {
            font-family: sans-serif;
            margin: 40px;
            background-color: #f4f4f4;
            display: flex;
            justify-content: center;
        }
        .container {
            background-color: #fff;
            display: flex;
            max-width: 900px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
        }
        .sidebar {
            background-color: #f9f3ec;
            color: #333;
            padding: 30px;
            width: 30%;
        }
        .main-content {
            padding: 30px;
            width: 70%;
        }
        h1, h2, h3 {
            color: #222;
            margin-top: 0;
            margin-bottom: 10px;
        }
        h1 {
            font-size: 2.5em;
        }
        h2 {
            font-size: 1.8em;
            border-bottom: 2px solid #ddd;
            padding-bottom: 5px;
            margin-bottom: 15px;
        }
        h3 {
            font-size: 1.2em;
            font-weight: bold;
            margin-bottom: 5px;
        }
        .info-item {
            margin-bottom: 15px;
        }
        .info-item strong {
            display: block;
            margin-bottom: 5px;
            color: #555;
            font-size: 0.9em;
        }
        .social-links {
            list-style: none;
            padding: 0;
            margin-top: 20px;
        }
        .social-links li {
            margin-bottom: 8px;
        }
        .social-links li a {
            color: #333;
            text-decoration: none;
        }
        .work-experience-item, .education-item {
            margin-bottom: 20px;
        }
        .date-range {
            color: #777;
            font-size: 0.9em;
            margin-bottom: 5px;
        }
        .skill-item {
            margin-bottom: 10px;
        }
        .skill-item strong {
            display: block;
            margin-bottom: 3px;
            font-size: 0.95em;
        }
        .skill-bar {
            background-color: #ddd;
            height: 8px;
            border-radius: 4px;
            overflow: hidden;
        }
        .skill-level {
            background-color: #888; /* Adjust color as needed */
            height: 100%;
            border-radius: 4px;
        }
        .references-item {
            margin-bottom: 15px;
        }
        .references-item h4 {
            font-weight: bold;
            margin-bottom: 3px;
        }
        .references-item p {
            margin-bottom: 5px;
            font-size: 0.9em;
            color: #555;
        }
        .profile-image {
            width: 100px;
            height: 100px;
            border-radius: 50%;
            object-fit: cover;
            margin-bottom: 20px;
        }
        .sidebar-title {
            color: #222;
            font-size: 1.5em;
            margin-bottom: 15px;
            border-bottom: 1px solid #ddd;
            padding-bottom: 5px;
        }
        .main-content > h1 {
            font-size: 3em;
            margin-bottom: 5px;
        }
        .main-content > p {
            color: #555;
            font-size: 1.1em;
            margin-bottom: 20px;
        }
    </style>
</head>
<body>
    <div class='container'>
        <aside class='sidebar'>
            <img src='placeholder-profile.jpg' alt='Profile Picture' class='profile-image'>
            <h1>GRETA COOPER</h1>
            <p>GRAPHIC AND WEB DESIGNER</p>

            <div class='info-section'>
                <h2 class='sidebar-title'>INFO</h2>
                <div class='info-item'>
                    <strong>Name</strong>
                    <p>Aveda Beson</p>
                </div>
                <div class='info-item'>
                    <strong>Address</strong>
                    <p>45645 Smet ligme</p>
                    <p>City, Province</p>
                    <p>State Country</p>
                </div>
                <div class='info-item'>
                    <strong>Phone</strong>
                    <p>022301230223</p>
                </div>
                <div class='info-item'>
                    <strong>Email</strong>
                    <p>youreros@geunt.com</p>
                </div>
                <div class='info-item'>
                    <strong>Website</strong>
                    <p>VOUPWEDEE.CO</p>
                </div>
            </div>

            <div class='social-section'>
                <h2 class='sidebar-title'>SOCIAL</h2>
                <ul class='social-links'>
                    <li><a href='#'>Skype: youreskypem</a></li>
                    <li><a href='#'>Twitter: yourerwteraccount.com</a></li>
                    <li><a href='#'>LinkedIn</a></li>
                    <li><a href='#'>Facebook</a></li>
                    <li><a href='#'>Website: ndeowww.beenscow.com/</a></li>
                </ul>
            </div>

            <div class='references-section'>
                <h2 class='sidebar-title'>REFERENCES</h2>
                <div class='references-item'>
                    <h4>Carl Jager</h4>
                    <p>Senior Designer at Capital P.</p>
                    <p>Phone: 012201210123</p>
                    <p>Email: sourensstemmit.com</p>
                </div>
                <div class='references-item'>
                    <h4>Melissa Nortex</h4>
                    <p>Senior Designer at Capital P.</p>
                    <p>Phone: 012201220123</p>
                    <p>Email: yourinencall.com</p>
                </div>
            </div>
        </aside>
        <main class='main-content'>
            <h1>WORK EXPERIENCE</h1>
            <section class='work-experience-item'>
                <h3>D & P design developement</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Your first position</h4>
                <p>Ur fenitates volum que quiasimus ma ditium esendelenis eumquate poria por rehenimus, ut asit, qui ut quidelist laccae exerror autresequi oluptas et remo con plandae.</p>
            </section>

            <section class='work-experience-item'>
                <h3>Braunhouse Sudio xl</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Ur, tenitates volum que quiasim</h4>
                <p>Experum hil es utaest reperem peribus erspelias as estrum repent mod et que consequod ute laccus ernam, quidi volupta dundendam, alignam dernam eveni consecte valoribus eum.</p>
            </section>

            <section class='work-experience-item'>
                <h3>Strawberry Agency</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Senior Graphic Designer</h4>
                <p>Tin reperestrum harior maion conesto quo beri nestiam renostrum ipid ma qui iliquiae venihillamus ditatur alignaturem dolum a sus, quibust oribusam eatur aut ped.</p>
            </section>

            <section class='work-experience-item'>
                <h3>Toolkit Design Developement</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Senior Illustrator</h4>
                <p>Ur, tenitates volum que quiasimus ma ditium esendelenis eumquate poria por rehenimus, ut asit, qui ut quidelist laccae exerror aut resequi oluptas et remo con plandae.</p>
            </section>

            <h2>EDUCATION</h2>
            <section class='education-item'>
                <h3>Jueel University, London</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Bacelor in graphic design</h4>
                <p>Itates volum que quiasimus ma ditium esendelenis eum.</p>
            </section>

            <section class='education-item'>
                <h3>Clinton University, London</h3>
                <p class='date-range'>2012-2014</p>
                <h4>Master in graphic design</h4>
                <p>Itates volum que quiasimus ma ditium esendelenis eum vendit.</p>
            </section>

            <h2>SKILLS AND EXPERTIZE</h2>
            <section class='skill-item'>
                <strong>Photoshop</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 85%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Illustrator</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 90%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Dreamweaver</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 70%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>AfterEffects</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 60%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Ms Word</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 95%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Ms Exel</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 80%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Creativity</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 92%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Flexibility</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 88%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Work in group</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 90%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Personality</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 85%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Ms Word</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 95%;'></div></div>
            </section>
            <section class='skill-item'>
                <strong>Ms Exel</strong>
                <div class='skill-bar'><div class='skill-level' style='width: 80%;'></div></div>
            </section>
        </main>
    </div>
</body>
</html>
    "#;

    let options = WebOptions::default();

    let pdf_bytes = client
        .pdf_from_html(html_content, options).await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let encoded_pdf = STANDARD.encode(&pdf_bytes);
    Ok(encoded_pdf)
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
    _is_update_language: bool,
    _is_update_education: bool,
    _is_update_contact: bool
) -> Result<Option<Profile>, ServerFnError> {
    let updated = update_portfolio_api(
        profile,
        _is_update_skill,
        _is_update_portfolio,
        _is_update_experience,
        _is_update_language,
        _is_update_education,
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
            _is_update_language: bool,
            _is_update_education: bool,
            _is_update_contact: bool
        ) -> Result<Option<Profile>, ServerFnError> {
            database::update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_language,
                _is_update_education,
                _is_update_contact
            ).await
        }
        pub async fn update_skill_api(_skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            database::update_skill(_skills).await
        }
    }
}
