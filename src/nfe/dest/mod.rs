use crate::common::doc_type;
use crate::nfe::entity::DestApi;
use dfe::nfe::types::autorizacao4::Dest;

pub struct DestAPIBuilder;

impl DestAPIBuilder {
    pub fn process(dest: Option<DestApi>) -> Option<Dest> {
        let dest = if let Some(dest) = dest {
            dest
        } else {
            return None;
        };
        let doc_type = doc_type(dest.doc.clone());

        match doc_type.as_str() {
            "CPF" => Some(Dest {
                cpf: dest.doc.clone(),
                x_nome: dest.x_nome.clone(),
                x_lgr: dest.x_lgr.clone(),
                nro: dest.nro.clone(),
                x_bairro: dest.x_bairro.clone(),
                c_mun: dest.c_mun.clone(),
                x_mun: dest.x_mun.clone(),
                uf: dest.uf.clone(),
                cep: dest.cep.clone(),
                ind_ie_dest: dest.ind_ie_dest,
                ..Default::default()
            }),

            "CNPJ" => {
                if dest.ind_ie_dest == Some(9) || dest.ind_ie_dest == Some(2) {
                    Some(Dest {
                        cnpj: dest.doc.clone(),
                        x_nome: dest.x_nome.clone(),
                        x_lgr: dest.x_lgr.clone(),
                        nro: dest.nro.clone(),
                        x_bairro: dest.x_bairro.clone(),
                        c_mun: dest.c_mun.clone(),
                        x_mun: dest.x_mun.clone(),
                        uf: dest.uf.clone(),
                        cep: dest.cep.clone(),
                        ind_ie_dest: dest.ind_ie_dest,
                        ie: None,
                        ..Default::default()
                    })
                } else {
                    // remover caracteres especiais e espaços da inscricao estadual
                    let ie = dest.ie.as_ref().map(|s| {
                        s.replace(".", "")
                            .replace("/", "")
                            .replace("-", "")
                            .replace(" ", "")
                    });
                    if ie.is_none() || ie.as_ref().unwrap().is_empty() {
                        return None;
                    }
                    Some(Dest {
                        cnpj: dest.doc.clone(),
                        x_nome: dest.x_nome.clone(),
                        x_lgr: dest.x_lgr.clone(),
                        nro: dest.nro.clone(),
                        x_bairro: dest.x_bairro.clone(),
                        c_mun: dest.c_mun.clone(),
                        x_mun: dest.x_mun.clone(),
                        uf: dest.uf.clone(),
                        cep: dest.cep.clone(),
                        ind_ie_dest: dest.ind_ie_dest,
                        ie: ie,
                        ..Default::default()
                    })
                }
            }
            "ESTRANGEIRO" => Some(Dest {
                x_nome: dest.x_nome.clone(),
                x_lgr: dest.x_lgr.clone(),
                nro: dest.nro.clone(),
                x_bairro: dest.x_bairro.clone(),
                c_mun: dest.c_mun.clone(),
                x_mun: dest.x_mun.clone(),
                uf: dest.uf.clone(),
                cep: dest.cep.clone(),
                ind_ie_dest: dest.ind_ie_dest,
                ..Default::default()
            }),
            _ => None,
        }
    }
}
