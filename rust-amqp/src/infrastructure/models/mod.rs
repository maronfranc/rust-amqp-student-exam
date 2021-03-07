use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct ExamModel {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Debug)]
pub struct QuestionModel {
    pub id: i32,
    pub content: String,
}

#[derive(FromRow, Debug)]
pub struct AnswerModel {
    pub id: i32,
    pub content: String,
}

#[derive(FromRow, Debug)]
pub struct AnswerCorretionModel {
    pub is_correct: bool,
}

#[derive(FromRow, Debug)]
pub struct IdModel {
    pub id: i32,
}
