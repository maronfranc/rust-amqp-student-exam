use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};
use sqlx::PgPool;

use crate::{dtos, patterns};
pub fn pattern_queue(connection: &mut Connection, pool: &mut PgPool) {
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            "q_exam_pattern",
            QueueDeclareOptions {
                durable: true,
                exclusive: false,
                auto_delete: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();
    let consumer = queue
        .consume(ConsumerOptions {
            exclusive: true,
            no_ack: true,
            ..ConsumerOptions::default()
        })
        .unwrap();

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                let start_exam_data: dtos::pattern_dto::PatternDto =
                    serde_json::from_str(&body).unwrap();
                println!("{:#?}", start_exam_data);
                if start_exam_data.pattern == "start_exam" {
                    patterns::create_queue::create_queue(connection, body, pool);
                } else if start_exam_data.pattern == "answer_question" {
                    patterns::answer_question::answer_question(connection, body, pool);
                } else if start_exam_data.pattern == "finish_exam" {
                    patterns::finish_exam::finish_exam(body, pool);
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
