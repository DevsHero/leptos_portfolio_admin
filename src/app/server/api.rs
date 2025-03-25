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
  <meta charset='UTF-8' />
  <title>Resume - Greta Cooper</title>
  <style>
    /* Base resets and font imports (optional) */
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    body {
      font-family: 'Helvetica Neue', Arial, sans-serif;
      color: #333;
      line-height: 1.6;
      background-color: #f5f5f5;
    }
    img {
      max-width: 100%;
      display: block;
    }

    /* Container */
    .resume-container {
      max-width: 900px;
      margin: 40px auto;
      background-color: #fff;
      display: flex;
      box-shadow: 0 0 10px rgba(0,0,0,0.1);
    }

    /* Left Column (Sidebar) */
    .sidebar {
      width: 30%;
      background-color: #f2efe9; /* Light beige or any preferred color */
      padding: 30px;
    }
    .profile-photo {
      width: 100px;
      height: 100px;
      border-radius: 50%;
      overflow: hidden;
      margin: 0 auto 20px auto;
    }
    .profile-photo img {
      width: 100%;
      height: auto;
    }
    .sidebar h2 {
      font-size: 1.4rem;
      margin-bottom: 15px;
      text-align: center;
      letter-spacing: 1px;
    }
    .sidebar .contact-info,
    .sidebar .social {
      margin-bottom: 30px;
    }
    .sidebar .contact-info li,
    .sidebar .social li {
      list-style: none;
      margin-bottom: 8px;
    }
    .sidebar .contact-info li i,
    .sidebar .social li i {
      margin-right: 8px;
    }
    .sidebar .contact-info li a,
    .sidebar .social li a {
      color: #333;
      text-decoration: none;
    }

    /* Main Content */
    .main-content {
      width: 70%;
      padding: 30px;
      background-color: #fff;
    }
    .main-content h1 {
      font-size: 2rem;
      margin-bottom: 5px;
      text-transform: uppercase;
    }
    .main-content h2 {
      font-size: 1rem;
      color: #999;
      margin-bottom: 30px;
      letter-spacing: 2px;
    }

    /* Sections */
    .section {
      margin-bottom: 40px;
    }
    .section h3 {
      font-size: 1.2rem;
      margin-bottom: 15px;
      text-transform: uppercase;
      border-bottom: 1px solid #ddd;
      padding-bottom: 5px;
      letter-spacing: 1px;
    }
    .timeline {
      margin-top: 15px;
    }
    .timeline .timeline-item {
      margin-bottom: 20px;
    }
    .timeline .timeline-item h4 {
      font-size: 1rem;
      margin-bottom: 5px;
    }
    .timeline .timeline-item .period {
      font-size: 0.85rem;
      color: #777;
      margin-bottom: 8px;
    }
    .timeline .timeline-item p {
      font-size: 0.9rem;
      color: #555;
      line-height: 1.4;
    }

    /* Skills */
    .skills-list {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
    }
    .skill-item {
      background-color: #f2efe9;
      padding: 8px 12px;
      border-radius: 4px;
      font-size: 0.85rem;
    }
  </style>
</head>
<body>
  <div class='resume-container'>
    <!-- Sidebar -->
    <aside class='sidebar'>
      <!-- Profile Photo -->
      <div class='profile-photo'>
        <!-- Replace with your own image URL -->
        <img src='https://via.placeholder.com/150' alt='Profile Photo'>
      </div>

      <!-- Name on Sidebar (Optional) -->
      <h2>Greta Cooper</h2>

      <!-- Contact Info -->
      <ul class='contact-info'>
        <li><strong>Name:</strong> Greta Cooper</li>
        <li><strong>Address:</strong> 123 Main Street, City, Country</li>
        <li><strong>Phone:</strong> +1 123 456 7890</li>
        <li><strong>Email:</strong> <a href='mailto:greta@example.com'>greta@example.com</a></li>
        <li><strong>Website:</strong> <a href='#'>www.gretacooper.com</a></li>
      </ul>

      <!-- Social Links -->
      <ul class='social'>
        <li><strong>Skype:</strong> greta.skype</li>
        <li><strong>Twitter:</strong> @gretacooper</li>
        <li><strong>LinkedIn:</strong> <a href='#'>linkedin.com/in/greta</a></li>
        <li><strong>Facebook:</strong> <a href='#'>facebook.com/gretacooper</a></li>
      </ul>
    </aside>

    <!-- Main Content -->
    <main class='main-content'>
      <!-- Name and Title -->
      <h1>Greta Cooper</h1>
      <h2>Graphic and Web Designer</h2>

      <!-- Work Experience -->
      <section class='section'>
        <h3>Work Experience</h3>
        <div class='timeline'>
          <div class='timeline-item'>
            <h4>D &amp; D Design Development</h4>
            <div class='period'>2013 - 2014</div>
            <p>
              Worked on brand identity projects and website designs for small to medium-sized businesses.
            </p>
          </div>
          <div class='timeline-item'>
            <h4>Brushstroke Studio</h4>
            <div class='period'>2012 - 2013</div>
            <p>
              Assisted senior designers in conceptualizing marketing materials and campaigns for clients.
            </p>
          </div>
          <div class='timeline-item'>
            <h4>Scribway Agency</h4>
            <div class='period'>2011 - 2012</div>
            <p>
              Developed and managed social media graphics and print advertisements for various brands.
            </p>
          </div>
        </div>
      </section>

      <!-- Education -->
      <section class='section'>
        <h3>Education</h3>
        <div class='timeline'>
          <div class='timeline-item'>
            <h4>Jaxel University, London</h4>
            <div class='period'>2012 - 2014</div>
            <p>Bachelor of Arts in Graphic Design</p>
          </div>
          <div class='timeline-item'>
            <h4>Clinton Community College</h4>
            <div class='period'>2010 - 2012</div>
            <p>Associate Degree in Visual Communication</p>
          </div>
        </div>
      </section>

      <!-- Skills and Expertise -->
      <section class='section'>
        <h3>Skills and Expertise</h3>
        <div class='skills-list'>
          <div class='skill-item'>Adobe Photoshop</div>
          <div class='skill-item'>Adobe Illustrator</div>
          <div class='skill-item'>Adobe InDesign</div>
          <div class='skill-item'>HTML/CSS</div>
          <div class='skill-item'>Responsive Design</div>
          <div class='skill-item'>Branding</div>
          <div class='skill-item'>UX/UI Principles</div>
        </div>
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
    _is_update_contact: bool
) -> Result<Option<Profile>, ServerFnError> {
    let updated = update_portfolio_api(
        profile,
        _is_update_skill,
        _is_update_portfolio,
        _is_update_experience,
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
            _is_update_contact: bool
        ) -> Result<Option<Profile>, ServerFnError> {
            database::update_all_tables(
                profile,
                _is_update_skill,
                _is_update_portfolio,
                _is_update_experience,
                _is_update_contact
            ).await
        }
        pub async fn update_skill_api(_skills: Vec<Skill>) -> Result<Vec<Skill>, ServerFnError> {
            database::update_skill(_skills).await
        }
    }
}
