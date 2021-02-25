use amiquip::Connection;

mod dtos;
mod pattern_queue;
mod patterns;

const URL: &str = "amqp://guest:guest@localhost:5672";

fn main() {
    let mut connection = match Connection::insecure_open(URL) {
        Ok(conn) => conn,
        Err(error) => panic!("Connection error: {:?}", error),
    };

    pattern_queue::pattern_queue(&mut connection);

    connection.close().unwrap()
}
