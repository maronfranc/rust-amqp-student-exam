CREATE TABLE IF NOT EXISTS student_answers (
    id          SERIAL PRIMARY KEY,
    id_student  SERIAL REFERENCES students(id) NOT NULL,
    id_question SERIAL REFERENCES questions(id) NOT NULL,
    id_answer   SERIAL REFERENCES answers(id) NOT NULL
);
