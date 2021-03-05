use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentExamDto {
    pub pattern: String,
    pub data: StudentExamData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudentExamData {
    pub id_student: i32,
    pub id_student_exam: i32,
}
