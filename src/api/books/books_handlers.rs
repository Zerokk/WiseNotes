
use crate::{utils::{LogType, log}, models::Book};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::ConnectionManager, SqliteConnection, };
mod books_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the books API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_book_handler(book_data: web::Json<Book>, pool: web::Data<DbPool>) -> HttpResponse {
    println!("create_book_handler: {:#?}", book_data);
    
    if validate_book(&book_data) {
        let result = books_dao::create_book(book_data, pool).await;
        match result {
            Ok(book) => {
                println!("create_book_handler, OK. Book: {:#?}", book);
                HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(&book)
            },
            Err(err) => {
                println!("create_book_handler, ERROR: {:#?}", err);
                log(LogType::Error, err.to_string());
                HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .body("{err: 'Unable to insert book into database'")
            }
        }
    } else {
        println!("create_book_handler, ERROR: {:#?}", "Invalid book data");
        log(LogType::Error, "Invalid book data".to_string());
        HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body("{err: 'Invalid book data'")
    }
    
}

pub async fn list_books_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_dao::list_books(pool).await;
    match result {
        Ok(books) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&books)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to list books from database'")
        }
    }
}

pub async fn read_book_by_id_handler(book_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_dao::read_book_by_id(book_id, pool).await;
    match result {
        Ok(book) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to read book from database'")
        }
    }
}

pub async fn update_book_handler(id: web::Path<String>, book_data: web::Json<Book>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_dao::update_book(id, book_data, pool).await;
    match result {
        Ok(book) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to update book in database'")
        }
    }
}

pub async fn delete_book_handler(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_dao::delete_book(id, pool).await;
    match result {
        Ok(book) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to delete book from database'")
        }
    }
}



fn validate_book(book: &Book) -> bool {
    if book.title.is_empty() || book.author.is_empty(){
        return false
    }

    true
}
