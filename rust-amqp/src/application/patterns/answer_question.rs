use amiquip::{
    AmqpProperties, Connection, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish,
};
use serde_json;
use sqlx::PgPool;

use crate::application::common::http_status;
use crate::application::common::response_helper;
use crate::application::dtos::answer_question_dto::AnswerQuestionDto;
use crate::application::utils::get_student_exam_queue_names;
use crate::domain::services::student_exam_service;

const PERSISTENT_MESSAGE: u8 = 2;

pub async fn answer_question(
    connection: &mut Connection,
    body: std::borrow::Cow<'_, str>,
    pool: &mut PgPool,
) -> Result<Vec<u8>, Vec<u8>> {
    let answer_question: AnswerQuestionDto = match serde_json::from_str(&body) {
        Ok(dto) => dto,
        Err(error) => {
            let error_message = format!("{}", error);
            return Err(response_helper::to_vec(
                http_status::BAD_REQUEST,
                error_message,
            ));
        }
    };
    println!("{:#?}", answer_question);
    let exam_exists =
        student_exam_service::find_finished_date_by_id(&pool, answer_question.data.id_student_exam)
            .await
            .unwrap();
    if exam_exists.finished_at.is_some() {
        return Err(response_helper::to_vec(
            http_status::UNPROCESSABLE_ENTITY,
            String::from("Exam already finished"),
        ));
    };
    let (exchange_name, _, routing_key) = get_student_exam_queue_names(
        answer_question.data.id_student,
        answer_question.data.id_student_exam,
    );
    let channel = connection.open_channel(None).unwrap();
    let exchange = channel
        .exchange_declare(
            ExchangeType::Direct,
            exchange_name,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: true,
                internal: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();
    let answer_question_bytes = serde_json::to_vec(&answer_question.data).unwrap();
    exchange
        .publish(Publish::with_properties(
            &answer_question_bytes,
            routing_key,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
    channel.close().unwrap();
    Ok(response_helper::to_vec(
        http_status::OK,
        String::from("Question answered"),
    ))
}
