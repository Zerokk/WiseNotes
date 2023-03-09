use crate::models::Book;
use crate::schema::books;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for Book class.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_books(pool: web::Data<DbPool>) -> Result<Vec<Book>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    books::table.load::<Book>(&mut conn).map_err(|e| e.into())
}

pub async fn read_book_by_id(
    book_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<Book, Error> {
    use crate::schema::books::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    books
        .filter(id.eq(book_id.into_inner()))
        .first::<Book>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn create_book(book: web::Json<Book>, pool: web::Data<DbPool>) -> Result<usize, Error> {

    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(books::table)
        .values(book.into_inner())
        .execute(&mut conn)
}

pub async fn update_book(
    book_id: web::Path<String>,
    new_data: web::Json<Book>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(books::table.find(book_id.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_book(book_id: web::Path<String>, pool: web::Data<DbPool>) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(books::table.find(book_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}


