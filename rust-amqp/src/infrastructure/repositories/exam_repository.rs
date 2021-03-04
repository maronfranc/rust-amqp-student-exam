use sqlx::{postgres::PgDone, PgPool};

use crate::infrastructure::models::ExamModel;

pub async fn insert(pool: &PgPool, id_exam: i32, id_student: i32) -> Result<PgDone, sqlx::Error> {
    sqlx::query_file!(
        "src/infrastructure/repositories/sql/insert_student_exam.sql",
        id_student,
        id_exam
    )
    .execute(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id_exam: i32) -> Result<ExamModel, sqlx::Error> {
    sqlx::query_file_as!(
        ExamModel,
        "src/infrastructure/repositories/sql/exam_by_id.sql",
        id_exam
    )
    .fetch_one(pool)
    .await
}
