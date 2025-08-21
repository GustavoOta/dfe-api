pub struct FieldsValidation;
use crate::nfe::entity::NFeApi;

impl FieldsValidation {
    pub fn request(nfe_api: &NFeApi) -> Result<(), String> {
        if nfe_api.cert_path.is_empty() {
            return Err("Certificado não informado.".to_string());
        }
        if nfe_api.cert_pass.is_empty() {
            return Err("Senha do certificado não informada.".to_string());
        }
        if nfe_api.ide.c_uf == 0 {
            return Err("UF não informada.".to_string());
        }
        if nfe_api.ide.serie == 0 {
            return Err("Série não informada.".to_string());
        }
        if nfe_api.ide.n_nf == 0 {
            return Err("Número da NF-e não informado.".to_string());
        }
        if nfe_api.emit.cnpj.is_empty() {
            return Err("CNPJ do emitente não informado.".to_string());
        }
        if nfe_api.emit.ie.is_empty() {
            return Err("Inscrição estadual do emitente não informada.".to_string());
        }
        if nfe_api.emit.x_nome.is_empty() {
            return Err("Nome do emitente não informado.".to_string());
        }
        if nfe_api.det.is_empty() {
            return Err("Nenhum item na NF-e.".to_string());
        } else {
            for item in &nfe_api.det {
                if item.x_prod.is_empty() {
                    return Err(format!(
                        "Descrição do produto não informada para o item: {:?}",
                        item
                    ));
                }

                if item.ncm.is_none() {
                    return Err(format!(
                        "NCM não informado para o produto:[{:?}] de cód:[{:?}]",
                        item.x_prod, item.c_prod
                    ));
                }
                if item.cfop.is_none() {
                    return Err(format!(
                        "CFOP não informado para o produto:[{:?}] de cód:[{:?}]",
                        item.x_prod, item.c_prod
                    ));
                }
            }
        }

        Ok(())
    }
}
