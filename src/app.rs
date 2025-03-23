pub mod components;
pub mod models;
pub mod pages;
pub mod server;
pub mod utils;
use components::Topbar;
use leptos::{ logging, prelude::* };
use leptos_meta::*;
use leptos_router::{ components::{ Route, Router, Routes }, StaticSegment };

use pages::{ EditPage, HomePage };
// use server::api::site_config;
use leptoaster::{ provide_toaster, Toaster };
#[component]
pub fn App() -> impl IntoView {
    // logging::log!("where do I run?");
    provide_meta_context();
    provide_toaster();
    // let get_config = Resource::new(
    //     || (),
    //     move |_| async move { site_config().await }
    // );
    // let config = get_config.get().and_then(Result::ok).unwrap_or_default().clone();
    view! {

        <Stylesheet id="leptos" href="/pkg/leptos-portfolio-admin.css"/>

        <Router>
        <Toaster/>
            <main class="layout">

        <Topbar/>
             <Routes fallback=move || view! { <NotFound /> }>
                    <Route path=StaticSegment("") view=HomePage/>

                    <Route path=StaticSegment("/edit") view=EditPage/>

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
