use dotenv::{self, Error};
use jwt::{header::HeaderType, AlgorithmType, Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::collections::BTreeMap;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTSigning {
    pub user_id: u32,
    pub person_id: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub session_datetime: String
}

impl JWTSigning {

    pub fn new(person_id: u32, user_id: u32) -> JWTSigning {
        JWTSigning{person_id, user_id}
    }

    pub fn sign_token(&self) -> Result<TokenResponse, Error> { //future_token: JWTSigning

        dotenv::dotenv().ok();
        let secret_key = dotenv::var("SECRET_TOKEN_KEY").expect("No se encontró una variable de ambiente");
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).unwrap();
        let session_datetime = Utc::now();
        let expiration_datetime = Utc::now() + Duration::hours(1);
        let header = Header {
            algorithm: AlgorithmType::Hs256,
            type_: Some(HeaderType::JsonWebToken),
            ..Default::default()
        };
        let mut claims = BTreeMap::new();
        claims.insert("person_id", self.person_id);
        claims.insert("user_id", self.user_id);
        claims.insert("exp", expiration_datetime.timestamp() as u32);

        let token = Token::new(header, claims).sign_with_key(&key).unwrap();

        Ok(TokenResponse {
            token: String::from(token.as_str()),
            session_datetime: session_datetime.timestamp().to_string()
        })
    }
}

/* pub struct JWTVerify {
    pub token: String
}

impl JWTVerify {
    pub fn verifyToken(token: JWTVerify) {

    }
} */