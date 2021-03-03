CREATE TABLE IF NOT EXISTS questions (
    id          SERIAL PRIMARY KEY,
    content     VARCHAR NOT NULL,
    id_exam     INTEGER NOT NULL,
    CONSTRAINT fk_exams FOREIGN KEY(id_exam) REFERENCES exams(id)
);
