// Depdendencies
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;

// Modules import
mod utils;
mod tests;
mod guards;
use utils::*;
mod api_modules;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Load .env file and set initialization variables
    dotenv().ok();
    log(LogType::Info, "Starting server...".to_string());

    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create db connection pool with SQLite
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool<ConnectionManager<SqliteConnection>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server and register routes
    log(
        LogType::Info,
        "Starting server at http://localhost:8080".to_string(),
    );
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            // User class
            .route(
                "/create_user",
                web::post().to(api_modules::users_handlers::create_user_handler),
            )
            .route(
                "/login",
                web::post().to(api_modules::users_handlers::user_login_handler),
            )
            .route(
                "/logout",
                web::post().to(api_modules::users_handlers::user_logout_handler),
            )
            // Book class
            .route(
                "/create_book",
                web::post().to(api_modules::books_handlers::create_book_handler),
            )
            .route(
                "/list_books",
                web::get().to(api_modules::books_handlers::list_books_handler),
            )
            .route(
                "/get_book/{id}",
                web::post().to(api_modules::books_handlers::read_book_by_id_handler),
            )
            .route(
                "/update_book/{id}",
                web::put().to(api_modules::books_handlers::update_book_handler),
            )
            .route(
                "/delete_book/{id}",
                web::delete().to(api_modules::books_handlers::delete_book_handler),
            )
            // BookRelationships class
            .route(
                "/create_book_relationship",
                web::post().to(api_modules::books_relationships_handlers::create_book_relationship_handler),
            )
            .route(
                "/list_book_relationships",
                web::get().to(api_modules::books_relationships_handlers::list_books_handler),
            )
            .route(
                "/get_book_relationship/{id}",
                web::post().to(api_modules::books_relationships_handlers::read_book_by_id_handler),
            )
            .route(
                "/update_book_relationship/{id}",
                web::put().to(api_modules::books_relationships_handlers::update_book_handler),
            )
            .route(
                "/delete_book_relationship/{id}",
                web::delete().to(api_modules::books_relationships_handlers::delete_book_handler),
            )
            // Category class
            .route(
                "/create_category",
                web::post().to(api_modules::categories_handlers::create_category_handler),
            )
            .route(
                "/list_categories",
                web::get().to(api_modules::categories_handlers::list_categories_handler),
            )
            .route(
                "/get_category/{id}",
                web::post().to(api_modules::categories_handlers::read_category_by_id_handler),
            )
            .route(
                "/update_category/{id}",
                web::put().to(api_modules::categories_handlers::update_category_handler),
            )
            .route(
                "/delete_category/{id}",
                web::delete().to(api_modules::categories_handlers::delete_category_handler),
            )
            // Book Relationship Types Class
            .route(
                "/create_book_relationship_type",
                web::post().to(api_modules::book_relationship_types_handlers::create_book_relationship_handler),
            )
            .route(
                "/list_book_relationship_types",
                web::get().to(api_modules::book_relationship_types_handlers::list_books_handler),
            )
            .route(
                "/get_book_relationship_type/{id}",
                web::post().to(api_modules::book_relationship_types_handlers::read_book_by_id_handler),
            )
            .route(
                "/update_book_relationship_type/{id}",
                web::put().to(api_modules::book_relationship_types_handlers::update_book_handler),
            )
            .route(
                "/delete_book_relationship_type/{id}",
                web::delete().to(api_modules::book_relationship_types_handlers::delete_book_handler),
            )
            // Book Notes Class
            .route(
                "/create_book_note",
                web::post().to(api_modules::book_notes_handlers::create_book_note_handler),
            )
            .route(
                "/list_book_notes",
                web::get().to(api_modules::book_notes_handlers::list_book_notes_handler),
            )
            .route(
                "/get_book_note/{id}",
                web::post().to(api_modules::book_notes_handlers::read_book_note_by_id_handler),
            )
            .route(
                "/update_book_note/{id}",
                web::put().to(api_modules::book_notes_handlers::update_book_note_handler),
            )
            .route(
                "/delete_book_note/{id}",
                web::delete().to(api_modules::book_notes_handlers::delete_book_note_handler),
            )
            // Text Fragments Class
            .route(
                "/create_text_fragment",
                web::post().to(api_modules::text_fragments_handlers::create_text_fragment_handler),
            )
            .route(
                "/list_text_fragments",
                web::get().to(api_modules::text_fragments_handlers::list_text_fragments_handler),
            )
            .route(
                "/get_text_fragment/{id}",
                web::post().to(api_modules::text_fragments_handlers::read_text_fragment_by_id_handler),
            )
            .route(
                "/update_text_fragment/{id}",
                web::put().to(api_modules::text_fragments_handlers::update_text_fragment_handler),
            )
            .route(
                "/delete_text_fragment/{id}",
                web::delete().to(api_modules::text_fragments_handlers::delete_text_fragment_handler),
            )

            

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
