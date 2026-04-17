use super::models::requests::*;
use super::models::responses::*;

use actix_web::{web, Responder};
use dfe::distribuicao::{
    CienciaOperacao, ConfirmacaoOperacao, Consulta, ConsultaChaveAcesso, ConsultaNSU,
    DesconhecimentoOperacao, DistribuicaoResposta, ManifestacaoResposta, OperacaoNaoRealizada,
};
pub struct DistribuicaoServices;

impl DistribuicaoServices {
    pub async fn consultar(payload: web::Json<ReqConsulta>) -> impl Responder {
        let response: DistribuicaoResposta = match Consulta::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .uf(payload.uf)
            .ambiente(payload.ambiente)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespConsulta {
                    error: 1,
                    msg: format!("Erro ao consultar distribuição: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespConsulta {
            error: 0,
            msg: "Consulta realizada com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn consultar_ultnsu(payload: web::Json<ReqConsultaUltNSU>) -> impl Responder {
        let response: DistribuicaoResposta = match ConsultaNSU::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .uf(payload.uf)
            .ambiente(payload.ambiente)
            .nsu(&payload.nsu)
            .check_flag()
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespConsulta {
                    error: 1,
                    msg: format!("Erro ao consultar por NSU: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespConsulta {
            error: 0,
            msg: "Consulta por NSU realizada com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn consultar_chave_acesso(
        payload: web::Json<ReqConsultaChaveAcesso>,
    ) -> impl Responder {
        let response: DistribuicaoResposta = match ConsultaChaveAcesso::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .uf(payload.uf)
            .ambiente(payload.ambiente)
            .chave_acesso(&payload.chave_acesso)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespConsulta {
                    error: 1,
                    msg: format!("Erro ao consultar por chave de acesso: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespConsulta {
            error: 0,
            msg: "Consulta por chave de acesso realizada com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn ciencia_operacao(payload: web::Json<ReqManifestacao>) -> impl Responder {
        let response: ManifestacaoResposta = match CienciaOperacao::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .ambiente(payload.ambiente)
            .chave_acesso(&payload.chave_acesso)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespManifestacao {
                    error: 1,
                    msg: format!("Erro ao enviar ciência da operação: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespManifestacao {
            error: 0,
            msg: "Ciência da operação enviada com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn confirmar_operacao(payload: web::Json<ReqManifestacao>) -> impl Responder {
        let response: ManifestacaoResposta = match ConfirmacaoOperacao::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .ambiente(payload.ambiente)
            .chave_acesso(&payload.chave_acesso)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespManifestacao {
                    error: 1,
                    msg: format!("Erro ao confirmar operação: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespManifestacao {
            error: 0,
            msg: "Confirmação da operação enviada com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn desconhecimento_operacao(payload: web::Json<ReqManifestacao>) -> impl Responder {
        let response: ManifestacaoResposta = match DesconhecimentoOperacao::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .ambiente(payload.ambiente)
            .chave_acesso(&payload.chave_acesso)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespManifestacao {
                    error: 1,
                    msg: format!("Erro ao enviar desconhecimento da operação: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespManifestacao {
            error: 0,
            msg: "Desconhecimento da operação enviado com sucesso".to_string(),
            data: Some(response),
        })
    }

    pub async fn nao_realizada(payload: web::Json<ReqOperacaoNaoRealizada>) -> impl Responder {
        let response: ManifestacaoResposta = match OperacaoNaoRealizada::new()
            .cert_path(&payload.cert_path)
            .cert_pass(&payload.cert_pass)
            .cnpj(&payload.cnpj)
            .ambiente(payload.ambiente)
            .chave_acesso(&payload.chave_acesso)
            .justificativa(&payload.justificativa)
            .send()
            .await
        {
            Ok(res) => res,
            Err(err) => {
                return web::Json(RespManifestacao {
                    error: 1,
                    msg: format!("Erro ao enviar operação não realizada: {}", err),
                    data: None,
                });
            }
        };

        web::Json(RespManifestacao {
            error: 0,
            msg: "Operação não realizada enviada com sucesso".to_string(),
            data: Some(response),
        })
    }
}
