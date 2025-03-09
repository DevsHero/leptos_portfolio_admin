# Leptos Portfolio Admin
### Fullstack ssr pure rust portfolio site with admin system 

## **Overview**

This is all in one portfolio project that can manage any data from admin system.
My inspiration is anyone can update portfolio with dynamic data not just static data.
And anyone can learning from my scracth how to understand ssr and fullstack role.

## **Stack**
- **Leptos**: rust frontend with ssr framework .
- **Actix Web**: rust backend framework for API and other.
- **SurrealDB**: for storing data in database.
- **Tailwind**: css for ui.

## **Features**

- **Portfolio page:** 
  - Profile: name, age, nationality, gender ,job role
  - About me: support rich html text **(On development)**
  - Skills: multiple skill name with level
  - Contact: multiple dynamic contact that can customize icon 
  - Experiences: company logo , company name, position, work period, description
  - Portfolio: project name, project photos, is opensource, description, stacks 
- **Admin edit page:**
  - Permission Mode: for edit page have 2 mode can access edit page 
    - viewer mode can only view no password require.
    - admin mode can update the data and require password (password in .env) 
  - section Profile: edit profile, about me
  - section Skill: add or remove add Skill 
  - section Experience: add or remove Experience 
  - section Portfolio: add or remove Portfolio 
  - section Contact: add or remove Contact (The contact icon mapping from rust icondata now only have 7 icons if you need more just add in ICON_MAP utils 
  you can checkout this link https://carloskiki.github.io/icondata/ for more icon)  

- **Server:** Simple API support all connect method to surrealdb
- **Site Config:** 
  - site title - support now
  - meta tags -  on development
  - other seo tags - maybe
- **Darkmode:** now support darkmode (but not permanent setup yet on progress to store config localstorage)


## **How to run**

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
