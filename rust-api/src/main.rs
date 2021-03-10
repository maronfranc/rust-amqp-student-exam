use actix_web::{App, HttpServer};

mod exam;
use exam::exam_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(exam_controller::start_exam)
            .service(exam_controller::answer_question)
            .service(exam_controller::finish_exam)
    });
    server.bind("127.0.0.1:4001").unwrap().run().await
}
