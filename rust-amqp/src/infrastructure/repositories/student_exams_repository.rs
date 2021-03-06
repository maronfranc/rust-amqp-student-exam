use sqlx::{postgres::PgDone, PgPool};

use crate::infrastructure::models::IdModel;

pub async fn increment(pool: &PgPool, id_student_exam: i32) -> Result<PgDone, sqlx::Error> {
    sqlx::query_as!(
        AnswerCorretionModel,
        "UPDATE student_exams SET score = score + 1 WHERE id = $1;",
        id_student_exam,
    )
    .execute(pool)
    .await
}

pub async fn insert(pool: &PgPool, id_student: i32, id_exam: i32) -> i32 {
    sqlx::query_file_as!(
        IdModel,
        "src/infrastructure/repositories/sql/insert_student_exam.sql",
        id_student,
        id_exam,
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .id
}
