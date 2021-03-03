CREATE TABLE IF NOT EXISTS student_answers (
    id              SERIAL PRIMARY KEY,
    id_student      INTEGER REFERENCES students(id) NOT NULL,
    id_question     INTEGER REFERENCES questions(id) NOT NULL,
    id_answer       INTEGER REFERENCES answers(id) NOT NULL,
    id_student_exam INTEGER REFERENCES student_exams(id) NOT NULL,
    CONSTRAINT fk_students FOREIGN KEY(id_student) REFERENCES students(id),
    CONSTRAINT fk_questions FOREIGN KEY(id_question) REFERENCES questions(id),
    CONSTRAINT fk_answers FOREIGN KEY(id_answer) REFERENCES answers(id),
    CONSTRAINT fk_student_exam FOREIGN KEY(id_student_exam) REFERENCES student_exams(id)
);
