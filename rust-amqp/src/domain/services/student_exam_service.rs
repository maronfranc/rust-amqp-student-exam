use sqlx::PgPool;

use crate::infrastructure::repositories::student_exams_repository;

pub async fn insert(pool: &PgPool, id_student: i32, id_exam: i32) -> i32 {
    student_exams_repository::insert(pool, id_student, id_exam).await
}
