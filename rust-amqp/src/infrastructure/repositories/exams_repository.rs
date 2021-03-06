use sqlx::PgPool;

use crate::infrastructure::models::ExamModel;

pub async fn find_by_id(pool: &PgPool, id_exam: i32) -> Result<ExamModel, sqlx::Error> {
    sqlx::query_file_as!(
        ExamModel,
        "src/infrastructure/repositories/sql/exam_by_id.sql",
        id_exam
    )
    .fetch_one(pool)
    .await
}
