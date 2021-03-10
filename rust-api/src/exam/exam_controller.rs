use crate::exam::dto::answer_question_dto::AnswerQuestionData;
use crate::exam::dto::finish_exam_dto::FinishExamData;
use crate::exam::dto::start_exam_dto::StartExamData;
use actix_web::{post, web, Responder};
use serde::Serialize;
use web::{HttpResponse, Path};

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

use crate::exam::exam_queue_service;

const MOCK_ID_STUDENT: i32 = 999;

#[post("/v1/exam/{id_exam}/start")]
pub async fn start_exam(Path(id_exam): Path<i32>) -> impl Responder {
    let exam_dto = exam_queue_service::send_start_exam(StartExamData {
        id_exam: id_exam,
        id_student: MOCK_ID_STUDENT,
    });

    HttpResponse::Ok().json(exam_dto)
}

#[post("/v1/exam/{id_student_exam}/question/{id_question}/answer/{id_answer}")]
pub async fn answer_question(
    Path((id_student_exam, id_question, id_answer)): Path<(i32, i32, i32)>,
) -> impl Responder {
    let res = exam_queue_service::send_answer_question(AnswerQuestionData {
        id_student: MOCK_ID_STUDENT,
        id_student_exam: id_student_exam,
        id_question: id_question,
        id_answer: id_answer,
    });
    HttpResponse::Ok().json(res)
}

#[post("/v1/exam/{id_student_exam}/finish")]
pub async fn finish_exam(Path(id_student_exam): Path<i32>) -> impl Responder {
    let res = exam_queue_service::send_finish_exam(FinishExamData {
        id_student: MOCK_ID_STUDENT,
        id_student_exam: id_student_exam,
    });
    HttpResponse::Ok().json(res)
}
