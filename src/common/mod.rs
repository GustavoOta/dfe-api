pub fn doc_type(doc: &str) -> String {
    let doc = doc.chars().filter(|c| c.is_numeric()).collect::<String>();
    if doc.len() == 14 {
        "CNPJ".to_string()
    } else if doc.len() == 11 {
        "CPF".to_string()
    } else {
        "ESTRANGEIRO".to_string()
    }
}
