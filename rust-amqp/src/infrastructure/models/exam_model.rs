use sqlx::FromRow;

#[derive(FromRow, Debug)]
#[sqlx(rename = "exams")]
pub struct ExamModel {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(FromRow, Debug)]
#[sqlx(rename = "questions")]
pub struct QuestionModel {
    pub id: i32,
    pub content: String,
}

#[derive(FromRow, Debug)]
#[sqlx(rename = "answers")]
pub struct AnswerModel {
    pub id: i32,
    pub content: String,
}
