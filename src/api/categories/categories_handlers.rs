use crate::{utils::{LogType, log}, models::{Category}};
use actix_web::{http::header::ContentType, web, HttpResponse};
use diesel::{r2d2::{ConnectionManager}, SqliteConnection, };
mod categories_dao;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the books API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 * 
 */


pub async fn create_category_handler(category_data: web::Json<Category>, pool: web::Data<DbPool>) -> HttpResponse {

    let result = categories_dao::create_category(category_data, pool).await;
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
                .body("{err: 'Unable to insert category into database'")
        }
    }
}

pub async fn list_categories_handler(pool: web::Data<DbPool>) -> HttpResponse {
    let result = categories_dao::list_categories(pool).await;
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
                .body("{err: 'Unable to list categories from database'")
        }
    }
}

pub async fn read_category_by_id_handler(category_id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = categories_dao::read_category_by_id(category_id, pool).await;
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
                .body("{err: 'Unable to read category from database'")
        }
    }
}

pub async fn update_category_handler(id: web::Path<String>, category_data: web::Json<Category>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = categories_dao::update_category(id, category_data, pool).await;
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
                .body("{err: 'Unable to update category in database'")
        }
    }
}

pub async fn delete_category_handler(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let result = categories_dao::delete_category(id, pool).await;
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
                .body("{err: 'Unable to delete category from database'")
        }
    }
}



