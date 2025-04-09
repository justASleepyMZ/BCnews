# MyCryptoNewsApp

This is a Rust application that uses the `reqwest` library to make HTTP requests to the News API and displays the latest Bitcoin-related news.

## Description

The program makes a request to the [NewsAPI](https://newsapi.org/) and prints the first 10 articles related to Bitcoin. Each article contains the title and a link to the full article.

### Structures:

- **`NewsResponse`**: Represents the API response, containing the status, total number of results, and a list of articles.
- **`NewsArticle`**: Represents an individual article with fields `title` (the article title) and `url` (link to the full article).

## Installation and Running

### 1. Installing Dependencies

Before running the project, you need to install the necessary dependencies. Add the following lines to your `Cargo.toml` file:

[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }```

## 2. Getting the API Key
To access the NewsAPI, you need to get an API key:

Go to NewsAPI.

Sign up and obtain your API key.

Replace the line in the code:

const NEWS_API_KEY: &str = "your_api_key_here";

with your own API key.

## 3. Running the Project
Clone the repository or create a new project with this code.

In the terminal, run the following command to start the project:

cargo run
![image](https://github.com/user-attachments/assets/1c8c0d07-a59b-4548-9ac6-d101f4425067)

## Technologies
Rust — Primary programming language.

reqwest — HTTP client for making requests.

serde and serde_json — For working with JSON.

tokio — Asynchronous runtime for running async functions.

## License
This project is licensed under the MIT License.
Now you have a `README.md` file fully in English that explains how to install dependencies, set up the API key, and run the project.
