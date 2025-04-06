use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)] 
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub link: String,
}