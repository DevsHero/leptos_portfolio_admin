# Leptos Porfolio Admin
## Full Stack SSR Pure Rust None Javascript  

## **Overview**

This project we are build a modern full-stack Portfolio application using Rust. It combines:

- **Leptos** for the frontend, providing interactive and responsive user interfaces.
- **Actix Web** for the backend, ensuring high-performance and robust API handling.
- **SurrealDB** for managing and querying complex data with ease.

## **Features**

- **User-friendly Portfolio:** Displays key statistics like team size and monthly costs.
- **Team management:**
  - Add new team members with details like name, title, level, and compensation.
  - Edit existing team member information.
  - Delete team members from the system.
- **Data visualization:** Includes a simple bar chart to visualize team member distribution by role.
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
  - Example: When a user visits the Portfolio, the server generates the initial view before the browser loads additional interactivity.
- **Client-Side Rendering (CSR):**
  - Renders the application on the client’s browser using JavaScript or WebAssembly.
  - Benefits: Rich interactivity and smoother navigation without reloading the page.
  - Example: After the Portfolio loads, any user interaction (e.g., adding data) is handled dynamically in the browser.

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

- Stores user data and Portfolio information.
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
   `bash
    surreal start file:Portfolio.db --user root --pass root 
    `
4. Start project
   ```bash
       cargo leptos watch
   ```
5. Open the browser and navigate to `http://localhost:3000` to see the Portfolio.

---

## **Learning Resources**

- [Leptos Documentation](https://book.leptos.dev/)
- [Actix Web Documentation](https://docs.rs/actix-web/latest/actix_web/)
- [SurrealDB Documentation](https://surrealdb.com/docs/)

---
