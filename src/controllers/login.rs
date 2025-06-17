use actix_web::{http::header::ContentType, web, HttpResponse, Responder, Result};
use crate::{handlers::login::LoginUser, lib_::jwt_token::{JWTSigning, TokenResponse}};

pub async fn login(form: web::Json<LoginUser>) -> Result<impl Responder> {

    let auth = LoginUser::new(&form.username, &form.password);
    let user = auth.login()?;
    let token = JWTSigning::new(user.person_id, user.user_id).sign_token().unwrap();
    
    Ok(HttpResponse::Ok().insert_header(ContentType::json()).json(TokenResponse {
        token: token.token,
        session_datetime: token.session_datetime
    }))
}