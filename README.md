# Leptos Portfolio Admin
### Fullstack SSR Pure Rust Portfolio Site with Administration Panel

## **Overview**

An all-in-one portfolio solution with dynamic content management through an admin interface. Designed to enable users to update portfolio content dynamically rather than using static data. This project also serves as a learning resource for understanding SSR (Server-Side Rendering) and full-stack implementation in Rust.

## **Stack**
- **Leptos 6**: Rust frontend framework with SSR capabilities
- **Actix Web**: Rust backend framework for API handling
- **SurrealDB**: Database for data storage
- **Tailwind**:  Utility-first CSS framework for UI design

## **Features**

- **Portfolio Page:**
  - **Profile:** Displays the user's name, age, nationality, gender, job role.
  - **About Me:** Supports rich HTML text (currently under development).
  - **Skills:** Lists multiple skills with their respective levels.
  - **Contact:** Allows for dynamic contact information customization with different icons.
  - **Experiences:** Includes company logo, company name, position, work period, and description.
  - **Portfolio:** Features project names, photos, open-source status, descriptions, and associated technology stacks.

- **Admin Edit Page:**
  - **Permission Modes:** Offers two access modes for the edit page:
    - **Viewer Mode:** Allows viewing without a password requirement.
    - **Admin Mode:** Enables data updates with a required password (password stored in `.env`).
  - **Sections:**
    - **Profile:** Edit profile and about me information.
    - **Skills:** Add or remove skills.
    - **Experiences:** Add or remove experiences.
    - **Portfolio:** Add or remove portfolio items.
    - **Contact:** Add or remove contact details.
      (Note: The current icon mapping is limited to 7 icons. For additional icons, you can expand the `ICON_MAP` in utils and refer to this link for more icons: [https://carloskiki.github.io/icondata/](https://carloskiki.github.io/icondata/) )


- **Server:** Provides a simple API supporting all connection methods to SurrealDB.
- **Responsive UI:** Supports both web and mobile layouts through Tailwind CSS.
- **Site Config:**
  - **Site Title:** Currently supported.
  - **Meta Tags:** Under development.
  - **Other SEO Tags:** Potential future additions.
- **Dark Mode:** Now supports dark mode (currently under progress to store configuration in local storage).



## **How to run**
- **Prerequisites:**
   **Prepare .env:**
  ```
  mv .env-example .env
  ```
   **Setup Surrealdb:**
  You have two ways to use SurrealDB for free—just choose one.
   - [Local Surrealdb](https://surrealdb.com/install)
   - [Cloud Surrealdb](https://surrealist.app/)

   **Setup Database:**
    
- **Via Local Development:**
  1. install rust
  ```
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```  
  2. install leptos
  ```
  rustup toolchain install nightly
  rustup default nightly
  rustup target add wasm32-unknown-unknown
  cargo install cargo-leptos
  ```
  3. run letpos
  ```
  cargo leptos watch
  ```
  4. The site already served at localhost:3000

- **Via Docker:**
  1. Build image (From macbook m4 cpu 12core 512gb the build time estimate 850 sec.)
  ```
  docker build --platform linux/amd64 -t leptos-portfolio-admin:lastest . 
  ```
  2. run image with .env file
  ```
  docker run -d -p 3000:3000 --platform linux/amd64 --env-file '.env' leptos-portfolio-admin:latest
  ```
  3. The site already served at localhost:3000

## **Future Feature** 
- **LLM Chat:**
 About Portfolio: Integrate a language model (LLM) to provide chat-based interactions about the portfolio.
- **Rich Text Editor:**
 Implement a rich text editor for enhanced content creation in sections like "About Me."
- **Export PDF Resume:**
 Allow to export their portfolio data as a PDF resume.

- **One script setup:**
 Shell script with all in one setup and ready to dev.

## **Contributing**
This project is my first open-source endeavor, and I am always looking for contributions to help improve it. If you encounter any bugs or have suggestions for new features, please don't hesitate to open an issue. This project has been tested on Linux and macOS environments, but I haven’t tested it on Windows yet, so stability on that platform is uncertain.