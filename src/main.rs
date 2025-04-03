#![recursion_limit = "256"]
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos_portfolio_admin::app::*;
    use dotenvy::dotenv;
    use leptos::*;
    use leptos_actix::{ generate_route_list, LeptosRoutes };
    let env_result = dotenv();
    if env_result.is_err() {
        logging::warn!("There is no local development .env file");
    }
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);
    if cfg!(debug_assertions) {
        println!("Running in development mode");
    } else {
        println!("Running in production mode");
    }
    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;
        App::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
    })
        .bind(&addr)?
        .run().await
}
#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}
#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {}
#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use leptos_portfolio_admin::app::*;
    leptos::mount_to_body(App);
}
