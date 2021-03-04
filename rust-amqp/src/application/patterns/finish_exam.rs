use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, QueueDeleteOptions,
};
use sqlx::PgPool;
use std::env::var;

use crate::application::dtos::answer_question_dto::AnswerQuestionData;
use crate::application::dtos::finish_exam_dto::FinishExamDto;
use crate::infrastructure::repositories::answer_repository;

pub async fn finish_exam(body: std::borrow::Cow<'_, str>, pool: &mut PgPool) {
    let amqp_url: String = var("AMQP_URL").expect("AMQP_URL is not set");
    let finish_exam: FinishExamDto = serde_json::from_str(&body).unwrap();
    println!("{:#?}", finish_exam);
    let exchange_name = "e_exam";
    let queue_name = format!("q_exam_{}", finish_exam.data.id_exam.to_string());
    let routing_key = format!("r_exam_{}", finish_exam.data.id_exam.to_string());
    let mut connection = match Connection::insecure_open(&amqp_url) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };
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
    let consumer = queue
        .consume(ConsumerOptions {
            exclusive: true,
            no_ack: true,
            ..ConsumerOptions::default()
        })
        .unwrap();
    let total_queue_messages = queue.declared_message_count().unwrap() as usize;
    for (ii, message) in consumer.receiver().iter().enumerate() {
        match (ii, message) {
            (ii, ConsumerMessage::Delivery(message)) => {
                let answer: AnswerQuestionData = serde_json::from_slice(&message.body).unwrap();
                answer_repository::insert(&pool, &answer).await.unwrap();

                if ii + 1 == total_queue_messages {
                    queue
                        .delete(QueueDeleteOptions {
                            if_unused: false,
                            if_empty: true,
                        })
                        .unwrap();
                    break;
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close().unwrap();
}
