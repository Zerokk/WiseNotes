use crate::models::BookRelationshipType;
use crate::schema::book_relationship_types;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for book_relationship Relationships class.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_book_relationship_types(pool: web::Data<DbPool>) -> Result<Vec<BookRelationshipType>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

        book_relationship_types::table.load::<BookRelationshipType>(&mut conn).map_err(|e| e.into())
}

pub async fn read_book_relationship_types_by_id(
    book_relationship_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<BookRelationshipType, Error> {
    use crate::schema::book_relationship_types::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    // Function for reading a book_relationship by id
    book_relationship_types
        .filter(id.eq(book_relationship_id.into_inner()))
        .first::<BookRelationshipType>(&mut conn)
        .map_err(|e| e.into())

}

pub async fn create_book_relationship_type(book_relationship_type: web::Json<BookRelationshipType>, pool: web::Data<DbPool>) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(book_relationship_types::table)
        .values(book_relationship_type.into_inner())
        .execute(&mut conn)
}

pub async fn update_book_relationship_type(
    book_relationship_type_id: web::Path<String>,
    new_data: web::Json<BookRelationshipType>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(book_relationship_types::table.find(book_relationship_type_id.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_book_relationship_type(book_relationship_type_id: web::Path<String>, pool: web::Data<DbPool>) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(book_relationship_types::table.find(book_relationship_type_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}
