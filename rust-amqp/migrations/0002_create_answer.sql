CREATE TABLE IF NOT EXISTS answers (
    id          SERIAL PRIMARY KEY,
    content     VARCHAR NOT NULL,
    is_correct  BOOLEAN NOT NULL DEFAULT FALSE,
    id_question INTEGER NOT NULL REFERENCES questions(id)
);
