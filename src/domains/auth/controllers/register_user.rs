use axum::{Json, extract::Extension, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use crate::utils::generate_tokens::generate_tokens;
use crate::utils::generate_tokens::User;
// utils import
// use crate::utils::error_handlers::coded_error_handlers::print_error;
use crate::utils::hashing_handler::hashing_handler;

// pub struct User {
//     pub id: i64,
//     pub email: String,
// }


#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserProfile {
    #[sqlx(rename = "id")]
    user_id: i64,
    full_name: String,
    email: String,
    profile_image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResponseCore {
    user_profile: UserProfile,
}

// ====== Response Data ======
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    response_message: String,
    response: Option<ResponseCore>,
    error: Option<String>,
}

pub async fn register_user(
    Extension(db_pool): Extension<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    let _user_email = payload.email;

    let tokens = match generate_tokens("auth", User { id: 3, email: _user_email.to_string() }).await {
        Ok(tokens) => tokens,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse {
                    response_message: "Failed to generate tokens".to_string(),
                    response: None,
                    error: Some(format!("Token generation error: {}", e)),
                }),
            );
        }
    };
    
    println!("Tokens generated: {:?}", tokens);

    let user_email = _user_email;

    // Hash the password
    let hashed_password = match hashing_handler(payload.password.as_str()).await {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(RegisterResponse {
                    response_message: "Failed to hash password".to_string(),
                    response: None,
                    error: Some(format!("Password hashing error: {}", e)),
                }),
            );
        }
    };

    // Check if email already exists
    let email_exists: Option<i64> =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
            .bind(&user_email)
            .fetch_optional(&db_pool)
            .await
            .unwrap_or(None)
            .flatten();

    if email_exists.unwrap_or(0) > 0 {
        return (
            StatusCode::FORBIDDEN,
            Json(RegisterResponse {
                response_message: "Registration failed".to_string(),
                response: None,
                error: Some("Email already exists".to_string()),
            }),
        );
    }

    // Insert user into database (schema: email, password, full_name, profile_image_url)
    let full_name = format!("{} {}", payload.first_name, payload.last_name);

    let result = sqlx::query_as::<_, UserProfile>(
        r#"
            INSERT INTO users (email, password, full_name, profile_image_url)
            VALUES ($1, $2, $3, $4)
            RETURNING id, full_name, email, profile_image_url
        "#,
    )
    .bind(&user_email)
    .bind(&hashed_password)
    .bind(&full_name)
    .bind(Option::<String>::None)
    .fetch_one(&db_pool)
    .await;

    match result {
        Ok(new_user) => (
            StatusCode::CREATED,
            Json(RegisterResponse {
                response_message: format!(
                    "User with email '{}' registered successfully!",
                    &user_email
                ),
                response: Some(ResponseCore {
                    user_profile: new_user,
                }),
                error: None,
            }),
        ),
        Err(e) => {
            // Handle unique constraint violations or other DB errors
            let error_msg =
                if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                    "Email already exists".to_string()
                } else {
                    format!("Database error: {}", e)
                };

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterResponse {
                    response_message: "Failed to register user".to_string(),
                    response: None,
                    error: Some(error_msg),
                }),
            )
        }
    }
}
