use serde::{Deserialize, Serialize};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, Validation, errors::Error as JwtError};
use chrono::{Utc, Duration};
use actix_web::{FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use futures::future::{ready, Ready};
use std::env;

// Claims structure for JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // User ID (subject)
    pub exp: usize, // Expiration timestamp
}

impl Claims {
    // Create new claims with a 24-hour expiration
    pub fn new(user_id: i32) -> Self {
        let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize; 
        Claims { sub: user_id, exp }
    }
}

// Generate a JWT token for a user
pub fn create_token(user_id: i32) -> Result<String, JwtError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); 
    let claims = Claims::new(user_id);
    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

// Decode a JWT token and extract claims
pub fn decode_token(token: &str) -> Result<Claims, JwtError> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); 
    jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
}

// Wrapper for user ID extracted from JWT
#[derive(Debug)]
pub struct UserId(pub i32);

// Implement `FromRequest` to extract `UserId` from the Authorization header
impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str[7..].trim(); // Extract token from "Bearer <token>"
                    match decode_token(token) {
                        Ok(claims) => return ready(Ok(UserId(claims.sub))), // Return user ID from claims
                        Err(e) => {
                            eprintln!("JWT error: {:?}", e);
                            return ready(Err(ErrorUnauthorized("Invalid token")));
                        }
                    }
                }
            }
        }

        ready(Err(ErrorUnauthorized("Missing or invalid Authorization header")))
    }
}