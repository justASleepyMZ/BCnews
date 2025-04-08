document.getElementById('search-btn').addEventListener('click', async function() {
    const symbol = document.getElementById('crypto-symbol').value.trim().toLowerCase();

    if (symbol === '') {
        alert('Please enter a valid cryptocurrency symbol.');
        return;
    }

    try {
        // Отправка запроса на сервер
        const response = await fetch(`/crypto?symbol=${symbol}`);
        if (!response.ok) {
            throw new Error('Failed to fetch crypto data');
        }

        const data = await response.json();

        // Обработка данных криптовалюты
        const cryptoDetails = document.getElementById('crypto-details');
        const newsList = document.getElementById('news-list');

        const cryptoInfo = JSON.parse(data.crypto);
        const cryptoName = cryptoInfo.name;
        const cryptoPrice = cryptoInfo.market_data.current_price.usd;
        cryptoDetails.textContent = `Name: ${cryptoName}\nPrice (USD): $${cryptoPrice}`;

        // Обработка новостей
        const newsInfo = JSON.parse(data.news);
        newsList.innerHTML = '';

        newsInfo.articles.forEach(article => {
            const li = document.createElement('li');
            const a = document.createElement('a');
            a.href = article.url;
            a.target = '_blank';
            a.textContent = article.title;
            li.appendChild(a);
            newsList.appendChild(li);
        });

    } catch (error) {
        console.error(error);
        alert('Something went wrong. Please try again later.');
    }
});
