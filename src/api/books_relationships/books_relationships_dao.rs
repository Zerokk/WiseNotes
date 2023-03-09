use crate::models::BookRelationship;
use crate::schema::book_relationships;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for book_relationship Relationships class.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_book_relationships(pool: web::Data<DbPool>) -> Result<Vec<BookRelationship>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    book_relationships::table.load::<BookRelationship>(&mut conn).map_err(|e| e.into())
}

pub async fn read_book_relationship_by_id(
    book_relationship_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<BookRelationship, Error> {
    use crate::schema::book_relationships::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    // Function for reading a book_relationship by id
        book_relationships
        .filter(id.eq(book_relationship_id.into_inner()))
        .first::<BookRelationship>(&mut conn)
        .map_err(|e| e.into())

    /*book_relationships
        .filter(id.eq(book_relationship_id.into_inner()))
        .first::<BookRelationship>(&mut conn)
        .map_err(|e| e.into())*/
}

pub async fn create_book_relationship(book_relationship: web::Json<BookRelationship>, pool: web::Data<DbPool>) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(book_relationships::table)
        .values(book_relationship.into_inner())
        .execute(&mut conn)
}

pub async fn update_book_relationship(
    book_relationship_id: web::Path<String>,
    new_data: web::Json<BookRelationship>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(book_relationships::table.find(book_relationship_id.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_book_relationship(book_relationship_id: web::Path<String>, pool: web::Data<DbPool>) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(book_relationships::table.find(book_relationship_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}
