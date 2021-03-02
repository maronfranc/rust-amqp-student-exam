use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerQuestionDto {
    pub pattern: String,
    pub data: AnswerQuestionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerQuestionData {
    pub id_student: i32,
    pub id_exam: i32,
    pub id_question: i32,
    pub id_answer: i32,
}
