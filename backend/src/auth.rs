use axum::headers::authorization::Credentials;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{api::UserId, error::AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: UserId,
    exp: i64,
}

pub fn generate_jwt(user_id: UserId, key: &EncodingKey) -> AppResult<String> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp();

    let claims = Claims { user_id, exp };

    let token = encode(&Header::new(Algorithm::RS384), &claims, key)?;

    Ok(token)
}

pub fn verify_jwt(token: &str, key: &DecodingKey) -> AppResult<Claims> {
    let header = jsonwebtoken::decode_header(token)?;

    let claims =
        jsonwebtoken::decode::<Claims>(token, key, &jsonwebtoken::Validation::new(header.alg))?
            .claims;
    Ok(claims)
}

pub struct JWTToken(pub String);

impl Credentials for JWTToken {
    const SCHEME: &'static str = "Token";

    fn decode(value: &axum::http::HeaderValue) -> Option<Self> {
        let mut it = value.to_str().ok()?.split_whitespace();
        let scheme = it.next()?;
        let token = it.next()?;

        if scheme != Self::SCHEME || it.next().is_some() {
            None?
        }

        Some(Self(token.to_string()))
    }

    fn encode(&self) -> axum::http::HeaderValue {
        unreachable!()
    }
}
