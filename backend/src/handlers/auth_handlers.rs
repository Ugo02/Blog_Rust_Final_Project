use actix_web::{web, HttpResponse};
use bcrypt;
use sqlx::PgPool;
use crate::models::user::create_token;
use crate::models::log::{LoginRequest, LoginResponse, LogoutResponse, RegisterResponse, RegisterRequest};

pub async fn login_handler(
    pool: web::Data<PgPool>, // Database connection pool
    login_data: web::Json<LoginRequest>, // Login request data
) -> HttpResponse {
    // Fetch user from the database using the provided email
    let user = sqlx::query!(
        "SELECT id, email, password FROM users WHERE email = $1",
        login_data.email
    )
    .fetch_optional(pool.get_ref())
    .await;

    match user {
        Ok(Some(user)) => {
            // Verify the provided password against the hashed password in the database
            if bcrypt::verify(&login_data.password, &user.password).unwrap_or(false) {
                // Generate a JWT token for the authenticated user
                match create_token(user.id) {
                    Ok(token) => HttpResponse::Ok().json(LoginResponse {
                        success: true,
                        message: "Login successful!".into(),
                        token: Some(token),
                    }),
                    Err(e) => {
                        eprintln!("JWT generation error: {:?}", e);
                        HttpResponse::InternalServerError().json(LoginResponse {
                            success: false,
                            message: "Failed to generate token.".into(),
                            token: None,
                        })
                    }
                }
            } else {
                // Return unauthorized if the password is invalid
                HttpResponse::Unauthorized().json(LoginResponse {
                    success: false,
                    message: "Invalid email or password.".into(),
                    token: None,
                })
            }
        }
        Ok(None) => {
            // Return unauthorized if the email is not found
            HttpResponse::Unauthorized().json(LoginResponse {
                success: false,
                message: "Invalid email or password.".into(),
                token: None,
            })
        }
        Err(e) => {
            // Handle database errors
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(LoginResponse {
                success: false,
                message: "Failed to process login request.".into(),
                token: None,
            })
        }
    }
}

pub async fn logout_handler() -> HttpResponse {
    // Simple logout handler that returns a success message
    HttpResponse::Ok().json(LogoutResponse {
        success: true,
        message: "Logout successful!".into(),
    })
}


pub async fn register_handler(
    pool: web::Data<PgPool>, // Database connection pool
    new_user: web::Json<RegisterRequest>, // Registration request data
) -> HttpResponse {
    // Hash the user's password using bcrypt
    let hashed_password = match bcrypt::hash(&new_user.password, 4) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(RegisterResponse {
                success: false,
                message: "Failed to hash password.".into(),
                token: None,
            });
        }
    };

    // Insert the new user into the database
    let query = sqlx::query!(
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING id",
        new_user.first_name,
        new_user.last_name,
        new_user.email,
        hashed_password,
    );

    match query.fetch_one(pool.get_ref()).await {
        Ok(record) => {
            // Generate a JWT token for the newly registered user
            match create_token(record.id) {
                Ok(token) => HttpResponse::Ok().json(RegisterResponse {
                    success: true,
                    message: "User registered successfully!".into(),
                    token: Some(token),
                }),
                Err(e) => {
                    eprintln!("JWT generation error: {:?}", e);
                    HttpResponse::InternalServerError().json(RegisterResponse {
                        success: false,
                        message: "Failed to generate token.".into(),
                        token: None,
                    })
                }
            }
        },
        Err(e) => {
            // Handle database errors, including duplicate email
            if let Some(db_err) = e.as_database_error() {
                if db_err.code().as_deref() == Some("23505") {
                    return HttpResponse::BadRequest().json(RegisterResponse {
                        success: false,
                        message: "Email already exists.".into(),
                        token: None,
                    });
                }
            }
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json(RegisterResponse {
                success: false,
                message: "Failed to register user.".into(),
                token: None,
            })
        },
    }
}