pub mod components;
pub mod models;
pub mod pages;
pub mod server;
pub mod utils;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use pages::{ HomePage, EditPage };
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-portfolio-admin.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css" />
        <Title text="Full Stack Dashboard App"/>
 
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <HomePage />
                    
                        }
                    }/>
                 
                    <Route path="/edit" view=move || {
                        view! {
                            <EditPage />
                     
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    view! {
        <h1>"Not Found"</h1>
    }
}
