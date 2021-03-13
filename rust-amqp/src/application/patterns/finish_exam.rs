use crate::infrastructure::models::FinishedAtModel;
use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, QueueDeleteOptions,
};
use sqlx::PgPool;

use crate::application::common::http_status;
use crate::application::common::response_to_vec::response_to_vec;
use crate::application::dtos::answer_question_dto::AnswerQuestionData;
use crate::application::dtos::finish_exam_dto::FinishExamDto;
use crate::application::utils::get_student_exam_queue_names;
use crate::domain::services::{student_answer_service, student_exam_service};

pub async fn finish_exam(
    connection: &mut Connection,
    body: std::borrow::Cow<'_, str>,
    pool: &mut PgPool,
) -> Result<Vec<u8>, Vec<u8>> {
    let finish_exam_dto: FinishExamDto = match serde_json::from_str(&body) {
        Ok(dto) => dto,
        Err(error) => {
            let error_message = format!("{}", error);
            return Err(response_to_vec(
                http_status::BAD_REQUEST,
                error_message,
                None,
            ));
        }
    };
    let exam_exists: FinishedAtModel =
        student_exam_service::find_finished_date_by_id(&pool, finish_exam_dto.data.id_student_exam)
            .await
            .unwrap();
    if exam_exists.finished_at.is_some() {
        return Err(response_to_vec(
            http_status::UNPROCESSABLE_ENTITY,
            String::from("Exam already finished"),
            None,
        ));
    };
    let (exchange_name, queue_name, routing_key) = get_student_exam_queue_names(
        finish_exam_dto.data.id_student,
        finish_exam_dto.data.id_student_exam,
    );
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
    let total_queue_messages = queue.declared_message_count().unwrap() as usize;
    if total_queue_messages != 0 {
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
        for (ii, message) in consumer.receiver().iter().enumerate() {
            match (ii, message) {
                (ii, ConsumerMessage::Delivery(message)) => {
                    let answer: AnswerQuestionData = serde_json::from_slice(&message.body).unwrap();
                    student_answer_service::insert(&pool, &answer).await;

                    if ii + 1 == total_queue_messages {
                        queue
                            .delete(QueueDeleteOptions {
                                if_unused: false,
                                if_empty: true,
                            })
                            .unwrap();
                        drop(consumer);
                        channel.close().unwrap();
                        student_exam_service::finish_student_exam(&pool, answer.id_student_exam)
                            .await;
                        return Ok(response_to_vec(
                            http_status::OK,
                            String::from("Exam finished"),
                            None,
                        ));
                    }
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
    }
    Err(response_to_vec(
        http_status::UNPROCESSABLE_ENTITY,
        String::from("Empty queue"),
        None,
    ))
}
