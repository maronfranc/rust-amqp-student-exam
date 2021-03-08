use sqlx::PgPool;

use crate::infrastructure::models::FinishedAtModel;
use crate::infrastructure::repositories::student_exams_repository;

pub async fn insert(pool: &PgPool, id_student: i32, id_exam: i32) -> i32 {
    student_exams_repository::insert(pool, id_student, id_exam).await
}

pub async fn finish_student_exam(pool: &PgPool, id_student_exam: i32) {
    student_exams_repository::finish_student_exam(pool, id_student_exam)
        .await
        .unwrap();
}

pub async fn find_finished_date_by_id(
    pool: &PgPool,
    id_student_exam: i32,
) -> Result<FinishedAtModel, sqlx::Error> {
    student_exams_repository::find_finished_date_by_id(pool, id_student_exam).await
}
