pub fn get_student_exam_queue_names(
    id_student: i32,
    id_student_exam: i32,
) -> (String, String, String) {
    let exchange_name = String::from("e_exam");
    let queue_name = format!(
        "q_exam_{}_student_{}",
        id_student_exam.to_string(),
        id_student.to_string()
    );
    let routing_key = format!(
        "r_exam_{}_student_{}",
        id_student_exam.to_string(),
        id_student.to_string()
    );

    return (exchange_name, queue_name, routing_key);
}
