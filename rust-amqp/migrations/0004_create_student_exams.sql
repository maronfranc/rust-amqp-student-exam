CREATE TABLE IF NOT EXISTS student_exams (
    id          SERIAL PRIMARY KEY,
    id_student  INTEGER NOT NULL,
    id_exam     INTEGER NOT NULL,
    score       SMALLINT NOT NULL DEFAULT 0 CONSTRAINT non_negative_score CHECK(score >= 0),
    CONSTRAINT fk_students FOREIGN KEY(id_student) REFERENCES students(id),
    CONSTRAINT fk_exam FOREIGN KEY(id_exam) REFERENCES questions(id)
);
