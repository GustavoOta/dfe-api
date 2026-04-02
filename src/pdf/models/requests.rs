use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReqAsFile {
    pub xml: String,
    pub paper_size: String,
    pub as_file: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReqAsBase64 {
    pub xml: String,
    pub paper_size: String,
}
