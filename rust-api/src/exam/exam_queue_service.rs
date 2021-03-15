use crate::exam::dto::exam_dto::StudentExamDto;
use amiquip::{
    AmqpProperties, Channel, Connection, Consumer, ConsumerMessage, ConsumerOptions, Exchange,
    FieldTable, Publish, Queue, QueueDeclareOptions, Result,
};
use serde_json;
use std::env::var;
use uuid::Uuid;

use crate::exam::dto::answer_question_dto::{AnswerQuestionData, AnswerQuestionDto};
use crate::exam::dto::finish_exam_dto::{FinishExamData, FinishExamDto};
use crate::exam::dto::response_dto::ResponseDto;
use crate::exam::dto::start_exam_dto::{StartExamData, StartExamDto};

struct ExamQueueService<'a> {
    exchange: Exchange<'a>,
    queue: Queue<'a>,
    consumer: Consumer<'a>,
}

const NON_PERSISTENT_MESSAGE: u8 = 1;
const QUEUE_PATTERN_EXAM: &str = "q_patterns";
const PATTERN_START_EXAM: &str = "exam_started";
const PATTERN_ANSWER_QUESTION: &str = "question_answered";
const PATTERN_FINISH_EXAM: &str = "exam_finished";

impl<'a> ExamQueueService<'a> {
    fn new(channel: &Channel) -> Result<ExamQueueService> {
        let exchange = Exchange::direct(&channel);

        let queue = channel.queue_declare(
            "",
            QueueDeclareOptions {
                durable: false,
                exclusive: false,
                auto_delete: true,
                arguments: FieldTable::default(),
                ..QueueDeclareOptions::default()
            },
        )?;
        let consumer = queue.consume(ConsumerOptions {
            no_ack: true,
            ..ConsumerOptions::default()
        })?;

        Ok(ExamQueueService {
            exchange,
            queue,
            consumer,
        })
    }

    fn call(&self, body: &[u8]) -> Result<String, String> {
        let correlation_id = format!("{}", Uuid::new_v4());
        self.exchange
            .publish(Publish::with_properties(
                body,
                QUEUE_PATTERN_EXAM,
                AmqpProperties::default()
                    .with_reply_to(self.queue.name().to_string())
                    .with_correlation_id(correlation_id.clone())
                    .with_delivery_mode(NON_PERSISTENT_MESSAGE),
            ))
            .unwrap();
        for message in self.consumer.receiver().iter() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    if delivery.properties.correlation_id().as_ref() == Some(&correlation_id) {
                        return Ok(String::from_utf8_lossy(&delivery.body).into());
                    }
                }
                other => {
                    println!("Consumer ended: {:?}", other);
                    break;
                }
            }
        }
        Err("ERROR: server failed to respond to RPC call".to_string())
    }
}

pub fn send_start_exam(start_exam_dto: StartExamData) -> ResponseDto<StudentExamDto> {
    dotenv::dotenv().ok();
    let amqp_url: String = var("AMQP_URL").expect("AMQP_URL is not set");
    let mut connection = Connection::insecure_open(&amqp_url).unwrap();
    let channel = connection.open_channel(None).unwrap();
    let exam_queue_service = ExamQueueService::new(&channel).unwrap();
    let body = serde_json::to_vec(&StartExamDto {
        pattern: PATTERN_START_EXAM.to_string(),
        data: start_exam_dto,
    })
    .unwrap();
    let result = exam_queue_service.call(&body).unwrap();
    connection.close().unwrap();
    let exam_dto: ResponseDto<StudentExamDto> = serde_json::from_str(&result).unwrap();
    exam_dto
}

pub fn send_answer_question(answer_question: AnswerQuestionData) -> ResponseDto<String> {
    dotenv::dotenv().ok();
    let amqp_url: String = var("AMQP_URL").expect("AMQP_URL is not set");
    let mut connection = Connection::insecure_open(&amqp_url).unwrap();
    let channel = connection.open_channel(None).unwrap();
    let exam_queue_service = ExamQueueService::new(&channel).unwrap();
    let body = serde_json::to_vec(&AnswerQuestionDto {
        pattern: PATTERN_ANSWER_QUESTION.to_string(),
        data: answer_question,
    })
    .unwrap();
    let result = exam_queue_service.call(&body).unwrap();
    let response: ResponseDto<String> = serde_json::from_str(&result).unwrap();
    connection.close().unwrap();
    response
}

pub fn send_finish_exam(finish_exam: FinishExamData) -> ResponseDto<String> {
    dotenv::dotenv().ok();
    let amqp_url: String = var("AMQP_URL").expect("AMQP_URL is not set");
    let mut connection = Connection::insecure_open(&amqp_url).unwrap();
    let channel = connection.open_channel(None).unwrap();
    let exam_queue_service = ExamQueueService::new(&channel).unwrap();
    let body = serde_json::to_vec(&FinishExamDto {
        pattern: PATTERN_FINISH_EXAM.to_string(),
        data: finish_exam,
    })
    .unwrap();
    let result = exam_queue_service.call(&body).unwrap();
    let response: ResponseDto<String> = serde_json::from_str(&result).unwrap();
    connection.close().unwrap();
    response
}
