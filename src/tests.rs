
#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use diesel::{SqliteConnection, r2d2::ConnectionManager};
    pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
    use crate::{api_modules::{self, books_handlers}, models::*};
    use chrono::{self};
    use serde_json;
    use uuid::Uuid;
    use actix_web::web;
    use tokio_test::block_on;

    const TEST_BOOK_ID: &str = "0001-0002-0003-0004";

    #[test]
    pub fn api_test_add_book() {
        let book = Book {
            id: TEST_BOOK_ID.to_string(),
            user_id: "Test user id".to_string(),
            title: "Test book".to_string(),
            author: "Test author".to_string(),
            creation_date: Some(chrono::Utc::now().to_string()),
            publishing_house: Some("Test publishing house".to_string()),
            cover_image: Some("https://img.freepik.com/free-psd/book-cover-blue-background-mock-up_1390-101.jpg?w=996&t=st=1679908830~exp=1679909430~hmac=7c93929f277e9a04cc109017528be38ef43e7208192ec177afe353f3572c2786".to_string()),
            release_date: Some("03-10-2015".to_string())
    };
        let wrapped_json = web::Json(book);
        let wrapped_pool = web::Data::new(get_pool());
        let result = api_modules::books_handlers::create_book_handler(wrapped_json, wrapped_pool);
        // use Tokio to unwrap the future
        let http_response = block_on(result);
        assert_eq!(http_response.status(), 200, "Status should be 200");
    }



    #[test]
    pub fn api_test_get_books() {
        let wrapped_pool = web::Data::new(get_pool());
        let result = api_modules::books_handlers::list_books_handler(wrapped_pool);
        // use Tokio to unwrap the future
        let http_response = block_on(result);
        assert_eq!(http_response.status(), 200, "Status should be 200");
    }

    #[test]
   /*  
   pub fn api_test_get_book() {
        let wrapped_pool = web::Data::new(get_pool());
        let wrapped_id = web::Path<String>::(TEST_BOOK_ID.to_string());
        let result = api_modules::books_handlers::read_book_by_id_handler(TEST_BOOK_ID.to_string(), wrapped_pool);
        // use Tokio to unwrap the future
        let http_response = block_on(result);
        assert_eq!(http_response.status(), 200, "Status should be 200");
    }
*/
    #[test]
    pub fn api_test_update_book() {
        let book = Book {
            id: Uuid::new_v4().to_string(),
            user_id: "Test user id".to_string(),
            title: "Test book".to_string(),
            author: "Test author".to_string(),
            creation_date: Some(chrono::Utc::now().to_string()),
            publishing_house: Some("Test publishing house".to_string()),
            cover_image: None,
            release_date: Some("03-10-2015".to_string())
        };
        let wrapped_json = web::Json(book);
        let wrapped_pool = web::Data::new(get_pool());
        let result = api_modules::books_handlers::update_book_handler(TEST_BOOK_ID.to_string(), wrapped_json, wrapped_pool);
        // use Tokio to unwrap the future
        let http_response = block_on(result);
        assert_eq!(http_response.status(), 200, "Status should be 200");
    }

    #[test]
    pub fn api_test_delete_book() {
        let pool = get_pool();
        let result = books_handlers::delete_book_handler(TEST_BOOK_ID, pool);
        let http_response = block_on(result);
        assert_eq!(http_response.status(), 200, "Status should be 200");
    }

fn get_pool() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create db connection pool with SQLite
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}

}