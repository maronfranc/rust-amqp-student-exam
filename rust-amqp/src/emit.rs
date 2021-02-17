use amiquip::AmqpProperties;
use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::{Connection, ExchangeType, Publish};
use std::env;
const EXCHANGE_NAME: &str = "e_exams";
const ROUTING_KEY: &str = "routing_exam";
const URL: &str = "amqp://guest:guest@localhost:5672";
const PERSISTENT_MESSAGE: u8 = 2;

fn main() {
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };
    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None).unwrap();
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
    let mut message = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if message.is_empty() {
        message = "Hello world.".to_string();
    }
    exchange
        .publish(Publish::with_properties(
            message.as_bytes(),
            ROUTING_KEY,
            AmqpProperties::default().with_delivery_mode(PERSISTENT_MESSAGE),
        ))
        .unwrap();
    connection.close().unwrap()
}
