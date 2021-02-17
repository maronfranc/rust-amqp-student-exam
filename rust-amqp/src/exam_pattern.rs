use amiquip::AmqpProperties;
use amiquip::ConsumerOptions;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::Publish;
use amiquip::QueueDeclareOptions;
use amiquip::{Connection, ConsumerMessage, ExchangeType};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;

const URL: &str = "amqp://guest:guest@localhost:5672";
const PERSISTENT_MESSAGE: u8 = 2;

#[derive(Serialize, Deserialize, Debug)]
struct StartExam {
    pattern: String,
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id: u64,
    content: String,
}

fn create_queue(
    connection: &mut Connection,
    exchange_name: &str,
    routing_key: &str,
    queue_name: &str,
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
}

fn answer_question(
    connection: &mut Connection,
    exchange_name: &str,
    routing_key: &str,
    answer_question: StartExam,
) {
    let channel = connection.open_channel(None).unwrap();
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
    println!("{:#?}", answer_question);
    let buffer_answer_question = to_vec(&answer_question).unwrap();
    exchange
        .publish(Publish::with_properties(
            &buffer_answer_question,
            routing_key,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
}

fn finish_exam(
    connection: &mut Connection,
    exchange_name: &str,
    routing_key: &str,
    queue_name: &str,
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
    let consumer = queue
        .consume(ConsumerOptions {
            exclusive: true,
            no_ack: true,
            ..ConsumerOptions::default()
        })
        .unwrap();

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}

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
                let start_exam_data: StartExam = serde_json::from_str(&body).unwrap();
                let exchange_name = "e_exam";
                let routing_key = format!("r_exam_{}", start_exam_data.data.id.to_string());
                let queue_name = format!("q_exam_{}", start_exam_data.data.id.to_string());
                if start_exam_data.pattern == "start_exam" {
                    create_queue(&mut connection, &exchange_name, &routing_key, &queue_name);
                } else if start_exam_data.pattern == "answer_question" {
                    answer_question(
                        &mut connection,
                        &exchange_name,
                        &routing_key,
                        start_exam_data,
                    );
                } else if start_exam_data.pattern == "finish_exam" {
                    finish_exam(&mut connection, &exchange_name, &routing_key, &queue_name);
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
