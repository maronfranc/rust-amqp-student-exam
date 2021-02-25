use amiquip::AmqpProperties;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::Publish;
use amiquip::{Connection, ExchangeType};
use serde_json::to_vec;

use crate::dtos::answer_question_dto::AnswerQuestionDto;
const PERSISTENT_MESSAGE: u8 = 2;

pub fn answer_question(connection: &mut Connection, body: std::borrow::Cow<str>) {
    let create_queue: AnswerQuestionDto = serde_json::from_str(&body).unwrap();
    let exchange_name = "e_exam";
    let routing_key = format!("r_exam_{}", create_queue.data.id_exam.to_string());
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
    let buffer_answer_question = to_vec(&create_queue).unwrap();
    exchange
        .publish(Publish::with_properties(
            &buffer_answer_question,
            routing_key,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
}
