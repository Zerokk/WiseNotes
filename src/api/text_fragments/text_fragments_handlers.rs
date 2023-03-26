use crate::{utils::{LogType, log}, models::{TextFragment}};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::{ConnectionManager}, SqliteConnection, };
mod text_fragments_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the books API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_text_fragment_handler(text_fragment_data: web::Json<TextFragment>, pool: web::Data<DbPool>) -> HttpResponse {

    let result = text_fragments_dao::create_text_fragment(text_fragment_data, pool).await;
    match result {
        Ok(text_fragment) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&text_fragment)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to insert text fragment into database'")
        }
    }
}

pub async fn list_text_fragments_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = text_fragments_dao::list_text_fragments(pool).await;
    match result {
        Ok(text_fragments) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&text_fragments)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to list text fragments from database'")
        }
    }
}

pub async fn read_text_fragment_by_id_handler(text_fragment_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = text_fragments_dao::read_text_fragment_by_id(text_fragment_id, pool).await;
    match result {
        Ok(text_fragment) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&text_fragment)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to read text fragment from database'")
        }
    }
}

pub async fn update_text_fragment_handler(id: web::Path<String>, text_fragment_data: web::Json<TextFragment>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = text_fragments_dao::update_text_fragment(id, text_fragment_data, pool).await;
    match result {
        Ok(text_fragment) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&text_fragment)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to update text fragment in database'")
        }
    }
}

pub async fn delete_text_fragment_handler(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = text_fragments_dao::delete_text_fragment(id, pool).await;
    match result {
        Ok(text_fragment) => {
            HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(&text_fragment)
        },
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to delete book from database'")
        }
    }
}



