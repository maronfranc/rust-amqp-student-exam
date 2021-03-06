use sqlx::PgPool;

use crate::infrastructure::models::{AnswerCorretionModel, AnswerModel};

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

pub async fn find_correction_by_id(
    pool: &PgPool,
    id_answer: i32,
) -> Result<AnswerCorretionModel, sqlx::Error> {
    sqlx::query_as!(
        AnswerCorretionModel,
        "SELECT is_correct from answers WHERE id = $1;",
        id_answer,
    )
    .fetch_one(pool)
    .await
}
