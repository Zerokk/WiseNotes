// Depdendencies
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use utils::*;


// Modules import
mod utils;
#[path = "api/books/books_handlers.rs"]
mod books_handlers;
#[path = "api/books_relationships/books_relationships_handlers.rs"]
mod books_relationships_handlers;
#[path = "api/users/users_handlers.rs"]
mod users_handlers;
mod models;
mod routes;
mod schema;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and set initialization variables
    log(LogType::Info, "Starting server...".to_string());
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create db connection pool with SQLite
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server and register routes
    log(LogType::Info, "Starting server at http://localhost:8080".to_string());
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            // User class
            .route(
                "/create_user",
                web::post().to(users_handlers::create_user_handler),
            )
            .route(
                "/login",
                web::post().to(users_handlers::user_login_handler),
            )
            .route(
                "/logout",
                web::post().to(users_handlers::user_logout_handler),
            )
            // Book class
            .route(
                "/create_book",
                web::post().to(books_handlers::create_book_handler),
            )
            .route(
                "/list_books",
                web::get().to(books_handlers::list_books_handler),
            )
            .route(
                "/get_book/{id}",
                web::post().to(books_handlers::read_book_by_id_handler),
            )
            .route(
                "/update_book/{id}",
                web::put().to(books_handlers::update_book_handler),
            )
            .route(
                "/delete_book/{id}",
                web::delete().to(books_handlers::delete_book_handler),
            )
            // BookRelationships class
            .route(
                "/create_book_relationship",
                web::post().to(books_relationships_handlers::create_book_relationship_handler),
            )
            .route(
                "/list_book_relationships",
                web::get().to(books_relationships_handlers::list_books_handler),
            )
            .route(
                "/get_book_relationship/{id}",
                web::post().to(books_relationships_handlers::read_book_by_id_handler),
            )
            .route(
                "/update_book_relationship/{id}",
                web::put().to(books_relationships_handlers::update_book_handler),
            )
            .route(
                "/delete_book_relationship/{id}",
                web::delete().to(books_relationships_handlers::delete_book_handler),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
