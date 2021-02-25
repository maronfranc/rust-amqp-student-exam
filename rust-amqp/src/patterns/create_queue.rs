use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::QueueDeclareOptions;
use amiquip::{Connection, ExchangeType};

use crate::dtos::start_exam_dto::StartExamDto;

pub fn create_queue(connection: &mut Connection, body: std::borrow::Cow<str>) {
    let create_queue: StartExamDto = serde_json::from_str(&body).unwrap();
    let exchange_name = "e_exam";
    let queue_name = format!("q_exam_{}", create_queue.data.id_exam.to_string());
    let routing_key = format!("r_exam_{}", create_queue.data.id_exam.to_string());
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                exclusive: false,
                auto_delete: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();
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
    queue
        .bind(&exchange, routing_key, FieldTable::default())
        .unwrap();
}
