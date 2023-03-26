use crate::models::Category;
use crate::schema::categories;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/*
* DAO file for relationship types between books.
* This file contains the functions that are used to interact with the database.
*/

pub async fn list_categories(pool: web::Data<DbPool>) -> Result<Vec<Category>, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    categories::table
        .load::<Category>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn read_category_by_id(
    category_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<Category, Error> {
    use crate::schema::categories::dsl::*;
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    categories
        .filter(id.eq(category_id.into_inner()))
        .first::<Category>(&mut conn)
        .map_err(|e| e.into())
}

pub async fn create_category(
    category: web::Json<Category>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::insert_into(categories::table)
        .values(category.into_inner())
        .execute(&mut conn)
}

pub async fn update_category(
    category: web::Path<String>,
    new_data: web::Json<Category>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::update(categories::table.find(category.into_inner()))
        .set(new_data.into_inner())
        .execute(&mut conn)
        .map_err(|e| e.into())
}

pub async fn delete_category(
    category_id: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<usize, Error> {
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    diesel::delete(categories::table.find(category_id.into_inner()))
        .execute(&mut conn)
        .map_err(|e| e.into())
}
