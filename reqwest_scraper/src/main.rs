

fn parse_document(response: String) -> Vec<String> {
    let document = scraper::Html::parse_document(&response);
    let product_selector = scraper::Selector::parse("li.product").unwrap();
    let products = document.select(&product_selector).map(|product| {
        let name_selector = scraper::Selector::parse("h2").unwrap();
        let name = product.select(&name_selector).next().unwrap().text().collect::<Vec<_>>().join(" ");
        name
    }).collect::<Vec<_>>();
    products
}

async fn get_response(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(response)
}

#[tokio::main]
async fn main() {
    let url = "https://scrapeme.live/shop/";
    let response = get_response(url).await.unwrap();
    let products = parse_document(response);
    println!("{:?}", products);
}
