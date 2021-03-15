use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDto<T> {
    pub code: u16,
    pub message: String,
    pub body: Option<T>,
}
