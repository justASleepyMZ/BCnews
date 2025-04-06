use crate::models::NewsArticle;
use reqwest::Client;
use serde_json::Value;

pub async fn fetch_news(crypto: &str) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut articles = Vec::new();

    // Example API calls (replace with actual API endpoints and keys)
    let sources = vec![
        format!("https://cryptonews-api.com/api/v1?tickers={}&items=5&token=YOUR_API_KEY", crypto),
        format!("https://newsapi.org/v2/everything?q={}&apiKey=YOUR_API_KEY", crypto),
    ];

    for source in sources {
        let response: Value = client.get(&source).send().await?.json().await?;
        // Parse the response based on the API structure
        // This is a placeholder; adjust according to the actual API response
        for article in response["articles"].as_array().unwrap() {
            articles.push(NewsArticle {
                title: article["title"].as_str().unwrap().to_string(),
                source: article["source"]["name"].as_str().unwrap().to_string(),
                date: article["publishedAt"].as_str().unwrap().to_string(),
                summary: article["description"].as_str().unwrap().to_string(),
                link: article["url"].as_str().unwrap().to_string(),
            });
        }
    }

    Ok(articles)
}