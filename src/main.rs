use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use reqwest::Client;
use std::env; 
use dotenv::dotenv;

#[derive(Deserialize)]
struct CryptoQuery {
    symbol: String,
}

async fn get_crypto_info(query: web::Query<CryptoQuery>) -> impl Responder {
    dotenv().ok();

    let news_api_key = env::var("NEWS_API_KEY").expect("NEWS_API_KEY not set");

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .unwrap();

    let crypto_url = format!("https://api.coingecko.com/api/v3/coins/{}", query.symbol);

    let crypto_response = client.get(&crypto_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let crypto_data: serde_json::Value = serde_json::from_str(&crypto_response).unwrap();
    let crypto_name = crypto_data["name"].as_str().unwrap_or(query.symbol.as_str());

    let news_url = format!("https://newsapi.org/v2/everything?q={}&apiKey={}", crypto_name, news_api_key);
    let news_response = client.get(&news_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let combined_response = format!(r#"{{"crypto": {}, "news": {}}}"#, crypto_response, news_response);

    HttpResponse::Ok().body(combined_response)
}

use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/crypto", web::get().to(get_crypto_info))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
