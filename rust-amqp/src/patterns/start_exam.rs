use amiquip::{
    AmqpProperties, Channel, Connection, Delivery, Exchange, ExchangeDeclareOptions, ExchangeType,
    FieldTable, Publish, QueueDeclareOptions,
};

use crate::dtos::start_exam_dto::StartExamDto;
use crate::repositories::exam_repository;
use crate::services::exam_service;
use sqlx::PgPool;

const RPC_ERROR: &str = "Received delivery without reply_to or correlation_id";
const RPC_SUCCESS: &str = "Exam data publish to reply-to queue";
const NON_PERSISTENT_MESSAGE: u8 = 1;

pub fn deserialized_data_names(start_exam_dto: &StartExamDto) -> (String, String, String) {
    let queue_name = format!("q_exam_{}", start_exam_dto.data.id_exam.to_string());
    let exchange_name = String::from("e_exam");
    let routing_key = format!("r_exam_{}", start_exam_dto.data.id_exam.to_string());

    return (queue_name, exchange_name, routing_key);
}

pub async fn rpc(
    delivery: &Delivery,
    channel: &Channel,
    pool: &PgPool,
    start_exam_dto: &StartExamDto,
) -> Result<&'static str, &'static str> {
    let exchange = Exchange::direct(&channel);
    let (reply_to, corr_id) = match (
        delivery.properties.reply_to(),
        delivery.properties.correlation_id(),
    ) {
        (Some(r), Some(c)) => (r.clone(), c.clone()),
        _ => {
            println!("received delivery without reply_to or correlation_id");
            return Err(RPC_ERROR);
        }
    };
    let exam = exam_service::find_exam_template_by_id(&pool, start_exam_dto.data.id_exam).await;
    let buffer_exam = serde_json::to_vec(&exam).unwrap();
    exchange
        .publish(Publish::with_properties(
            &buffer_exam,
            reply_to,
            AmqpProperties::default()
                .with_correlation_id(corr_id)
                .with_delivery_mode(NON_PERSISTENT_MESSAGE),
        ))
        .unwrap();
    Ok(RPC_SUCCESS)
}

fn create_exam_queue(
    connection: &mut Connection,
    queue_name: String,
    exchange_name: String,
    routing_key: String,
) {
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

    channel.close().unwrap();
}

pub async fn start_exam(
    connection: &mut Connection,
    channel: &Channel,
    pool: &PgPool,
    delivery: &Delivery,
    body: std::borrow::Cow<'_, str>,
) {
    let start_exam_dto: StartExamDto = serde_json::from_str(&body).unwrap();
    let (queue_name, exchange_name, routing_key) = deserialized_data_names(&start_exam_dto);
    exam_repository::insert(
        &pool,
        start_exam_dto.data.id_exam,
        start_exam_dto.data.id_student,
    )
    .await
    .unwrap();
    create_exam_queue(connection, queue_name, exchange_name, routing_key);
    rpc(&delivery, &channel, &pool, &start_exam_dto)
        .await
        .unwrap();
}
