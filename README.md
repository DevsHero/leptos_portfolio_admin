# Leptos Portfolio Admin

### Fullstack SSR Pure Rust Portfolio Site with Administration Panel

## Overview

Leptos Portfolio Admin is a comprehensive, full-stack portfolio website solution built entirely in Rust. It leverages the Leptos framework for server-side rendering (SSR) and interactive frontend components, coupled with an Actix Web backend API. You can visit my **[website](https://thanon.dev/)** to check it out.

**Key Highlights:**

* **Dynamic Content Management:** Features a secure admin panel allowing the site owner to easily add, update, and delete portfolio content (profile, skills, experience, projects, education, contact info) without modifying static code. The admin panel includes a WYSIWYG editor (TinyMCE) for rich text formatting.
* **Pure Rust Stack:** Demonstrates a full-stack web application using popular Rust crates like Leptos (SSR Frontend), Actix Web (Backend), SurrealDB (Database), and Redis (Caching).
* **Rich Portfolio Display:** Presents a clean, responsive portfolio page showcasing detailed information, including rich text descriptions ("About Me"), project galleries with tech stacks, skill levels, professional experience timelines, and more.
* **PDF Generation:** Includes functionality to automatically generate a downloadable PDF version of the portfolio content using Chromium in the backend.
* **Configurable & Themeable:** Supports site title customization and includes a dark mode toggle.
* **Learning Resource:** Serves as a practical example for developers interested in exploring Rust for web development, particularly SSR with Leptos and backend integration with Actix Web and SurrealDB.

This project aims to provide a ready-to-use, customizable portfolio site that is easy to maintain through its dedicated admin interface.

## Stacks

-   **[Leptos 0.6](https://leptos.dev/)**: A Rust frontend framework with SSR capabilities.
-   **[Actix Web](https://actix.rs/)**: A Rust backend framework for API handling.
-   **[SurrealDB](https://surrealdb.com/)**: The database used for data storage.
-   **[Tailwind CSS](https://tailwindcss.com/)**: A utility-first CSS framework for UI design.
-   **[Redis](https://redis.io/)**: Used for caching database queries.
-   **[Chromium](https://www.chromium.org/chromium-projects/)**: Used by the backend to generate PDF versions of the portfolio from HTML.
-   **[TinyMCE](https://www.tiny.cloud/)**: A WYSIWYG HTML editor integrated into the admin panel.

## Features

### Portfolio Page (Public View)

-   **Profile:** Displays name, age, nationality, gender, and job role.
-   **About Me:** Supports rich HTML text for detailed descriptions.
-   **Skills:** Lists skills with proficiency levels.
-   **Contact:** Dynamically configurable contact methods with associated icons show as link or popup message.
-   **Experiences:** Shows professional history including company logo, name, position, work period, and description.
-   **Portfolio:** Features projects with names, photos, open-source status, descriptions, and technology stacks.
-   **Education:** Lists educational background including institute name, address, major, degree, and GPA.
-   **Language:** Displays languages known and their proficiency levels.
-   **PDF:** Provides an option to view/download a PDF version of the portfolio.

### Admin Edit Page

-   **Permission Modes:**
    -   **Viewer Mode:** Allows viewing the admin sections without requiring a password (read-only).
    -   **Admin Mode:** Requires a password (set in `.env`) to enable adding, editing, and deleting content.
-   **Editable Sections:**
    -   **Profile:** Edit profile details and the "About Me" section using TinyMCE.
    -   **Skills:** Add or remove skills.
    -   **Experiences:** Add or remove work experiences.
    -   **Portfolio:** Add or remove portfolio projects.
    -   **Contact:** Add or remove contact methods.
    -   **Education:** Add or remove education entries.
    -   **Language:** Add or remove language proficiencies.
    -   **PDF:** Configure PDF generation (e.g., use generated HTML or a custom PDF link) and other related settings.

### General Features
-   **WYSIWYG Content:** Display your content as HTML, allowing complete freedom to design beautiful information.
-   **PDF Template:** Generate an HTML PDF from portfolio data using a minimal, cool resume template.
-   **Form Validation:** Ensure that all required fields are validated and prevent updates if any required fields are missing.
-   **Toast Notifications:** Provides feedback for actions performed on the admin page.
-   **Backend Server:** A simple Actix Web API server that interacts with the SurrealDB database.
-   **Responsive UI:** The user interface adapts smoothly to all devices screen sizes using Tailwind CSS.
-   **Caching:** Utilizes Redis to cache profile data and generated PDF files, improving website performance by reducing database queries and server processing.
-   **Site Configuration:**
    -   Set the website title.
    -   *Meta Tags: Under development.*
    -   *Other SEO Tags: Planned.*
-   **Dark Mode:** Toggle between light and dark themes.
-   **Intro Animation:** Welcome intro animation using Tailwind CSS..

## Prerequisites

1.  **Clone Project:**
    ```bash
    git clone [https://github.com/DevsHero/leptos_portfolio_admin.git](https://github.com/DevsHero/leptos_portfolio_admin.git)
    cd leptos_portfolio_admin
    ```
2.  **Prepare `.env` file:**
    Copy the example environment file. You will need to fill this file with your specific configuration details.
    ```bash
    cp .env-example .env
    ```
    Below are the variables defined in `.env-example` and their purpose:

    ```dotenv
    # --- SurrealDB Connection ---
    # Defines the protocol (http/https) and host/port for the SurrealDB instance.
    # Example: http:127.0.0.1:8000 or https:[your-cloud-instance.com](https://www.google.com/search?q=your-cloud-instance.com)
    SURREAL_PROTOCOL_HOST=http:127.0.0.1:8000
    # Username for SurrealDB authentication.
    SURREAL_USER=root
    # Password for SurrealDB authentication.
    SURREAL_PASS=root
    # The specific database name to use within SurrealDB.
    SURREAL_DB=portfolio
    # The namespace within SurrealDB.
    SURREAL_NAMESPACE=portfolio

    # --- Admin Panel ---
    # Password required to access the Admin Mode for editing site content.
    ADMIN_MODE_PASSWORD=admin

    # --- Site Configuration ---
    # The title displayed in the browser tab.
    SITE_TITLE="Portfolio site based on pure rust"

    # --- Redis Connection ---
    # Redis connection URL used during local development (cargo leptos watch).
    REDIS_URL_DEV="redis://localhost:6379"
    # Redis connection URL used in the production Docker environment (connects to the 'redis' service).
    REDIS_URL_PROD="redis://redis:6379"

    # --- Other (Add any other necessary variables here) ---
    ```
    **Important:** Replace the default values (like `SURREAL_USER`, `SURREAL_PASS`, `ADMIN_MODE_PASSWORD`) with your own secure settings before deployment.

3.  **Setup SurrealDB:**
    You need a running SurrealDB instance. You can set one up locally or use a cloud provider.
    -   **Local:** Follow the [official SurrealDB installation guide](https://surrealdb.com/install).
    -   **Cloud:** Services like [SurrealDB Cloud](https://surrealdb.com/cloud) are available.
4.  **Initialize Database Schema:**
    Connect to your SurrealDB instance (using the `surreal sql` command-line tool or a GUI like [Surrealist](https://surrealist.app/)). Copy and execute all the commands from the `surreal/script.surql` file to set up the necessary tables and schemas. Ensure you are connected to the correct namespace and database defined in your `.env` file (`NAMESPACE portfolio; USE DB portfolio;`).

## How to Run

### Option 1: Via Local Development

1.  **Install Rust:**
    If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    source "$HOME/.cargo/env" # Or restart your terminal
    ```
2.  **Install Leptos Toolchain:**
    ```bash
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown
    cargo install cargo-leptos
    ```
3.  **Install Redis:**
    The easiest way is often using Docker:
    ```bash
    # Ensure Docker is running
    docker compose -f docker-compose.dev.yml up -d redis
    ```
    *(Alternatively, install Redis directly via your system's package manager.)*
4.  **Install Chromium:**
    This is required for PDF generation. It must be `chromium`, not `google-chrome`. Installation methods vary by OS. See this guide for help: [Chromium Installation Guide](https://github.com/ArchiveBox/ArchiveBox/wiki/Chromium-Install)
5.  **Run the Application:**
    This command compiles both frontend (WASM) and backend, watches for changes, and serves the site using the settings in your `.env` file (specifically `REDIS_URL_DEV`).
    ```bash
    cargo leptos watch
    ```
6.  Access the site at `http://localhost:3000`.

### Option 2: Via Local Docker Build

1.  **Build the Docker Image:**
    *(Note: Build times can be significant, e.g., ~15 minutes on a high-end machine. Adjust `--platform` if needed.)*
    ```bash
    docker build --platform linux/amd64 -t leptos-portfolio-admin:latest .
    ```
2.  **Run using Docker Compose:**
    Make sure your `.env` file is configured correctly in the project root. This command will start the application container and the Redis container. The application inside Docker uses `REDIS_URL_PROD`.
    ```bash
    docker compose -f docker-compose.yml up -d --force-recreate leptos-portfolio-admin redis
    ```
    *(This assumes your `docker-compose.yml` defines services named `leptos-portfolio-admin` and `redis`, configured to use the built image and read the `.env` file.)*
3.  Access the site at `http://localhost:8080` (or the port mapped in your `docker-compose.yml`).

### Option 3: Via Docker Hub Image (If available)

*(Assuming you have published an image to Docker Hub and have a suitable `docker-compose.yml`)*

1.  **Pull and Start Containers:**
    Make sure your `.env` file is configured.
    ```bash
    # Ensure your docker-compose.yml points to the Docker Hub image for the leptos-portfolio-admin service
    docker compose -f docker-compose.yml pull leptos-portfolio-admin redis
    docker compose -f docker-compose.yml up -d --force-recreate
    ```
2.  Access the site at `http://localhost:8080` (or the port mapped in your `docker-compose.yml`).

## Planned Features

-   **LLM Chat Integration:** Integrate a Large Language Model (LLM) for chat-based interactions about the portfolio content.
-   **Agentic AI Features:** Implement AI-driven assistance for visitors, such as:
    -   Sending email inquiries to the portfolio owner.
    -   Booking meetings via a scheduling service.
    -   Translating content.
    -   Answering questions intelligently (Q&A).
    -   Providing deeper insights into profile details or projects.
-   **One-Script Setup:** Develop a shell script for automated setup of all prerequisites and dependencies.
-   **Migration Leptos 0.7:** Migrated to branch migration-leptos-v7 but encountered several bugs and performance issues that I couldn't resolve. Maybe I'll try again next time.

## Contributing

This is my first Rust project, Contributions are highly welcome! If you find any bugs, have suggestions for improvements, or want to add new features, please feel free to open an issue or submit a pull request.

*Note: This project has been primarily tested on Linux (Ubuntu) and macOS. While it may work on Windows (especially with Docker or WSL), its stability on that platform is not guaranteed.*

## Contact Me

Thanon Aphithanawat (Hero)
**mail@thanon.dev**