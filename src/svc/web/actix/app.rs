use actix_web::{web, Responder};

use crate::ctx::SharedState;

pub async fn index(state: web::Data<SharedState>) -> impl Responder {
    // use your state:
    let server_name = &state.my_name();
    format!("hello from {}", server_name)
}

pub async fn ping() -> impl Responder {
    "pong"
}

pub(crate) fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index)).route("/ping", web::get().to(ping));
}

