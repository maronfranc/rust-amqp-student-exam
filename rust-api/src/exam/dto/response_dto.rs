use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDto {
    pub code: u16,
    pub message: String,
    pub body: Option<String>,
}
