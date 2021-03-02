use amiquip::{
    AmqpProperties, Connection, ExchangeDeclareOptions, ExchangeType, FieldTable, Publish,
};
use serde_json;
use sqlx::PgPool;

use crate::dtos::answer_question_dto::AnswerQuestionDto;

const PERSISTENT_MESSAGE: u8 = 2;

pub fn answer_question(
    connection: &mut Connection,
    body: std::borrow::Cow<str>,
    pool: &mut PgPool,
) {
    let answer_question: AnswerQuestionDto = serde_json::from_str(&body).unwrap();
    println!("{:#?}", answer_question);
    let exchange_name = "e_exam";
    let routing_key = format!("r_exam_{}", answer_question.data.id_exam.to_string());
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
    let buffer_answer_question = serde_json::to_vec(&answer_question.data).unwrap();
    exchange
        .publish(Publish::with_properties(
            &buffer_answer_question,
            routing_key,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
    channel.close().unwrap();
}
