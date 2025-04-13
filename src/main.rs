use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use reqwest::Client;

#[derive(Deserialize)]
struct CryptoQuery {
    symbol: String,
}

async fn get_crypto_info(query: web::Query<CryptoQuery>) -> impl Responder {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()
        .unwrap();
    let cmc_api_key = "48dc7c63-c686-4961-8d22-17522a011f58";
    let news_api_key = "ea78dcaf7fbf4d978078c4cb50820f84";
    let crypto_url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/info?symbol={}", query.symbol);

    let crypto_response = client.get(&crypto_url)
        .header("X-CMC_PRO_API_KEY", cmc_api_key)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let crypto_data: serde_json::Value = serde_json::from_str(&crypto_response).unwrap();
    let crypto_name = crypto_data["data"][query.symbol.to_uppercase()]["name"].as_str().unwrap_or(query.symbol.as_str());

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