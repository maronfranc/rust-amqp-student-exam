use sqlx::postgres::PgPoolOptions;
use std::env::var;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_url: String = var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Unable to connect to database");

    sqlx::query(
        r#"
    INSERT INTO exams
        (id, name, description)
    VALUES 
        (1, 'Exam #1', 'Exam description.');
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
    INSERT INTO questions
        (id, content, id_exam)
    VALUES 
        (1, 'Question #1', 1),
        (2, 'Question #2', 1),
        (3, 'Question #3', 1),
        (4, 'Question #4', 1),
        (5, 'Question #5', 1);
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
    INSERT INTO answers
        (id, content, is_correct, id_question)
    VALUES 
        (DEFAULT, 'Question #1 Answer #1', TRUE, 1),
        (DEFAULT, 'Question #1 Answer #2', FALSE, 1),
        (DEFAULT, 'Question #1 Answer #3', FALSE, 1),
        (DEFAULT, 'Question #1 Answer #4', FALSE, 1),
        (DEFAULT, 'Question #1 Answer #5', FALSE, 1),
    
        (DEFAULT, 'Question #2 Answer #1', FALSE, 2),
        (DEFAULT, 'Question #2 Answer #2', TRUE, 2),
        (DEFAULT, 'Question #2 Answer #3', FALSE, 2),
        (DEFAULT, 'Question #2 Answer #4', FALSE, 2),
        (DEFAULT, 'Question #2 Answer #5', FALSE, 2),
    
        (DEFAULT, 'Question #3 Answer #1', FALSE, 3),
        (DEFAULT, 'Question #3 Answer #2', FALSE, 3),
        (DEFAULT, 'Question #3 Answer #3', TRUE, 3),
        (DEFAULT, 'Question #3 Answer #4', FALSE, 3),
        (DEFAULT, 'Question #3 Answer #5', FALSE, 3),
    
        (DEFAULT, 'Question #4 Answer #1', FALSE, 4),
        (DEFAULT, 'Question #4 Answer #2', FALSE, 4),
        (DEFAULT, 'Question #4 Answer #3', TRUE, 4),
        (DEFAULT, 'Question #4 Answer #4', FALSE, 4),
        (DEFAULT, 'Question #4 Answer #5', FALSE, 4),
    
        (DEFAULT, 'Question #5 Answer #1', FALSE, 5),
        (DEFAULT, 'Question #5 Answer #2', FALSE, 5),
        (DEFAULT, 'Question #5 Answer #3', TRUE, 5),
        (DEFAULT, 'Question #5 Answer #4', FALSE, 5),
        (DEFAULT, 'Question #5 Answer #5', FALSE, 5);
    "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(r#"INSERT INTO students (id, name) VALUES (999, 'Dev Tester');"#)
        .execute(&pool)
        .await
        .unwrap();
}
