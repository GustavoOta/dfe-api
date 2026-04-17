use actix_web::web;
pub mod handlers;
pub mod models;
pub mod services;

use crate::distribuicao::handlers::DistribuicaoHandlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/distribuicao").configure(DistribuicaoHandlers::routes));
}
