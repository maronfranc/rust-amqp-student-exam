CREATE TABLE IF NOT EXISTS student_answers (
    id              SERIAL PRIMARY KEY,
    id_student      INTEGER NOT NULL REFERENCES students(id),
    id_question     INTEGER NOT NULL REFERENCES questions(id),
    id_answer       INTEGER NOT NULL REFERENCES answers(id),
    id_student_exam INTEGER NOT NULL REFERENCES student_exams(id)
);
