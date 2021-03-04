use sqlx::{postgres::PgDone, PgPool};

use crate::application::dtos::answer_question_dto::AnswerQuestionData;
use crate::infrastructure::models::AnswerModel;

pub async fn insert(pool: &PgPool, answer: &AnswerQuestionData) -> Result<PgDone, sqlx::Error> {
    sqlx::query_file!(
        "src/infrastructure/repositories/sql/insert_answer.sql",
        answer.id_student,
        answer.id_question,
        answer.id_answer,
        answer.id_student_exam
    )
    .execute(pool)
    .await
}

pub async fn find_answers_by_question_id(
    pool: &PgPool,
    id_question: i32,
) -> Result<Vec<AnswerModel>, sqlx::Error> {
    sqlx::query_file_as!(
        AnswerModel,
        "src/infrastructure/repositories/sql/question_answers.sql",
        id_question,
    )
    .fetch_all(pool)
    .await
}
