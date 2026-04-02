use crate::pdf::services::PDFServices;
use actix_web::web;
pub struct PDFHandlers;

impl PDFHandlers {
    pub fn routes(cfg: &mut web::ServiceConfig) {
        // pdf/as-file
        cfg.route("/as-file", web::post().to(PDFServices::as_file));
        // pdf/as-base64
        cfg.route("/as-base64", web::post().to(PDFServices::as_base64));
    }
}
