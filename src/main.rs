use actix_web::{web, App, HttpServer, Responder};
use crate::api::fetch_news;
use log::info; 

mod api;
mod models;

async fn get_news(crypto: web::Path<String>) -> impl Responder {
    match fetch_news(&crypto).await {
        Ok(articles) => web::Json(articles),
        Err(_) => web::Json(vec![]),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/news/{crypto}", web::get().to(get_news))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}