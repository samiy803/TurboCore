use serde::Serialize;
use sea_orm::entity::prelude::DateTime;

pub mod login;
pub mod refresh;
pub mod signup;
pub mod get_user;
pub mod util;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<'a> {
    ApiError {
        message: &'a str,
        error_code: &'a str,
    },
    SignupResponse {
        uid: String,
    },
    LoginResponse {
        uid: String,
        token: String,
        expiry: i64,
        refresh_token: String,
        email_verified: bool,
        metadata: String,
    },
    RefreshResponse {
        uid: String,
        access_token: String,
        refresh_token: String,
        expiry: i64,
    },
    UserResponse {
        uid: String,
        email: String,
        created_at: DateTime,
        updated_at: DateTime,
        last_login: Option<DateTime>,
        active: bool,
        metadata: Option<String>,
        email_verified: bool,
    }
}
