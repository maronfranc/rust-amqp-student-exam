use amiquip::{
    Channel, Connection, Delivery, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions,
};
use sqlx::PgPool;

use crate::application::dtos::start_exam_dto::StartExamDto;
use crate::application::reply_to;
use crate::application::utils::get_student_exam_queue_names;
use crate::domain::services::{exam_service, student_exam_service};

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
    let student_exam_dto: StartExamDto = serde_json::from_str(&body).unwrap();
    let id_student_exam: i32 = student_exam_service::insert(
        &pool,
        student_exam_dto.data.id_student,
        student_exam_dto.data.id_exam,
    )
    .await;
    let (exchange_name, queue_name, routing_key) =
        get_student_exam_queue_names(student_exam_dto.data.id_student, id_student_exam);
    create_exam_queue(connection, exchange_name, queue_name, routing_key);
    let exam_template =
        exam_service::find_exam_template_by_id(&pool, student_exam_dto.data.id_exam).await;
    let buffer_exam_template = serde_json::to_vec(&exam_template).unwrap();
    reply_to::rpc(&delivery, &channel, &buffer_exam_template)
        .await
        .unwrap();
}
