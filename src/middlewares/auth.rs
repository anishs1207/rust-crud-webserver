use crate::db::models::Claims;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    println!("jwt secret{}", jwt_secret);

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let decoded = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(jwt_secret.as_ref()),
                    &Validation::default(),
                );

                if decoded.is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
