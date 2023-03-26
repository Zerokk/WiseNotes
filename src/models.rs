

use diesel::prelude::*;
use serde::{Serialize, Deserialize};    
use crate::schema::*;

/*
* Diesel models for the database tables.
*/

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset, Debug)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub user_id: String,
    pub creation_date: Option<String>,
    pub publishing_house: Option<String>,
    pub release_date: Option<String>,
    pub cover_image: Option<String>
}

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = book_relationships)]
pub struct BookRelationship {
    pub id: String,
    pub book_id: String,
    pub related_book_id: String,
    pub relationship_type: i32,
    pub user_id: String,
    pub comment: Option<String>,
    pub text_fragments: Option<String>,
}


// Struct for "user" class
#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub creation_date: String,
    pub auth_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}


#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent_category: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = book_relationship_types)]
pub struct BookRelationshipType {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = book_notes)]
pub struct BookNote {
    pub id: String,
    pub book_id: String,
    pub user_id: String,
    pub content: String,
    pub creation_date: String,
    pub header: Option<String>,
    pub relationships: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = text_fragments)]
pub struct TextFragment {
    pub id: String,
    pub book_id: String,
    pub user_id: String,
    pub text: String,
    pub creation_date: String,
}