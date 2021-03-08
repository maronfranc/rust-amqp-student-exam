use sqlx::postgres::PgDone;
use sqlx::PgPool;

use crate::infrastructure::models::{FinishedAtModel, IdModel};

pub async fn increment_score(pool: &PgPool, id_student_exam: i32) -> Result<PgDone, sqlx::Error> {
    sqlx::query_as!(
        AnswerCorretionModel,
        "UPDATE student_exams SET score = score + 1 WHERE id = $1;",
        id_student_exam,
    )
    .execute(pool)
    .await
}

pub async fn finish_student_exam(
    pool: &PgPool,
    id_student_exam: i32,
) -> Result<PgDone, sqlx::Error> {
    sqlx::query_as!(
        AnswerCorretionModel,
        "UPDATE student_exams SET finished_at = CURRENT_TIMESTAMP WHERE id = $1;",
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

pub async fn find_finished_date_by_id(
    pool: &PgPool,
    id_student_exam: i32,
) -> Result<FinishedAtModel, sqlx::Error> {
    sqlx::query_as!(
        FinishedAtModel,
        "SELECT  finished_at from student_exams WHERE id = $1;",
        id_student_exam,
    )
    .fetch_one(pool)
    .await
}
