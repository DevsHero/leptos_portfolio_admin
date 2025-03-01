<!-- <picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>
# Leptos Starter Template
This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.
## Creating your template repo
If you don't have `cargo-leptos` installed you can install it with
`cargo install cargo-leptos --locked`
Then run
`cargo leptos new --git leptos-rs/start`
to generate a new project template (you will be prompted to enter a project name).
`cd {projectname}`
to go to your newly created project.
Of course, you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.
## Running your project
`cargo leptos watch`  
By default, you can access your local project at `http://localhost:3000`
## Installing Additional Tools
By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.
1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)
## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:
1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`
Copy these files to your remote server. The directory structure should be:
```text
leptos_start
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.
## Notes about CSR and Trunk:
Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:
`trunk serve --open --features csr`
This may be useful for integrating external tools which require a static site, e.g. `tauri`.
## Licensing
This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly. -->
# Full Stack Rust Dashboard App with Leptos, Actix Web & SurrealDB
## **Overview**
This project we are build a modern full-stack dashboard application using Rust. It combines:
- **Leptos** for the frontend, providing interactive and responsive user interfaces.
- **Actix Web** for the backend, ensuring high-performance and robust API handling.
- **SurrealDB** for managing and querying complex data with ease.
## **Features**
- **User-friendly dashboard:** Displays key statistics like team size and monthly costs.
- **Team management:**
    - Add new team members with details like name, title, level, and compensation.
    - Edit existing team member information.
    - Delete team members from the system.
- **Data visualization:**  Includes a simple bar chart to visualize team member distribution by role.
- **Multi-model database:** Integration with SurrealDB for flexible and powerful data handling.
---
## **What is Leptos?**
[Leptos](https://github.com/leptos-rs/leptos) is a Rust-based framework for building modern web applications. It allows you to write reactive UIs using declarative syntax.
- Why Leptos?
    - Performance: Highly optimized for speed and efficiency.
    - SSR: Render HTML on the server for better SEO and faster initial load times.
    - Reusability: Build reusable components for a modular codebase.
---
## **SSR vs. CSR**
- **Server-Side Rendering (SSR):**
    - Renders the application’s HTML on the server before sending it to the client.
    - Benefits: Faster initial load, SEO-friendly, and works well for content-heavy applications.
    - Example: When a user visits the dashboard, the server generates the initial view before the browser loads additional interactivity.
- **Client-Side Rendering (CSR):**
    - Renders the application on the client’s browser using JavaScript or WebAssembly.
    - Benefits: Rich interactivity and smoother navigation without reloading the page.
    - Example: After the dashboard loads, any user interaction (e.g., adding data) is handled dynamically in the browser.
---
## **Technologies Used:**
- **Rust:** Programming language.
- **Leptos:** Frontend framework.
- **Actix Web:** Backend framework.
- **SurrealDB:** NoSQL database.
- **Tailwind CSS:** For styling.
---
## **How It Works**
1. Frontend (Leptos):
- The user interface is built using Leptos components.
- Includes SSR for fast initial load and CSR for dynamic interactions.
2. Backend (Actix Web):
- Handles API requests and communicates with the database.
- Provides endpoints for authentication, data fetching, and updates.
3. Database (SurrealDB):
- Stores user data and dashboard information.
- Supports complex queries for multi-relational data.
---
## **Getting Started**
### **Prerequisites**
- Install Rust and Cargo.
- Install Leptos and its dependencies.
    ```bash
        -$ cargo install cargo-leptos
        -$ rustup toolchain install nightly // set as default
        -$ cargo install wasm32-unknown-unknown // WASM build pacakage
    ```
- Make sure install SurrealDB installed.
### **Setup Instructions**
1. **Clone the repository:**
    ```bash
        git clone https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin.git
    ```
2. **Install dependencies:**
    ```bash
        cargo build
    ```
3. Start the SurrealDB instance:
In the Project Directory open terminal perform below command!
    ```bash
    surreal start file:dashboard.db --user root --pass root 
    ```
4. Start project
    ```bash
        cargo leptos watch
    ```
5. Open the browser and navigate to `http://localhost:3000` to see the dashboard.
---
## **Learning Resources**
- [Leptos Documentation](https://book.leptos.dev/)
- [Actix Web Documentation](https://docs.rs/actix-web/latest/actix_web/)
- [SurrealDB Documentation](https://surrealdb.com/docs/)
---
## **Screenshots Project pages**
### 1. **Dashboard Page**
![dashboard](https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin/blob/main/assets/dashboard_page.png)
### 2. **Team Page**
![team_page](https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin/blob/main/assets/team_page.png)
### 3. **Add Person Page**
![Add Person page](https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin/blob/main/assets/add_person.png)
### 4. **Edit Person Page**
![Add person page](https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin/blob/main/assets/edit_person.png)
### 5. **Delete & Info page**
![Delete & info page](https://github.com/dev-dhanushkumar/leptos_leptos_portfolio_admin/blob/main/assets/info_person.png)
