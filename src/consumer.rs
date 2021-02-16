use amiquip::ConsumerOptions;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::QueueDeclareOptions;
use amiquip::{Connection, ConsumerMessage, ExchangeType};
const EXCHANGE_NAME: &str = "e_exams";
const ROUTING_KEY: &str = "routing_exam";
const QUEUE_NAME: &str = "q_process_exam";
const URL: &str = "amqp://guest:guest@localhost:5672";

fn main() {
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };
    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None).unwrap();
    let queue = channel
        .queue_declare(
            QUEUE_NAME,
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
            EXCHANGE_NAME,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: true,
                internal: false,
                arguments: FieldTable::default(),
            },
        )
        .unwrap();
    queue
        .bind(&exchange, ROUTING_KEY, FieldTable::default())
        .unwrap();
    let consumer = queue
        .consume(ConsumerOptions {
            exclusive: true,
            no_ack: true,
            ..ConsumerOptions::default()
        })
        .unwrap();
    println!("Waiting for messages. Press Ctrl-C to exit.");

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

    connection.close().unwrap()
}
