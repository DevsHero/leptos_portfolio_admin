cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::fmt::Write;
        use crate::app::models::Profile;
        use super::utils::FONT_AWESOME_MAP;
        mod qr_gen {
            // --- Imports for fast_qr ---
            use fast_qr::{
                qr::{QRBuilder }, // Use fast_qr's builder and ECC level
                convert::{image::ImageBuilder, Builder, Shape, ConvertError}, // Use fast_qr's image converter and Error type
            };
            // --- Imports for image encoding and base64 ---
            // NOTE: We don't directly use the `image` crate's types for rendering here,
            // but we *do* need it to encode the final raw pixel data from fast_qr into PNG format.
            use image::{ImageBuffer, Luma, ImageOutputFormat };
            use base64::{engine::general_purpose::STANDARD, Engine as _};
            use std::io::Cursor;
        
            pub fn generate_qr_code_data_uri(text: &str) -> Result<String, String> {
                // --- 1. Build QR data using fast_qr ---
                let qrcode = QRBuilder::new(text.as_bytes())
                    .build()
                    .unwrap(); 
        
                let image_builder = ImageBuilder::default()
                .shape(Shape::RoundedSquare)
                .background_color([255, 255, 255, 0]) // Handles transparency
                .fit_width(100) .to_bytes(&qrcode ).unwrap();;
        
                // --- 6. Encode the PNG bytes to base64 ---
                let base64_string = STANDARD.encode(image_builder);
        
                // --- 7. Format as a Data URI ---
                Ok(format!("data:image/png;base64,{}", base64_string))
            }
        }
        pub fn generate_html_string(profile: &Profile) -> Result<String, std::fmt::Error> {
            let mut html = String::with_capacity(16384); // Increased capacity slightly
            // --- HEAD ---
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
                pdf_css() // Embed CSS here
            )?;

            // --- Avatar Section ---
            write!(
                html,
                r#"<img src="{}" alt="{} {}" class="profile-pic">"#,
                html_escape(&profile.avatar),
                html_escape(&profile.first_name),
                html_escape(&profile.last_name)
            )?;

            write!(
                html,
                r#"<h1 class="nick-name">{}</h1><p class="job-title">{}</p></div>"#,  
                html_escape(&profile.nick_name.to_uppercase()),
                html_escape(&profile.role.to_uppercase())
            )?;

            // --- Profile Section ---
            write!(
                html,
                r#" <div class='section profile-section'>
                <h2><i class='fas fa-id-card'></i> Profile</h2>
                <ul class='profile-list'>
                    <li> <b class="b-class">Name</b>Thanon Aphithanawat</li>
                    <li> <b class="b-class">Age</b> 38</li>
                    <li> <b class="b-class">Nationality</b> Thai</li>
                </ul>
            </div>"#
            )?;
            // --- Contact Section ---
            if let Some(contacts) = &profile.contacts {
                write!(
                    html,
                    r#"<div class="section contact-section"><h2><i class="fas fa-envelope-open-text"></i> Contact</h2><ul class="contact-list">"#
                )?;

                // Handle Address separately
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

                // Loop through contacts
                for contact in contacts {
                    // Get icon class from map
                    let icon_class = FONT_AWESOME_MAP.get(contact.contact_icon.as_str())
                        .copied()
                        .unwrap_or("fas fa-link"); // Fallback icon

                    // --- Check if value is an HTTP URL ---
                    let value_is_http_url =
                        contact.contact_value.starts_with("http://") ||
                        contact.contact_value.starts_with("https://");

                    // Write the list item start and icon
                    write!(html, r#"<li><i class="{}"></i> "#, icon_class)?;

                    // --- Logic Branching ---
                    if value_is_http_url {
                        // --- Attempt QR Code Generation (SSR only) ---
                        #[cfg(feature = "ssr")]
                        {
                            match qr_gen::generate_qr_code_data_uri(&contact.contact_value) {
                                Ok(qr_data_uri) => {
                                    // Embed the QR code image (size set via CSS or attributes)
                                    write!(
                                        html,
                                        r#"<img class="qr-code" src="{}" alt="QR Code for {}" width="50" height="50">"#, // Example with attributes
                                        qr_data_uri,
                                        html_escape(&contact.contact_value)
                                    )?;
                                }
                                Err(e) => {
                                    // Fallback: Log error and display the URL as a text link
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
                            // Non-SSR Fallback (client-side, shouldn't normally render this HTML, but good practice)
                            // Just render as a link
                            write!(
                                html,
                                r#"<a href="{}" target="_blank" rel="noopener noreferrer">{}</a>"#,
                                html_escape(&contact.contact_value),
                                html_escape(
                                    contact.contact_title.as_ref().unwrap_or(&contact.contact_value)
                                )
                            )?;
                        }
                        // --- End QR Code Logic ---
                    } else if contact.use_link {
                        // --- Not an HTTP URL, BUT use_link is TRUE: Render as a standard link (like original code) ---
                        // This handles cases like mailto:, tel:, or other values intended as links
                        write!(
                            html,
                            r#"<a href="{}" target="_blank" rel="noopener noreferrer">{}</a>"#, // Modify href target if needed for mailto/tel
                            html_escape(&contact.contact_value), // Consider prefixing mailto: or tel: if needed based on contact_icon
                            html_escape(
                                contact.contact_title.as_ref().unwrap_or(&contact.contact_value)
                            )
                        )?;
                    } else {
                        // --- Not an HTTP URL and use_link is FALSE: Render as plain text ---
                        write!(html, "{}", html_escape(&contact.contact_value))?;
                    }

                    // Close the list item
                    write!(html, "</li>")?;
                }
                write!(html, "</ul></div>")?;
            }
            // --- Skills Section ---
            if let Some(skills) = &profile.skills {
                if !skills.is_empty() {
                    write!(
                        html,
                        r#"<div class="section skills-section"><h2><i class="fas fa-cogs"></i> Skills</h2>"#
                    )?;
                    for skill in skills {
                        write!(html, r#"<div class="skill"><p>{}</p>"#, html_escape(&skill.name))?;
                        // Simple Dot Logic: Assume level is "1" to "7"
                        let level_num = skill.level.parse::<u8>().unwrap_or(0); // Default to 0 on parse error
                        write!(html, r#"<div class="dots">"#)?;
                        for i in 1..=7 {
                            // Assuming max 7 dots
                            let dot_class = if i <= level_num { "filled" } else { "empty" };
                            write!(html, r#"<span class="dot {}"></span>"#, dot_class)?;
                        }
                        write!(html, r#"</div></div>"#)?; // Close dots and skill div
                    }
                    write!(html, r#"</div>"#)?; // Close skills section
                }
            }

            // --- Languages Section ---
            if let Some(languages) = &profile.languages {
                if !languages.is_empty() {
                    write!(
                        html,
                        r#"<div class="section skills-section"><h2><i class="fas fa-language"></i> Languages</h2>"#
                    )?; // Reuse skills style
                    for lang in languages {
                        write!(
                            html,
                            r#"<div class="skill"><p>{}</p><p class="level-text">({})</p></div>"#,
                            html_escape(&lang.name),
                            html_escape(&lang.level)
                        )?;
                    }
                    write!(html, r#"</div>"#)?; // Close languages section
                }
            }

            // --- Close Left Column ---
            write!(
                html,
                r#"</div><!-- ==================== Left Column End ==================== -->"#
            )?;

            // --- ==================== Right Column Start ==================== ---
            write!(html, r#"<div class="right-column">"#)?;

            // --- About Me Section ---
            if !profile.about.is_empty() {
                write!(
                    html,
                    r#"<div class="section about-section"><h2><i class="fas fa-user"></i> About Me</h2><p>{}</p></div>"#,
                    html_escape(&profile.about)
                )?;
            }

            // --- Education Section ---
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
                        // Optionally add address/GPA if needed and available
                        // write!(html, r#"<p>{}</p>"#, html_escape(&edu.institute_address))?;
                        // write!(html, r#"<p>GPA: {}</p>"#, html_escape(&edu.gpa))?;
                        write!(html, r#"</div></div>"#)?; // Close timeline-content and timeline-item
                    }
                    write!(html, r#"</div></div>"#)?; // Close timeline and section
                }
            }

            // --- Work Experience Section ---
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
                            html_escape(&exp.start_date),
                            html_escape(&exp.end_date)
                        )?;
                        write!(html, r#"<p class="role">{}</p>"#, html_escape(&exp.position_name))?;
                        write!(html, r#"<ul>"#)?;
                        for line in exp.describe.lines() {
                            let trimmed_line = line.trim();
                            if !trimmed_line.is_empty() {
                                write!(html, r#"<li>{}</li>"#, html_escape(trimmed_line))?;
                            }
                        }
                        write!(html, r#"</ul>"#)?;
                        write!(html, r#"</div></div>"#)?; // Close timeline-content and timeline-item
                    }
                    write!(html, r#"</div></div>"#)?; // Close timeline and section
                }
            }

            // --- Portfolio Section ---
            if let Some(portfolios) = &profile.portfolios {
                if !portfolios.is_empty() {
                    // Reuse work-experience style or create a specific one if needed
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

                        if !portfolio.portfolio_link.is_empty() {
                            write!(
                                html,
                                r#"<p class="role"><a href="{}" target="_blank" rel="noopener noreferrer">View Project</a>"#,
                                html_escape(&portfolio.portfolio_link)
                            )?;
                            if portfolio.is_opensource {
                                write!(html, r#" (Open Source)"#)?;
                            }
                            write!(html, r#"</p>"#)?;
                        }

                        write!(html, r#"<ul>"#)?;
                        if !portfolio.portfolio_detail.is_empty() {
                            write!(
                                html,
                                r#"<li>{}</li>"#,
                                html_escape(&portfolio.portfolio_detail)
                            )?;
                        }
                        if !portfolio.stacks.is_empty() {
                            write!(
                                html,
                                r#"<li>Stacks: {}</li>"#,
                                html_escape(&portfolio.stacks.join(", "))
                            )?;
                        }
                        write!(html, r#"</ul>"#)?;
                        write!(html, r#"</div></div>"#)?; // Close timeline-content and timeline-item
                    }
                    write!(html, r#"</div></div>"#)?; // Close timeline and section
                }
            }

            // --- Close Right Column ---
            write!(
                html,
                r#"</div><!-- ==================== Right Column End ==================== -->"#
            )?;

            // --- Closing Tags ---
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
        body {
        font-family: 'Lato', sans-serif;
        color: #333;
        line-height: 1.6;
        font-size: 10pt;
        /* background-color: #fff; <-- Can be removed if set on html, body above */
  
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
        
        .profile-pic {
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
        .right-column .section {
       
            
        }
        .right-column .about-section {
 
        }
        .right-column .about-section h2 {
            
        }
        .right-column .about-section p {
       
        }
        /* Contact Section Styling */
        .contact-section h2 {
            background-color: #2c3e50;
        }
       .profile-section h2 {
            background-color: #2c3e50;
        }
        .contact-list {
            list-style: none;
            padding-left: 20px; /* Indent list */
        }
        
        .contact-list li {
            margin-bottom: 12px;
            display: flex;
            align-items: center;
            font-size: 0.9em;
            color: #444;
        }
        
        .contact-list   li i {
            color: #2c3e50; /* Match header color */
             font-size: 20px; 
            margin-right: 15px;
            width: 16px; /* Fixed width for alignment */
            text-align: center;
        }
               .profile-list li {
            margin-bottom: 12px;
            display: flex;
           
            font-size: 0.9em;
            color: #444;
        }
        
        .profile-list   li b {
            color: #2c3e50; /* Match header color */
          
        }
        .b-class{
            width: 70px;
        
        }
        /* Skills Section Styling */
        .skills-section h2 {
            background-color: #2c3e50;
        }
        
        .skill {
            margin-bottom: 15px;
            padding-left: 20px; /* Indent skills */
        }
        
        .skill p {
            margin-bottom: 5px;
            font-weight: 700;
            color: #333;
            font-size: 1em;
        }
        
        .dots {
            display: flex; 
        }
        
        .dot {
            height: 10px;
            width: 10px;
            border-radius: 50%;
            background-color: #ccc;
            margin-right: 5px;
        }
        
        .dot.filled {
            background-color: #555; 
        }
        
       
        .right-column h3 {
            font-size: 1.1em;
            font-weight: 700;
            color: #2c3e50;
            margin-bottom: 2px;
        }
        
        .right-column .date {
            font-size: 0.85em;
            color: #666;
            font-weight: 700;
            display: block;
            margin-bottom: 5px;
        }
        .right-column .institution,
        .right-column .role {
            font-size: 1em;
            font-weight: 700;
            color: #444;
            margin-bottom: 10px;
        }
        
        .right-column ul {
            list-style: disc;
            margin-left: 20px; /* Indent bullet points */
            padding-right: 15px; /* Space on right */
            
        }
        
        .right-column ul li {
            margin-bottom: 8px;
            font-size: 0.9em;
            color: #555;
            
        }
        
    
        .timeline {
            position: relative;
            padding-left: 30px; /* Space for the line and dots */
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
