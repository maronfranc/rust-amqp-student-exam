use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};

use sqlx::PgPool;

use crate::application::dtos::pattern_dto::PatternDto;
use crate::application::{patterns, reply_to};

async fn patterns<'a>(
    connection: &mut Connection,
    pool: &mut PgPool,
    body: std::borrow::Cow<'_, str>,
    pattern: String,
) -> Result<Vec<u8>, Vec<u8>> {
    if pattern == "exam_started" {
        match patterns::start_exam::start_exam(connection, &pool, body).await {
            Ok(exam_template) => return Ok(exam_template),
            Err(error) => return Err(error),
        }
    } else if pattern == "question_answered" {
        match patterns::answer_question::answer_question(connection, body) {
            Ok(exam_template) => return Ok(exam_template),
            Err(error) => return Err(error),
        };
    } else if pattern == "exam_finished" {
        match patterns::finish_exam::finish_exam(body, pool).await {
            Ok(exam_template) => return Ok(exam_template),
            Err(error) => return Err(error),
        };
    }

    Err("Pattern not implemented".as_bytes().to_vec())
}

pub async fn rmq_listen(connection: &mut Connection, pool: &mut PgPool) {
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            "q_patterns",
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
                let pattern_dto: PatternDto = match serde_json::from_str(&body) {
                    Ok(dto) => dto,
                    Err(error) => {
                        let e = format!("{}", error);
                        let error_buffer: &[u8] = e.as_bytes();
                        reply_to::rpc(&delivery, &channel, &error_buffer);
                        continue;
                    }
                };
                println!("{:#?}", pattern_dto);
                match patterns(connection, pool, body, pattern_dto.pattern).await {
                    Ok(data) => reply_to::rpc(&delivery, &channel, &data),
                    Err(error) => reply_to::rpc(&delivery, &channel, &error),
                };
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}
