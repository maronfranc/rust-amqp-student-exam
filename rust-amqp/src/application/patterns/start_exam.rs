use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, FieldTable, QueueDeclareOptions};
use sqlx::PgPool;

use crate::application::common::http_status;
use crate::application::common::response_to_vec::response_to_vec;
use crate::application::dtos::start_exam_dto::StartExamDto;
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
    body: std::borrow::Cow<'_, str>,
    pool: &PgPool,
) -> Result<Vec<u8>, Vec<u8>> {
    let start_exam_dto: StartExamDto = match serde_json::from_str(&body) {
        Ok(dto) => dto,
        Err(error) => {
            let error_message = format!("{}", error);
            return Err(response_to_vec(
                http_status::INTERNAL_SERVER_ERROR,
                error_message,
                None,
            ));
        }
    };
    let id_student_exam: i32 = student_exam_service::insert(
        &pool,
        start_exam_dto.data.id_student,
        start_exam_dto.data.id_exam,
    )
    .await;
    let (exchange_name, queue_name, routing_key) =
        get_student_exam_queue_names(start_exam_dto.data.id_student, id_student_exam);
    create_exam_queue(connection, exchange_name, queue_name, routing_key);
    let exam_template =
        exam_service::find_exam_template_by_id(&pool, start_exam_dto.data.id_exam).await;
    let json_exam_template = serde_json::to_string(&exam_template).unwrap();
    let response_dto = response_to_vec(
        http_status::OK,
        String::from("Exam started"),
        Some(json_exam_template),
    );
    Ok(response_dto)
}
