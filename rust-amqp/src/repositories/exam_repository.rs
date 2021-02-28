use crate::dtos::exam_dto::{AnswerDto, ExamDto, QuestionDto};
#[allow(unused_imports)]
use crate::models::{AnswerModel, ExamModel, QuestionModel};
use sqlx::PgPool;

pub async fn find_exam_template_by_id(pool: &PgPool, id_exam: i32) -> ExamDto {
    let exam = sqlx::query_as!(
        ExamModel,
        r#"SELECT id, name, description FROM exams WHERE id = $1 GROUP BY id, name, description;"#,
        id_exam
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let exam_questions = sqlx::query_as!(
        QuestionModel,
        r#"
        SELECT
            id,
            content
        FROM 
            questions WHERE id_exam = $1
        GROUP BY id, content;
        "#,
        id_exam,
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let mut questions_dto: Vec<QuestionDto> = Vec::with_capacity(exam_questions.len() as usize);
    for question in exam_questions {
        let answer = sqlx::query_as!(
            AnswerModel,
            r#"SELECT id, content FROM answers WHERE id_question = $1;"#,
            question.id,
        )
        .fetch_all(pool)
        .await
        .unwrap();
        let answers_dto: Vec<AnswerDto> = answer
            .into_iter()
            .map(|a| AnswerDto {
                id: a.id,
                content: a.content,
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
