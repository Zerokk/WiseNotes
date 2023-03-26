use crate::models::TextFragment;
use crate::schema::text_fragments;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for relationship types between books.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_text_fragments(pool: web::Data<DbPool>) -> Result<Vec<TextFragment>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

        text_fragments::table
        .load::<TextFragment>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn read_text_fragment_by_id(
    text_fragment_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<TextFragment, Error> {
    use crate::schema::text_fragments::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    text_fragments
        .filter(id.eq(text_fragment_id.into_inner()))
        .first::<TextFragment>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn create_text_fragment(
    text_fragment: web::Json<TextFragment>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(text_fragments::table)
        .values(text_fragment.into_inner())
        .execute(&mut conn)
}

pub async fn update_text_fragment(
    text_fragment_id: web::Path<String>,
    new_data: web::Json<TextFragment>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(text_fragments::table.find(text_fragment_id.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_text_fragment(
    text_fragment_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(text_fragments::table.find(text_fragment_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}
