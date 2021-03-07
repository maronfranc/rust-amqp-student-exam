CREATE TABLE IF NOT EXISTS student_exams (
    id          SERIAL PRIMARY KEY,
    id_student  INTEGER NOT NULL REFERENCES students(id),
    id_exam     INTEGER NOT NULL REFERENCES questions(id),
    score       SMALLINT NOT NULL DEFAULT 0 CONSTRAINT non_negative_score CHECK(score >= 0)
);
