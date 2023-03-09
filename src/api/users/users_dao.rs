use crate::models::{LoginData, User};
use crate::schema::users;
use ::r2d2::PooledConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};
use diesel::result::Error;
use uuid::Uuid;

pub fn create_user(user_data: User, conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<usize, Error> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: user_data.username,
        password: user_data.password,
        email: user_data.email,
        creation_date: chrono::Utc::now().to_string(),
        auth_token: Some("".to_string()),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn user_login(
    user_data: LoginData,
    token: &String,
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let user: Result<User, Error> = users
        .filter(username.eq(user_data.username))
        .filter(password.eq(user_data.password))
        .first::<User>(conn)
        .map_err(|e| e.into())
        .map(|mut user| {
            user.auth_token = Some(token.to_string());
            user
        });

    user
}

pub fn user_logout(user: &User, conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<usize, Error> {

    change_auth_token(&user.id, &"".to_string(), conn)
}

pub fn change_auth_token(
    user_id: &String,
    token: &String,
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<usize, Error> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
        .set(auth_token.eq(Some(token.to_string())))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn find_user_by_id(user_id: &String, conn: &mut PooledConnection<ConnectionManager<SqliteConnection>> ) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .map_err(|e| e.into())
}
