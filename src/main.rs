use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqliteQueryResult};
use std::sync::Arc;

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Job {
    id: i64,
    command: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct AddJobRequest {
    command: String,
}

async fn add_job(
    db_pool: web::Data<SqlitePool>,
    json: web::Json<AddJobRequest>,
) -> impl Responder {
    let query = "INSERT INTO jobs (command, status) VALUES (?, 'pending')";
    let result: Result<SqliteQueryResult, sqlx::Error> = sqlx::query(query)
        .bind(&json.command)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Job added"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to add job: {}", e)),
    }
}

async fn list_jobs(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result: Result<Vec<Job>, sqlx::Error> = sqlx::query_as("SELECT id, command, status FROM jobs")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to fetch jobs: {}", e)),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Connect to SQLite DB file
    let db_pool = SqlitePool::connect("sqlite:data/data.db")
        .await
        .expect("Failed to connect to DB");

    // Create table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            command TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending'
        )
        "#,
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create table");

    let db_pool_data = web::Data::new(db_pool);

    println!("Server running on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone())
            .route("/add_job", web::post().to(add_job))
            .route("/jobs", web::get().to(list_jobs))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

