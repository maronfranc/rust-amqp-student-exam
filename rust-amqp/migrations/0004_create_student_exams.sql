CREATE TABLE IF NOT EXISTS student_exams (
    id          SERIAL PRIMARY KEY,
    id_student  SERIAL REFERENCES students(id) NOT NULL,
    id_exam     SERIAL REFERENCES exams(id) NOT NULL,
    score      SMALLINT CONSTRAINT non_negative_score CHECK(score >= 0)
);
