CREATE TABLE IF NOT EXISTS questions (
    id          SERIAL PRIMARY KEY,
    content    VARCHAR NOT NULL,
    id_exam   SERIAL REFERENCES exams(id) NOT NULL
);
