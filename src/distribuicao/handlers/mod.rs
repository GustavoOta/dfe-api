use crate::distribuicao::services::DistribuicaoServices;
use actix_web::web;
pub struct DistribuicaoHandlers;

impl DistribuicaoHandlers {
    pub fn routes(cfg: &mut web::ServiceConfig) {
        // distribuicao/consultar
        cfg.route("/consultar", web::get().to(DistribuicaoServices::consultar));
        // distribuicao/consultar-ultnsu
        cfg.route(
            "/consultar-ultnsu",
            web::get().to(DistribuicaoServices::consultar_ultnsu),
        );
        // distribuicao/consultar-chave-acesso
        cfg.route(
            "/consultar-chave-acesso",
            web::get().to(DistribuicaoServices::consultar_chave_acesso),
        );
        // distribuicao/ciencia-operacao
        cfg.route(
            "/ciencia-operacao",
            web::post().to(DistribuicaoServices::ciencia_operacao),
        );
        // distribuicao/confirmar-operacao
        cfg.route(
            "/confirmar-operacao",
            web::post().to(DistribuicaoServices::confirmar_operacao),
        );
        // distribuicao/desconhecimento-operacao
        cfg.route(
            "/desconhecimento-operacao",
            web::post().to(DistribuicaoServices::desconhecimento_operacao),
        );
        // distribuicao/nao-realizada
        cfg.route(
            "/operacao-nao-realizada",
            web::post().to(DistribuicaoServices::nao_realizada),
        );
    }
}
