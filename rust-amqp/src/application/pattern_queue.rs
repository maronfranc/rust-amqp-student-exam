use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};
use sqlx::PgPool;

use crate::application::{dtos, patterns};

pub async fn pattern_queue(connection: &mut Connection, pool: &mut PgPool) {
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
                let pattern_dto: dtos::pattern_dto::PatternDto =
                    serde_json::from_str(&body).unwrap();
                println!("{:#?}", pattern_dto);
                if pattern_dto.pattern == "start_exam" {
                    patterns::start_exam::start_exam(connection, &channel, &pool, &delivery, body)
                        .await;
                } else if pattern_dto.pattern == "answer_question" {
                    patterns::answer_question::answer_question(connection, body, pool);
                } else if pattern_dto.pattern == "finish_exam" {
                    patterns::finish_exam::finish_exam(body, pool).await;
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
