use sqlx::PgPool;

use crate::application::dtos::answer_question_dto::AnswerQuestionData;
use crate::infrastructure::repositories::answer_repository;

pub async fn insert(pool: &PgPool, answer: &AnswerQuestionData) {
    answer_repository::insert(&pool, &answer).await.unwrap();
}
