use crate::{utils::{LogType, log}, models::BookNote};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::ConnectionManager, SqliteConnection, };
mod book_notes_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the book_notes API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_book_note_handler(book_note_data: web::Json<BookNote>, pool: web::Data<DbPool>) -> HttpResponse {

    let result = book_notes_dao::create_book_note(book_note_data, pool).await;
    match result {
        Ok(book_note) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book_note)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to insert book note into database'")
        }
    }
}

pub async fn list_book_notes_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_notes_dao::list_book_notes(pool).await;
    match result {
        Ok(book_notes) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book_notes)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to list all book notes from database'")
        }
    }
}

pub async fn read_book_note_by_id_handler(book_note_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_notes_dao::read_book_note_by_id(book_note_id, pool).await;
    match result {
        Ok(book_note) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book_note)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to read book note from database'")
        }
    }
}

pub async fn update_book_note_handler(id: web::Path<String>, book_note_data: web::Json<BookNote>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_notes_dao::update_book_note(id, book_note_data, pool).await;
    match result {
        Ok(book_note) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book_note)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to update book note in database'")
        }
    }
}

pub async fn delete_book_note_handler(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = book_notes_dao::delete_book_note(id, pool).await;
    match result {
        Ok(book_note) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&book_note)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to delete book note from database'")
        }
    }
}


