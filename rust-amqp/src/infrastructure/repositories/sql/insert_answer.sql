INSERT INTO student_answers
    (id, id_student, id_question, id_answer, id_student_exam)
VALUES 
    (DEFAULT, $1, $2, $3, $4);