use sqlx::PgPool;

use crate::application::dtos::answer_question_dto::AnswerQuestionData;
use crate::infrastructure::models::AnswerCorretionModel;
use crate::infrastructure::repositories::{
    answer_repository, student_answers_repository, student_exams_repository,
};

pub async fn insert(pool: &PgPool, answer: &AnswerQuestionData) {
    student_answers_repository::insert(&pool, &answer)
        .await
        .unwrap();

    let answer_corretion: AnswerCorretionModel =
        answer_repository::find_correction_by_id(&pool, answer.id_answer)
            .await
            .unwrap();

    if answer_corretion.is_correct {
        student_exams_repository::increment_score(&pool, answer.id_student_exam)
            .await
            .unwrap();
    }
}
