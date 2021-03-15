use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDto {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBodyDto<T: Serialize> {
    pub code: u16,
    pub message: String,
    pub body: Option<T>,
}
