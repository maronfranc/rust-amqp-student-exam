use amiquip::AmqpProperties;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::Publish;
use amiquip::{Connection, ExchangeType};
use serde_json::to_vec;

use crate::dtos;
const PERSISTENT_MESSAGE: u8 = 2;

pub fn answer_question(
    connection: &mut Connection,
    exchange_name: &str,
    routing_key: &str,
    answer_question: dtos::start_exam::StartExam,
) {
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
    let buffer_answer_question = to_vec(&answer_question).unwrap();
    exchange
        .publish(Publish::with_properties(
            &buffer_answer_question,
            routing_key,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
}
