use crate::models::BookNote;
use crate::schema::book_notes;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for Book Notes class.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_book_notes(pool: web::Data<DbPool>) -> Result<Vec<BookNote>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    book_notes::table
        .load::<BookNote>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn read_book_note_by_id(
    book_note_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<BookNote, Error> {
    use crate::schema::book_notes::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    book_notes
        .filter(id.eq(book_note_id.into_inner()))
        .first::<BookNote>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn create_book_note(
    book_note: web::Json<BookNote>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(book_notes::table)
        .values(book_note.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn update_book_note(
    book_note_id: web::Path<String>,
    new_data: web::Json<BookNote>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(book_notes::table.find(book_note_id.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_book_note(
    book_note_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(book_notes::table.find(book_note_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}
