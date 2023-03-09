use crate::{utils::{LogType, log}, models::{BookRelationship}};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::{ConnectionManager}, SqliteConnection, };
mod books_relationships_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the books API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_book_relationship_handler(book_data: web::Json<BookRelationship>, pool: web::Data<DbPool>) -> HttpResponse {

    let result = books_relationships_dao::create_book_relationship(book_data, pool).await;
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
                .body("{err: 'Unable to insert book into database'")
        }
    }
}

pub async fn list_books_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_relationships_dao::list_book_relationships(pool).await;
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
    let result = books_relationships_dao::read_book_relationship_by_id(book_id, pool).await;
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

pub async fn update_book_handler(id: web::Path<String>, book_data: web::Json<BookRelationship>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = books_relationships_dao::update_book_relationship(id, book_data, pool).await;
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
    let result = books_relationships_dao::delete_book_relationship(id, pool).await;
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



