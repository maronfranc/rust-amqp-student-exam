use amiquip::ExchangeDeclareOptions;
use amiquip::FieldTable;
use amiquip::{Connection, ExchangeType};
const EXCHANGE_NAME: &str = "e_exams";
const URL: &str = "amqp://guest:guest@localhost:5672";

fn main() {
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };
    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None).unwrap();
    channel
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
    connection.close().unwrap()
}
