use sqlx::PgPool;

use crate::application::dtos::exam_dto::{AnswerDto, ExamDto, QuestionDto};
use crate::infrastructure::repositories::{
    answer_repository, exam_repository, questions_repository,
};

pub async fn find_exam_template_by_id(pool: &PgPool, id_exam: i32) -> ExamDto {
    let exam = exam_repository::find_by_id(&pool, id_exam).await.unwrap();
    let exam_questions = questions_repository::find_questions_by_exam_id(&pool, id_exam)
        .await
        .unwrap();
    let mut questions_dto: Vec<QuestionDto> = Vec::with_capacity(exam_questions.len() as usize);
    for question in exam_questions {
        let answers = answer_repository::find_answers_by_question_id(&pool, question.id)
            .await
            .unwrap();
        let answers_dto: Vec<AnswerDto> = answers
            .into_iter()
            .map(|answer| AnswerDto {
                id: answer.id,
                content: answer.content,
            })
            .collect();
        questions_dto.push(QuestionDto {
            id: question.id,
            content: question.content,
            answers: answers_dto,
        });
    }
    ExamDto {
        id: exam.id,
        name: exam.name,
        description: exam.description,
        questions: questions_dto,
    }
}

pub async fn insert(pool: &PgPool, id_exam: i32, id_student: i32) {
    exam_repository::insert(pool, id_exam, id_student)
        .await
        .unwrap();
}
