use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerQuestionDto {
    pub pattern: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub id_student: u64,
    pub id_exam: u64,
    pub id_question: u64,
    pub id_answer: u64,
}
