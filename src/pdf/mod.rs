use actix_web::web;
pub mod handlers;
pub mod models;
pub mod services;

use crate::pdf::handlers::PDFHandlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/pdf").configure(PDFHandlers::routes));
}
