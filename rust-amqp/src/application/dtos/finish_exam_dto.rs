use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishExamDto {
    pub pattern: String,
    pub data: FinishExamData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishExamData {
    pub id_student: i32,
    pub id_exam: i32,
}
