use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub questions: Vec<QuestionDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuestionDto {
    pub id: i32,
    pub content: String,
    pub answers: Vec<AnswerDto>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerDto {
    pub id: i32,
    pub content: String,
}
