use crate::application::common::http_status;
use crate::application::dtos::response_dto::ResponseDto;

pub fn response_to_vec(status_code: u16, message: String, json_body: Option<String>) -> Vec<u8> {
    let response = ResponseDto {
        code: status_code,
        message: message,
        body: json_body,
    };
    let response_dto: Vec<u8> = match serde_json::to_vec(&response) {
        Ok(res) => res,
        Err(_error) => serde_json::to_vec(&ResponseDto {
            code: http_status::INTERNAL_SERVER_ERROR,
            message: String::from("Error converting body into response"),
            body: None,
        })
        .unwrap(),
    };
    response_dto
}
