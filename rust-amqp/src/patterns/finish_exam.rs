use amiquip::ConsumerOptions;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::QueueDeclareOptions;
use amiquip::{Connection, ConsumerMessage, ExchangeType};

const URL: &str = "amqp://guest:guest@localhost:5672";

pub fn finish_exam(exchange_name: &str, routing_key: &str, queue_name: &str) {
    let mut connection = match Connection::insecure_open(URL) {
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
    for (i, message) in consumer.receiver().iter().enumerate() {
        match (i, message) {
            (i, _message) if i + 1 == total_queue_messages => {
                break;
            }
            (i, ConsumerMessage::Delivery(delivery)) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close().unwrap();
}
