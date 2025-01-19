use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables and set up database connection
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Configure and start the HTTP server
    HttpServer::new(move || {
        // Set up CORS for frontend communication
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Content-Type", "Authorization"])
            .supports_credentials()
            .max_age(3600);

        // Define routes and middleware
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            // Authentication routes
            .route("/register", web::post().to(handlers::auth_handlers::register_handler))
            .route("/login", web::post().to(handlers::auth_handlers::login_handler))
            .route("/logout", web::post().to(handlers::auth_handlers::logout_handler))
            // Post-related routes
            .route("/create_post", web::post().to(handlers::post_handlers::create_post_handler))
            .route("/posts", web::get().to(handlers::post_handlers::get_posts_handler))
            .route("/posts/{id}", web::get().to(handlers::post_handlers::get_post_handler))
            .route("/user_posts", web::get().to(handlers::post_handlers::get_user_posts_handler))
            .route("/posts/{id}/edit", web::get().to(handlers::post_handlers::get_post_for_edit_handler))
            .route("/posts/{id}/update", web::put().to(handlers::post_handlers::update_post_handler))
    })
    .bind("127.0.0.1:8000")? // Bind to localhost
    .run() // Start the server
    .await
}