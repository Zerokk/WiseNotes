use actix_web::cookie::Cookie;
use actix_web::guard::{Guard, GuardContext};

use crate::models::User;
use crate::{utils::log, LogType};
use crate::api_modules::users_handlers::users_dao;
struct LoginCheck;

impl Guard for LoginCheck {
    fn check(&self, req: &GuardContext) -> bool {
        let auth_token = req.head().headers().get("auth_token");

        let user_id = Cookie::parse_header(req.head().headers().get("Cookie"))
            .unwrap()
            .contains_key("user_id");

        if(auth_token && user_id){
            let user = users_dao::user_login(auth_token, user_id);
            match(user){
                Ok(user) => {
                    if(user.auth_token == auth_token && user.id == user_id){
                        return true;
                    } else {
                        // Log caution error: user_id and auth_token don't match
                        log.log(LogType::Security, "User id and auth_token don't match: {} - {}", user_id, auth_token);
                        return false;
                    }
                },
                Err(err) => {
                    // Log error: user not found
                    log.log(LogType::Error, "User not found: {}", err);
                    return false;
                }
            }
        } else {
            return false;
        }
    }
}
