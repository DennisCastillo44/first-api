use actix_web::{middleware::from_fn, web};
pub use crate::controllers::person;
use crate::handlers;

pub fn person_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/person").
        route(web::get().to(person::persons)).
        route(web::post().to(person::register_person))
    ).service(web::resource("/person/{person_id}").
        wrap(from_fn(handlers::middleware::token_middleware)).
            route(web::get().to(person::get_person)).
            route(web::put().to(person::update_person)).
            route(web::delete().to(person::delete_person))
    );
}

