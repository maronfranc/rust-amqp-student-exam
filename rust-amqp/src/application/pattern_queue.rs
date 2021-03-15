use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};

use sqlx::PgPool;

use crate::application::common::http_status;
use crate::application::common::response_helper;
use crate::application::dtos::pattern_dto::PatternDto;
use crate::application::{patterns, reply_to};

const QUEUE_PATTERN_EXAM: &str = "q_patterns";
const PATTERN_START_EXAM: &str = "exam_started";
const PATTERN_ANSWER_QUESTION: &str = "question_answered";
const PATTERN_FINISH_EXAM: &str = "exam_finished";

async fn patterns(
    connection: &mut Connection,
    pool: &mut PgPool,
    body: std::borrow::Cow<'_, str>,
    pattern: String,
) -> Result<Vec<u8>, Vec<u8>> {
    if pattern == PATTERN_START_EXAM {
        match patterns::start_exam::start_exam(connection, body, pool).await {
            Ok(response) => return Ok(response),
            Err(error) => return Err(error),
        }
    } else if pattern == PATTERN_ANSWER_QUESTION {
        match patterns::answer_question::answer_question(connection, body, pool).await {
            Ok(response) => return Ok(response),
            Err(error) => return Err(error),
        };
    } else if pattern == PATTERN_FINISH_EXAM {
        match patterns::finish_exam::finish_exam(connection, body, pool).await {
            Ok(response) => return Ok(response),
            Err(error) => return Err(error),
        };
    }
    let error_message = format!("Pattern not implemented: {}", pattern);
    Err(response_helper::to_vec(
        http_status::NOT_IMPLEMENTED,
        error_message,
    ))
}

pub async fn rmq_listen(connection: &mut Connection, pool: &mut PgPool) {
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            QUEUE_PATTERN_EXAM,
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
