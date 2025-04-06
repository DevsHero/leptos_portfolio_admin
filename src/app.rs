pub mod components;
pub mod models;
pub mod pages;
pub mod server;
pub mod utils;
pub mod constants;
use constants::constant::INTRO_LOGO_URL;
use leptos::*;
use leptos_meta::*;
use leptos_router::{ Route, Router, Routes, SsrMode };
use leptos_toaster::Toaster;
use pages::{ HomePage, EditPage };
use server::api::site_config;
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let get_config = Resource::new(
        || (),
        move |_| async move { site_config().await }
    );
    let config = get_config.get().and_then(Result::ok).unwrap_or_default().clone();
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-portfolio-admin.css"/>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.7.2/css/all.min.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css" />
        <Title text={config.title}/>
        <link data-trunk rel="icon" href="public/favicon.ico" />
        <Router>
            <main class="layout">
            <Toaster 
	    position=leptos_toaster::ToasterPosition::BottomCenter
	>
                <Routes>
                    <Route path="/"  ssr=SsrMode::Async  view=move || {
                        view! {
                            <HomePage />
                        }
                    }/>
                 
                    <Route path="/edit" ssr=SsrMode::Async view=move || {
                        view! {
                            <EditPage />
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>

                </Toaster>
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
     
 <img
alt="loading "
src=   {INTRO_LOGO_URL}
style="margin-top : 1rem;"
class="logo-animate animate-spin-slow  "
/>
<div class="notFound">   Url Link Not Found  </div> 
        
    }
}
