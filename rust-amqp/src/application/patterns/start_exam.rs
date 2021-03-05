use amiquip::{
    AmqpProperties, Channel, Connection, Delivery, Exchange, ExchangeDeclareOptions, ExchangeType,
    FieldTable, Publish, QueueDeclareOptions,
};
use sqlx::PgPool;

use crate::application::dtos::student_exam_dto::StudentExamDto;
use crate::application::utils::get_student_exam_queue_names;
use crate::domain::services::exam_service;

const RPC_ERROR: &str = "Received delivery without reply_to or correlation_id";
const RPC_SUCCESS: &str = "Exam data publish to reply-to queue";
const NON_PERSISTENT_MESSAGE: u8 = 1;

pub async fn rpc(
    delivery: &Delivery,
    channel: &Channel,
    pool: &PgPool,
    student_exam_dto: &StudentExamDto,
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
    let exam =
        exam_service::find_exam_template_by_id(&pool, student_exam_dto.data.id_student_exam).await;
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
    exchange_name: String,
    queue_name: String,
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
    let student_exam_dto: StudentExamDto = serde_json::from_str(&body).unwrap();
    let (exchange_name, queue_name, routing_key) = get_student_exam_queue_names(
        student_exam_dto.data.id_student,
        student_exam_dto.data.id_student_exam,
    );
    exam_service::insert(
        &pool,
        student_exam_dto.data.id_student_exam,
        student_exam_dto.data.id_student,
    )
    .await;
    create_exam_queue(connection, exchange_name, queue_name, routing_key);
    rpc(&delivery, &channel, &pool, &student_exam_dto)
        .await
        .unwrap();
}
