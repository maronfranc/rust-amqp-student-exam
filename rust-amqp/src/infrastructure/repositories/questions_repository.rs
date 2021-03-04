use sqlx::PgPool;

use crate::infrastructure::models::QuestionModel;

pub async fn find_questions_by_exam_id(
    pool: &PgPool,
    id_exam: i32,
) -> Result<Vec<QuestionModel>, sqlx::Error> {
    sqlx::query_file_as!(
        QuestionModel,
        "src/infrastructure/repositories/sql/exam_questions.sql",
        id_exam,
    )
    .fetch_all(pool)
    .await
}
