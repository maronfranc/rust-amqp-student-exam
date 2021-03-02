CREATE TABLE IF NOT EXISTS exams (
    id          SERIAL PRIMARY KEY,
    name        VARCHAR NOT NULL,
    description VARCHAR NOT NULL
);
