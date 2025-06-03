use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::Job;
use crate::tls::load_tls_config;

pub async fn run() -> std::io::Result<()> {
    let db_pool = SqlitePool::connect("sqlite:data.db").await.expect("Failed to connect to DB");
    let tls_config = load_tls_config().expect("Failed to load TLS config");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/add_job/{command}", web::post().to(
                |db_pool: web::Data<SqlitePool>, path: web::Path<String>| {
                    let command = path.into_inner();
                    add_job(db_pool, command)
                }
            ))
            .route("/jobs", web::get().to(get_jobs))
    })
    .bind_rustls("0.0.0.0:8443", tls_config)?
    .run()
    .await
}

async fn add_job(
    db_pool: web::Data<SqlitePool>,
    command: String,
) -> impl Responder {
    let result = sqlx::query("INSERT INTO jobs (id, command, status, retries) VALUES (?, ?, ?, ?)")
        .bind(Uuid::new_v4().to_string())
        .bind(&command)
        .bind("pending")
        .bind(0)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Job added"),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

async fn get_jobs(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let jobs = sqlx::query_as::<_, Job>("SELECT * FROM jobs")
        .fetch_all(db_pool.get_ref())
        .await;

    match jobs {
        Ok(jobs) => HttpResponse::Ok().json(jobs),
        Err(e) => HttpResponse::InternalServerError().body(format!("DB error: {}", e)),
    }
}

