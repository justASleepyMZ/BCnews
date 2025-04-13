# Crypto and News API Server

This project is a server built with **Actix Web** that provides an API for retrieving cryptocurrency information and related news. The server fetches data about cryptocurrencies and news using the **CoinMarketCap API** and **NewsAPI**.

## How to Run

### 1. Install Dependencies

Add the required dependencies to your `Cargo.toml` file:

[dependencies]
actix-web = "4.0"
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
actix-files = "0.6"
2. Obtain API Keys
To use this project, you need to register on the following services and get API keys:

CoinMarketCap API: Register at CoinMarketCap and get your API key.

NewsAPI: Register at NewsAPI and get your API key.

After obtaining the keys, replace them in the code in the following lines:

let cmc_api_key = "YOUR_CMC_API_KEY";
let news_api_key = "YOUR_NEWS_API_KEY";
3. Build and Run the Server
Clone the repository or download the source code.

Run the following command to build and run the server:

cargo run
The server will be available on port 8080.

4. API Usage
Make a GET request to the URL:

http://127.0.0.1:8080/crypto?symbol=BTC
Where BTC is the cryptocurrency symbol (e.g., use BTC for Bitcoin). The response will contain cryptocurrency information and related news.

5. Static Files
The server also serves static files from the ./static folder. An index.html file should be placed in this folder, and it will be served by default.

Project Structure
src/main.rs: The main server file.

static/: Folder to store static files such as index.html.

License
This project is licensed under the MIT License. See the LICENSE file for details.

This `README.md` provides an overview of the project, explains how to run it, and lists what is needed to use the API.
![image](https://github.com/user-attachments/assets/3241395b-6fa8-4f43-9866-46500af511e8)
![image](https://github.com/user-attachments/assets/cfae5d16-5d5e-4f8b-8893-7c4cf2fc0319)
![image](https://github.com/user-attachments/assets/1bc85eaf-cdaf-4d97-a5f6-03c90787ff47)


