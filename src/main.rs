use reqwest::{self, header};
use serde::{Deserialize, Serialize};
use std::error::Error;

const NEWS_API_KEY: &str = "ea78dcaf7fbf4d978078c4cb50820f84";

#[derive(Debug, Serialize, Deserialize)]
struct NewsResponse {
    status: String,
    #[serde(rename = "totalResults")]
    total_results: u32,
    articles: Vec<NewsArticle>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NewsArticle {
    title: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Делаем запрос к API...");

    let client = reqwest::Client::new();

    let url = format!(
        "https://newsapi.org/v2/everything?q=bitcoin&language=en&sortBy=publishedAt&apiKey={}",
        NEWS_API_KEY
    );

    let response = client
        .get(&url)
        .header(header::USER_AGENT, "MyCryptoNewsApp/1.0")
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("Request failed with status: {} Body: {}", status, body).into());
    }

    let news: NewsResponse = serde_json::from_str(&body)?;

    println!("Нашли {} новостей про биткоин:\n", news.total_results);

    for article in news.articles.iter().take(10) {
        println!("• {} \n  {}", article.title, article.url);
    }

    Ok(())
}
