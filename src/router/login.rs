use actix_web::web;
use crate::controllers::login;

pub fn login_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login").
        route(web::post().to(login::login))
    );
}