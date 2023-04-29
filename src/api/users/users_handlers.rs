use crate::{
    models::{LoginData, User},
    utils::{log, LogType},
};
use actix_web::{cookie::Cookie, http::header::ContentType, web, HttpRequest, HttpResponse};
use diesel::{r2d2::ConnectionManager, SqliteConnection};
use uuid::Uuid;
pub mod users_dao;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/**
 * HTTP Handlers for the users API
 * This layer is responsible for handling the HTTP requests and responses,
 * keep log of the errors, and call the DAO layer to interact with the database.
 *
 */

pub async fn create_user_handler(
    user_data_json: web::Json<User>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    match unpack_and_validate_user(user_data_json) {
        Ok(user) => {
            let mut conn = pool
                .get()
                .expect("Failed to get database connection from pool");
            let result = users_dao::create_user(user, &mut conn);
            match result {
                Ok(user) => HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(&user),
                Err(err) => {
                    println!("create_user_handler, ERROR: {:#?}", err);
                    log(LogType::Error, err.to_string());
                    HttpResponse::InternalServerError()
                        .content_type(ContentType::json())
                        .body("{err: 'Unable to insert user into database'")
                }
            }
        }
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("{err: 'Unable to validate user data'")
        }
    }
}

pub async fn user_login_handler(
    login_data_json: web::Json<LoginData>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    match unpack_and_validate_login_data(login_data_json) {
        Ok(login_data) => {
            let mut conn = pool
                .get()
                .expect("Failed to get database connection from pool");
            let auth_token = generate_login_token();
            let result = users_dao::user_login(login_data, &auth_token, &mut conn);
            match result {
                Ok(user) => {
                    log(LogType::Info, format!("User {} logged in", &user.username));
                    HttpResponse::Ok()
                        .content_type(ContentType::json())
                        .cookie(
                            Cookie::build("user_id", &(user.id).to_string())
                                .secure(true)
                                .finish(),
                        )
                        .cookie(
                            Cookie::build("auth_token", &auth_token)
                                .secure(true)
                                .finish(),
                        )
                        .json(user)
                }

                Err(err) => {
                    log(LogType::Error, err.to_string());
                    HttpResponse::InternalServerError()
                        .content_type(ContentType::json())
                        .body("{err: 'Unable to insert user into database'")
                }
            }
        }
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("{err: 'Unable to validate user data'")
        }
    }
}

pub async fn user_logout_handler(pool: web::Data<DbPool>, req: HttpRequest) -> HttpResponse {
    // Extract data from the request cookies and get connection object
    let token_cookie = req.cookie("auth_token").unwrap();
    let user_id_cookie = req.cookie("user_id").unwrap();
    let token_stored = token_cookie.value();
    let user_id_stored = user_id_cookie.value();
    let mut conn = pool
        .get()
        .expect("Failed to get database connection from pool");

    // Check if the request provides an auth token and a user id, and if the user exists
    let found_user = users_dao::find_user_by_id(&user_id_stored.to_string(), &mut conn);
    if token_stored.is_empty() || user_id_stored.is_empty() || found_user.is_err() {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{err: 'Not logged in'");
    }

    // Check if the user is logged in with the given token
    let user = found_user.unwrap();
    if user.auth_token.as_ref().map(String::as_str) != Some(token_stored) {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("{err: 'Invalid token'");
    }

    let result = users_dao::user_logout(&user, &mut conn);
    match result {
        Ok(_) => {
            log(
                LogType::Info,
                format!("User {} logged out", &user.id.to_string()),
            );
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .cookie(Cookie::build("user_id", "").secure(true).finish())
                .cookie(Cookie::build("auth_token", "").secure(true).finish())
                .body("User logged out")
        }
        Err(err) => {
            log(LogType::Error, err.to_string());
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("{err: 'Unable to insert user into database'")
        }
    }
}
// Private functions

fn unpack_and_validate_user(json_user: web::Json<User>) -> Result<User, &'static str> {
    let user: User = json_user.into_inner();
    if user.username.is_empty() {
        return Err("Username is required");
    }
    if user.password.is_empty() {
        return Err("Password is required");
    }
    Ok(user)
}

fn unpack_and_validate_login_data(
    json_login: web::Json<LoginData>,
) -> Result<LoginData, &'static str> {
    let login: LoginData = json_login.into_inner();
    if login.username.is_empty() {
        return Err("Username is required");
    }
    if login.password.is_empty() {
        return Err("Password is required");
    }
    Ok(login)
}

fn generate_login_token() -> String {
    let mut timestamp = chrono::Utc::now().timestamp().to_string();
    timestamp.push_str("#");
    timestamp.push_str(&Uuid::new_v4().to_string());
    timestamp
}
