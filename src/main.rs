use actix_web::{HttpServer, App, middleware, web, HttpResponse};
use sea_orm::{Database, DbConn, EntityTrait};
use entity::prelude::*;
use migration::{Migrator, MigratorTrait};

mod config;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    let cfg = config::Config::new();
    let conn =  Database::connect(cfg.db_url()).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = web::Data::new(conn);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .route("/quote", web::get().to(quote))
    }).bind(cfg.srv_url()).unwrap().run().await.unwrap();
}

async fn quote(conn: web::Data<DbConn>) -> HttpResponse {
    use rand::seq::SliceRandom;
    let db = conn.get_ref();
    let quotes = Quote::find().all(db).await.unwrap();
    let quote = quotes.choose(&mut rand::thread_rng()).unwrap();
    HttpResponse::Ok().json(quote)
}

