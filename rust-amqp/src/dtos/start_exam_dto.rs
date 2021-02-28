use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StartExamDto {
    pub pattern: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub id_student: i32,
    pub id_exam: i32,
}
