use crate::{utils::{LogType, log}, models::{BookRelationshipType}};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::{ConnectionManager}, SqliteConnection, };
mod book_relationship_types_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the books API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_book_relationship_handler(book_data: web::Json<BookRelationshipType>, pool: web::Data<DbPool>) -> HttpResponse {

    let result = book_relationship_types_dao::create_book_relationship_type(book_data, pool).await;
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
                .body("{err: 'Unable to insert book relationship type into database'")
        }
    }
}

pub async fn list_books_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_relationship_types_dao::list_book_relationship_types(pool).await;
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
                .body("{err: 'Unable to list book relationship types from database'")
        }
    }
}

pub async fn read_book_by_id_handler(book_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_relationship_types_dao::read_book_relationship_types_by_id(book_id, pool).await;
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
                .body("{err: 'Unable to read book relationship type from database'")
        }
    }
}

pub async fn update_book_handler(id: web::Path<String>, book_data: web::Json<BookRelationshipType>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_relationship_types_dao::update_book_relationship_type(id, book_data, pool).await;
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
                .body("{err: 'Unable to update book relationship type in database'")
        }
    }
}

pub async fn delete_book_handler(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_relationship_types_dao::delete_book_relationship_type(id, pool).await;
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
                .body("{err: 'Unable to delete book relationship type from database'")
        }
    }
}



