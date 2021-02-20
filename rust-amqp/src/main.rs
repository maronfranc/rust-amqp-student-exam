use amiquip::ConsumerOptions;
use amiquip::FieldTable;
use amiquip::QueueDeclareOptions;
use amiquip::{Connection, ConsumerMessage};

mod dtos;
mod patterns;

const URL: &str = "amqp://guest:guest@localhost:5672";

fn main() {
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };
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
                let start_exam_data: dtos::start_exam::StartExam =
                    serde_json::from_str(&body).unwrap();
                println!("{:#?}", start_exam_data);
                let exchange_name = "e_exam";
                let routing_key = format!("r_exam_{}", start_exam_data.data.id.to_string());
                let queue_name = format!("q_exam_{}", start_exam_data.data.id.to_string());
                if start_exam_data.pattern == "start_exam" {
                    patterns::create_queue::create_queue(
                        &mut connection,
                        &exchange_name,
                        &routing_key,
                        &queue_name,
                    );
                } else if start_exam_data.pattern == "answer_question" {
                    patterns::answer_question::answer_question(
                        &mut connection,
                        &exchange_name,
                        &routing_key,
                        start_exam_data,
                    );
                } else if start_exam_data.pattern == "finish_exam" {
                    patterns::finish_exam::finish_exam(&exchange_name, &routing_key, &queue_name);
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close().unwrap()
}
