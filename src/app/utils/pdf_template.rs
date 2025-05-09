cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::app::{ models::Profile, utils::utils::convert_date_format };
        use super::utils::FONT_AWESOME_MAP;
        use crate::app::constants::constant::{ LANGUAGE_LEVELS, SKILL_LEVELS };
        pub fn generate_pdf(html_content: &str) -> Result<Vec<u8>, String> {
            use std::io::Write;
            use std::env;
            use std::process::Command;
            use std::fs::{ self, File };
            use leptos::logging;
            use uuid::Uuid;
            let temp_dir = env::temp_dir();
            let unique_id = Uuid::new_v4();
            let input_html_path = temp_dir.join(format!("input_{}.html", unique_id));
            let output_pdf_dir = temp_dir.join(format!("output_{}.pdf", unique_id));
            let input_path_str = input_html_path
                .to_str()
                .ok_or("Failed to create valid input path string")?;
            let output_path_str = output_pdf_dir
                .to_str()
                .ok_or("Failed to create valid output path string")?;

            {
                let mut input_file = File::create(&input_html_path).map_err(|e|
                    format!("Failed to create temp HTML file '{}': {}", input_path_str, e)
                )?;
                input_file
                    .write_all(html_content.as_bytes())
                    .map_err(|e|
                        format!("Failed to write to temp HTML file '{}': {}", input_path_str, e)
                    )?;
            }
            let mut cmd = Command::new("chromium");
            cmd.arg("--no-sandbox")
                .arg("--headless")
                .arg("--disable-gpu ")
                .arg("--no-pdf-header-footer")
                .arg("--virtual-time-budget=10000")
                .arg("--print-to-pdf=".to_owned() + output_path_str)
                .arg(input_path_str);

            let output = cmd
                .output()
                .map_err(|e|
                    format!("Failed to execute generate pdf command: {}. Is it installed and in PATH? Is Chrome/Chromium installed?", e)
                )?;

            let pdf_bytes: Result<Vec<u8>, String>;
            if output.status.success() {
                pdf_bytes = fs
                    ::read(&output_pdf_dir)
                    .map_err(|e|
                        format!(
                            "Successfully ran generate pdf, but failed to read output PDF '{}': {}",
                            output_path_str,
                            e
                        )
                    );
            } else {
                let stderr_output = String::from_utf8_lossy(&output.stderr);
                let stdout_output = String::from_utf8_lossy(&output.stdout);
                logging::error!("generate pdf stdout: {}", stdout_output);
                logging::error!("generate pdf stderr: {}", stderr_output);
                pdf_bytes = Err(
                    format!(
                        "generate pdf failed with status {}. Check logs for details.",
                        output.status
                    )
                );
            }
            let _ = fs::remove_file(&input_html_path);
            let _ = fs::remove_file(&output_pdf_dir);

            pdf_bytes
        }
        mod qr_gen {
            use fast_qr::{ qr::{ QRBuilder }, convert::{ image::ImageBuilder, Builder, Shape } };

            use base64::{ engine::general_purpose::STANDARD, Engine as _ };

            pub fn generate_qr_code_data_uri(text: &str) -> Result<String, String> {
                let qrcode = QRBuilder::new(text.as_bytes()).build().unwrap();
                let image_builder = ImageBuilder::default()
                    .shape(Shape::RoundedSquare)
                    .background_color([255, 255, 255, 0]) 
                    .fit_width(100)
                    .to_bytes(&qrcode)
                    .unwrap();
                let base64_string = STANDARD.encode(image_builder);
                Ok(format!("data:image/png;base64,{}", base64_string))
            }
        }
        pub fn generate_html_string(profile: &Profile) -> Result<String, std::fmt::Error> {
            use std::fmt::Write;
            use super::utils::calculate_age;
            let mut html = String::with_capacity(16384); 

            write!(
                html,
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} {} - {}</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Lato:wght@300;400;700;900&display=swap" rel="stylesheet">
    <style>
        {}
    </style>
</head>
<body>
    <div class="resume-container">
        <!-- ==================== Left Column Start ==================== -->
        <div class="left-column">
            <div class="avatar-section">"#,
                html_escape(&profile.first_name),
                html_escape(&profile.last_name),
                html_escape(&profile.role),
                pdf_css() 
            )?;

            if profile.pdf.show_avatar {
                write!(
                    html,
                    r#"<img src="{}" alt="{} {}" class="avatar-pic">"#,
                    if profile.pdf.use_avatar_pdf_version {
                        html_escape(&profile.pdf.avatar_pdf_url.clone().unwrap())
                    } else {
                        html_escape(&profile.avatar)
                    },
                    html_escape(&profile.first_name),
                    html_escape(&profile.last_name)
                )?;
            }
            write!(
                html,
                r#"<h1 class="nick-name">{}</h1><p class="job-title">{}</p></div>"#,
                html_escape(&profile.nick_name.to_uppercase()),
                html_escape(&profile.role.to_uppercase())
            )?;

            if profile.pdf.show_profile {
                let age = calculate_age(&profile.birth_date);
                write!(
                    html,
                    r#" <div class='section profile-section'>
                <h2><i class='fas fa-id-card'></i> Profile</h2>
                <div class='profile-list'>
                    <p> <b class="b-class">Name</b>{} {}</p>
                    <p> <b class="b-class">Age</b>{}</p>
                    <p> <b class="b-class">Nationality</b>{}</p>
                </div>
            </div>"#,
                    &profile.first_name,
                    &profile.last_name,
                    &age,
                    &profile.nationality
                )?;
            }

            if profile.pdf.show_contact {
                if let Some(contacts) = &profile.contacts {
                    write!(
                        html,
                        r#"<div class="section contact-section"><h2><i class="fas fa-envelope-open-text"></i> Contact</h2><ul class="contact-list">"#
                    )?;

                    if !profile.address.is_empty() {
                        let address_icon = FONT_AWESOME_MAP.get("Address")
                            .copied()
                            .unwrap_or("fas fa-map-marker-alt");
                        write!(
                            html,
                            r#"<li><i class="{}"></i> {}</li>"#,
                            address_icon,
                            html_escape(&profile.address)
                        )?;
                    }


                    for contact in contacts {
                        let icon_class = FONT_AWESOME_MAP.get(contact.contact_icon.as_str())
                            .copied()
                            .unwrap_or("fas fa-link");

                        let value_is_http_url =
                            contact.contact_value.starts_with("http://") ||
                            contact.contact_value.starts_with("https://");

                        write!(html, r#"<li><i class="{}"></i> "#, icon_class)?;

                        if value_is_http_url {
                            #[cfg(feature = "ssr")]
                            {
                                match qr_gen::generate_qr_code_data_uri(&contact.contact_value) {
                                    Ok(qr_data_uri) => {
                                       write!(
                                            html,
                                            r#"<img class="qr-code" src="{}" alt="QR Code for {}" width="50" height="50">"#,
                                            qr_data_uri,
                                            html_escape(&contact.contact_value)
                                        )?;
                                    }
                                    Err(e) => {
                                        eprintln!("SSR QR Generation Error: {}", e);
                                        write!(
                                            html,
                                            r#"<a href="{}" target="_blank" rel="noopener noreferrer">{} (QR Error)</a>"#,
                                            html_escape(&contact.contact_value),
                                            html_escape(
                                                contact.contact_title
                                                    .as_ref()
                                                    .unwrap_or(&contact.contact_value)
                                            )
                                        )?;
                                    }
                                }
                            }
                            #[cfg(not(feature = "ssr"))]
                            {
                                write!(
                                    html,
                                    r#"<a href="{}" target="_blank" rel="noopener noreferrer">{}</a>"#,
                                    html_escape(&contact.contact_value),
                                    html_escape(
                                        contact.contact_title
                                            .as_ref()
                                            .unwrap_or(&contact.contact_value)
                                    )
                                )?;
                            }
                        } else if contact.use_link {
                           write!(
                                html,
                                r#"<a href="{}" target="_blank" rel="noopener noreferrer">{}</a>"#, 
                                html_escape(&contact.contact_value), 
                                html_escape(
                                    contact.contact_title.as_ref().unwrap_or(&contact.contact_value)
                                )
                            )?;
                        } else {
                            write!(html, "{}", html_escape(&contact.contact_value))?;
                        }

                        write!(html, "</li>")?;
                    }
                    write!(html, "</ul></div>")?;
                }
            } 

            if profile.pdf.show_language {
                if let Some(languages) = &profile.languages {
                    if !languages.is_empty() {
                        write!(
                            html,
                            r#"<div class="section skills-section"><h2><i class="fas fa-cogs"></i> Languages</h2>"#
                        )?;
                        for language in languages {
                            let level_info = LANGUAGE_LEVELS.iter()
                                .find(|&&(value, _)| value == language.level)
                                .unwrap_or(&("0", "Unknown"));
                            write!(
                                html,
                                r#"<div class="skill"><p>{}</p><div class="level level-{}">{}</div></div>"#,
                                html_escape(&language.name),
                                html_escape(&language.level.to_lowercase()),
                                html_escape(&level_info.1)
                            )?;
                        }
                        write!(html, r#"</div>"#)?;
                    }
                }
            }
      
            if profile.pdf.show_skill {
                if let Some(skills) = &profile.skills {
                    if !skills.is_empty() {
                        write!(
                            html,
                            r#"<div class="section skills-section"><h2><i class="fas fa-cogs"></i> Skills</h2>"#
                        )?;
                        for skill in skills {
                            let level_info = SKILL_LEVELS.iter()
                                .find(|&&(value, _)| value == skill.level)
                                .unwrap_or(&("0", "Unknown"));
                            write!(
                                html,
                                r#"<div class="skill"><p>{}</p><div class="level level-{}">{}</div></div>"#,
                                html_escape(&skill.name),
                                html_escape(&skill.level.to_lowercase()),
                                html_escape(&level_info.1)
                            )?;
                        }
                        write!(html, r#"</div>"#)?; 
                    }
                }
            }
        
            write!(
                html,
                r#"</div><!-- ==================== Left Column End ==================== -->"#
            )?;

            write!(html, r#"<div class="right-column">"#)?;

            if profile.pdf.show_about {
                if !profile.about.is_empty() {
                    write!(
                        html,
                        r#"<div class="section about-section"><h2><i class="fas fa-user"></i> About Me</h2><p>{}</p></div>"#,
                        if profile.pdf.use_about_pdf_version {
                            html_escape(&profile.pdf.about_pdf_data.clone().unwrap())
                        } else {
                            html_escape(&profile.about)
                        }
                    )?;
                }
            }

            if profile.pdf.show_education {
                if let Some(educations) = &profile.educations {
                    if !educations.is_empty() {
                        write!(
                            html,
                            r#"<div class="section education-section"><h2><i class="fas fa-graduation-cap"></i> Education</h2><div class="timeline">"#
                        )?;
                        for edu in educations {
                            write!(
                                html,
                                r#"<div class="timeline-item"><div class="timeline-content">"#
                            )?;
                            write!(
                                html,
                                r#"<h3>{} in {}</h3>"#,
                                html_escape(&edu.degree),
                                html_escape(&edu.major)
                            )?;
                            write!(
                                html,
                                r#"<span class="date">{}</span>"#,
                                html_escape(&edu.graduated_year)
                            )?;
                            write!(
                                html,
                                r#"<p class="institution">{}</p>"#,
                                html_escape(&edu.institute_name)
                            )?;
                            write!(html, r#"</div></div>"#)?;
                        }
                        write!(html, r#"</div></div>"#)?;
                    }
                }
            }

            if profile.pdf.show_experience {
                if let Some(experiences) = &profile.experiences {
                    if !experiences.is_empty() {
                        write!(
                            html,
                            r#"<div class="section work-experience-section"><h2><i class="fas fa-briefcase"></i> Work Experience</h2><div class="timeline">"#
                        )?;
                        for exp in experiences {
                            write!(
                                html,
                                r#"<div class="timeline-item"><div class="timeline-content">"#
                            )?;
                            write!(html, r#"<h3>{}</h3>"#, html_escape(&exp.company_name))?;
                            write!(
                                html,
                                r#"<span class="date">{} - {}</span>"#,
                                {
                                    convert_date_format(&exp.start_date)
                                },
                                {
                                    convert_date_format(&exp.end_date)
                                }
                            )?;
                            write!(
                                html,
                                r#"<p class="role">{}</p>"#,
                                html_escape(&exp.position_name)
                            )?;

                            write!(html, r#"<p>{}</p>"#, if exp.use_describe_pdf_version {
                                html_escape(&exp.describe_pdf_data.clone().unwrap())
                            } else {
                                html_escape(&exp.describe)
                            })?;

                            write!(html, r#"</div></div>"#)?;
                        }
                        write!(html, r#"</div></div>"#)?;
                    }
                }
            }

            if profile.pdf.show_portfolio {
                if let Some(portfolios) = &profile.portfolios {
                    if !portfolios.is_empty() {
                        write!(
                            html,
                            r#"<div class="section work-experience-section"><h2><i class="fas fa-project-diagram"></i> Portfolio</h2><div class="timeline">"#
                        )?;
                        for portfolio in portfolios {
                            write!(
                                html,
                                r#"<div class="timeline-item"><div class="timeline-content">"#
                            )?;
                            write!(html, r#"<h3>{}</h3>"#, html_escape(&portfolio.portfolio_name))?;

                            if portfolio.is_opensource {
                                write!(html, r#"<p class="role"> (Open Source)</p>"#)?;
                            }

                            write!(html, r#"<ul>"#)?;

                            if !portfolio.stacks.is_empty() {
                                write!(
                                    html,
                                    r#"<li>Stacks: {}</li>"#,
                                    html_escape(&portfolio.stacks.join(", "))
                                )?;
                            }
                            write!(html, r#"<p>{}</p>"#, if
                                portfolio.use_portfolio_detail_pdf_version
                            {
                                html_escape(&portfolio.portfolio_detail_pdf_data.clone().unwrap())
                            } else {
                                html_escape(&portfolio.portfolio_detail)
                            })?;
                            write!(html, r#"</ul>"#)?;
                            write!(html, r#"</div></div>"#)?; 
                        }
                        write!(html, r#"</div></div>"#)?;
                    }
                }
            }

            write!(
                html,
                r#"</div><!-- ==================== Right Column End ==================== -->"#
            )?;

            write!(html, r#"</div> <!-- Close resume-container --></body></html>"#)?;

            Ok(html)
        }
        fn html_escape(s: &str) -> String {
            s.replace('&', "&").replace('<', "<").replace('>', ">").replace('\'', "'")
        }

        fn pdf_css() -> String {
            "    *     html, body {
        margin: 0 !important;    /* Remove default browser margins */
        padding: 0 !important;   /* Remove default browser padding */
        background-color: #fff; /* Set explicit white background */
        width: 100%;           /* Ensure they take full width */
        height: 100%;          /* Ensure they take full height */
        box-sizing: border-box;  /* Consistent box model */
    }  
    @page {
      size: 8.5in 11in; 
        margin: 0.1in;
}
        body {
        font-family: 'Lato', sans-serif;
        color: #333;
        line-height: 1.6;
        font-size: 10pt;
  
    }
       .resume-container {
        max-width: 100%; 
        min-height: 100vh; 
        margin: 0 !important;
        padding: 0 !important; 
        background-color: #fff;
        display: flex;
        /* box-shadow: 0 0 15px rgba(0,0,0,0.1); <-- Remove shadow for edge-to-edge */
    }
        
        .left-column {
            width: 35%; 
            background-color: #fff; 
           padding-left : 10px;
        }
        .right-column {
            width: 65%; 
            background-color: #fff;
        }
        .avatar-section {
        
            text-align: center;
        margin-top: 15px;
        }
        
        .avatar-pic {
            width: 180px; 
            height: 180px;
            object-fit: cover;
            display: block;
            margin-left: auto;
            margin-right: auto;
            border-radius: 20px;
           
        }
        .avatar-section h1 {
            font-size: 2.8em;
            font-weight: 300; /* Light weight for first name */
            color: #333;
            margin-bottom: -10px; /* Adjust spacing between names */
            letter-spacing: 1px;
        }
        .avatar-section .nick-name {
            font-weight: 900; /* Bold weight for last name */
            font-size: 2.9em;
            color: #000; /* Black for emphasis */
     
        }
        .avatar-section .job-title {
            font-size: 0.9em;
            color: #555;
            letter-spacing: 2px;
            font-weight: 400;
        }
        .section h2 {
            background-color: #2c3e50; /* Dark blue-gray */
            color: #fff;
            padding: 5px 10px;
            width: 95%;
            border-radius: 20px; /* Rounded corners */
            font-size: 1.2em;
            font-weight: 700;
            display: inline-flex; /* Align icon and text */
            align-items: center;
            
        }
        .section h2 i {
            margin-right: 10px;
            font-size: 1em;
        }
        .left-column .section {
                 margin-right:15px
        }
       
code{
   font-family: 'Courier New', Courier, monospace;
  color: hsl(107, 100%, 50%);
  display: block;
  padding: 1rem;
  margin: auto;
  background: hsl(0, 0%, 0%);      /* Light background for code */
  overflow-x: auto;         /* Horizontal scroll if needed */
  white-space: pre-wrap; 
  width:88%;
}
       
        .right-column .about-section p {
            padding-left:15px;
            padding-right:15px;
        }
        /* Contact Section Styling */
        .contact-section h2 {
            background-color: #2c3e50;.
           
        }
       .profile-section h2 {
            background-color: #2c3e50;
        }
        .contact-list {
            list-style: none;
            padding-left: 20px;
           margin: 0;
        }
        
        .contact-list li {
            height: 55px;
            display: flex;
            align-items: center;
            font-size: 1em;
            color: #444;
        }
        
        .contact-list   li i {
            color: #2c3e50; /* Match header color */
          font-size: 22px;
            margin-right: 15px;
            width: 16px; /* Fixed width for alignment */
            text-align: center;
        }
        .profile-list p {
            margin-bottom: 12px;
                display: flex;
            flex-direction: row;
            font-size: 1em;
            color: #444;
        }
        .b-class{
            width: 75px;
         color: #2c3e50;
             margin-left : 20px
        }
            .skills-section{
             margin-top: 10px;
            }
        /* Skills Section Styling */
        .skills-section h2 {
            background-color: #2c3e50; 
        }
        .skill {
            display: flex;
            flex-direction: row;
            align-items: center;
            justify-content: space-between;
            padding-left: 20px; /* Indent skills */
        }
.level{
font-weight: bold;
 text-align: center;   
padding: 5px 15px;
 width: 100px;
border-radius : 20px;
 
}

.level-1 {
  background: hwb(59 1% 34%);
  color: white;
}
.level-2 {
  background: hwb(33 5% 20%);
  color: white;
}
.level-3 {
  background: rgb(10, 143, 191);
  color: white;
}

.level-4 {
  background: #0abf28;
  color: white;
}
        .skill p {
            margin-bottom: 5px;
            font-weight: 700;
            color: #333;
            font-size: 1em;
        }

       
        .right-column h3 {
            font-size: 1.1em;
            font-weight: 700;
            color: #2c3e50; 
            margin:0 ;
        }
        
         .right-column p {
           margin:0;
        }
        
        .right-column .date {
            font-size: 0.9em;
            color: #666;
            font-weight: 700;
            display: block; 
        }
        .right-column .institution,
        .right-column .role {
            font-size: 1em;
            font-weight: 700;
            color: #444; 
        }
        
        .right-column ul {
            list-style: disc;
            padding-left: 10px; /* Indent bullet points */
            padding-right: 10px; /* Space on right */
            
        }
        
        .right-column ul li {
            margin-bottom: 4px;
            font-size: 0.9em;
            color: #555;
            
        }
        .timeline {
            position: relative;
            padding-left: 28px; /* Space for the line and dots */
            margin-left: 0; /* Align with header indent */
        }
        .timeline::before {
      
            content: '';
            position: absolute;
            left: 0;
            top: 10px; 
            bottom: 10px;
            width: 2px;
            background-color: #2c3e50; /* Line color */
        }
        .timeline-item {
            position: relative;
            margin-bottom: 30px;
        }
        .timeline-item:last-child {
            margin-bottom: 0;
        }
        .timeline-item::before {
            content: '';
            position: absolute;
            left: -36px; 
            top: 5px; 
            width: 12px;
            height: 12px;
            background-color: #fff; 
            border: 3px solid #2c3e50; 
            border-radius: 50%;
            z-index: 1; 
        }
        .qr-code {
    width: 50px;  
    height: 50px;  
    display: inline-block;
    vertical-align: middle;
  
}
        .timeline-content {
            position: relative;
        }".to_string()
        }
    }
}
