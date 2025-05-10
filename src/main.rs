use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_cors::Cors;
use env_logger;
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


mod db;
mod api;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // Server
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "info");
    // }
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Database Configuration
    let pool  = db::connection::get_db_pool().await.expect("Failed To Connect");
    let openapi = api::docs::ApiDoc::openapi();

    HttpServer::new(move|| {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")  // Your server origin
            .allowed_origin("http://localhost:3000")  // Common frontend dev server
            .allowed_origin_fn(|origin, _req_head| {
                // Allow all localhost origins for development
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(Logger::default())
        .route("/",web::get().to(api::routes::index_page))
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
        .service(api::routes::create_blogpost)
        .service(api::routes::get_all_blogposts)
        .service(api::routes::read_blogpost)
        .service(api::routes::update_blogpost)
        .service(api::routes::delete_blogpost)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}