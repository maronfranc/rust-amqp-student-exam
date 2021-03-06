use sqlx::{postgres::PgDone, PgPool};

use crate::application::dtos::answer_question_dto::AnswerQuestionData;

pub async fn insert(pool: &PgPool, answer: &AnswerQuestionData) -> Result<PgDone, sqlx::Error> {
    sqlx::query_file!(
        "src/infrastructure/repositories/sql/insert_student_answer.sql",
        answer.id_student,
        answer.id_question,
        answer.id_answer,
        answer.id_student_exam
    )
    .execute(pool)
    .await
}
