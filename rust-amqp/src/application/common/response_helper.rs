use serde::Serialize;

use crate::application::common::http_status;
use crate::application::dtos::response_dto::{ResponseBodyDto, ResponseDto};

pub fn body_to_vec<T: Serialize>(
    status_code: u16,
    message: String,
    serialize_body: Option<T>,
) -> Vec<u8> {
    let response = ResponseBodyDto::<T> {
        code: status_code,
        message: message,
        body: serialize_body,
    };
    let response_dto: Vec<u8> = match serde_json::to_vec(&response) {
        Ok(res) => res,
        Err(_error) => serde_json::to_vec(&ResponseBodyDto::<Option<T>> {
            code: http_status::INTERNAL_SERVER_ERROR,
            message: String::from("Error converting body into response"),
            body: None,
        })
        .unwrap(),
    };
    response_dto
}

pub fn to_vec(status_code: u16, message: String) -> Vec<u8> {
    let response = ResponseDto {
        code: status_code,
        message: message,
    };
    let response_dto: Vec<u8> = match serde_json::to_vec(&response) {
        Ok(res) => res,
        Err(_error) => serde_json::to_vec(&ResponseDto {
            code: http_status::INTERNAL_SERVER_ERROR,
            message: String::from("Error converting body into response"),
        })
        .unwrap(),
    };
    response_dto
}
