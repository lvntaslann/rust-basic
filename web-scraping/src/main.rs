use mongodb::{bson::{doc, to_document, Document}, Client, Collection};
use scraper::{Html, Selector};
use reqwest;
use serde::Serialize;
use std::error::Error;
use csv::Writer; // Import the CSV Writer

#[derive(Debug, Clone, Serialize)]
struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

async fn save_product_to_mongo(collection: &Collection<Document>, product: Product) -> Result<(), Box<dyn Error>> {
    let document = to_document(&product)?;
    collection.insert_one(document, None).await?;
    Ok(())
}

fn save_product_to_csv(writer: &mut Writer<std::fs::File>, product: &Product) -> Result<(), Box<dyn Error>> {
    writer.serialize(product)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("ecommerce");
    let collection = database.collection::<Document>("products");

    // Create or open the CSV file for writing
    let csv_file_path = "products.csv";
    let file = std::fs::File::create(csv_file_path)?;
    let mut writer = Writer::from_writer(file);

    let mut page_number = 1;

    loop {
        let url = format!("https://www.scrapingcourse.com/ecommerce/page/{}/", page_number);
        let response = reqwest::get(&url).await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            eprintln!("Reached an inaccessible page at page {}. Stopping the scraping.", page_number);
            break;
        }

        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);

        let html_product_selector = Selector::parse("li.product").unwrap();
        let html_products = document.select(&html_product_selector);

        if html_products.clone().count() == 0 {
            eprintln!("No more products found on page {}. Ending the scraping.", page_number);
            break;
        }

        for html_product in html_products {
            let url = html_product
                .select(&Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);
            let image = html_product
                .select(&Selector::parse("img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(str::to_owned);
            let name = html_product
                .select(&Selector::parse("h2").unwrap())
                .next()
                .map(|h2| h2.text().collect::<String>());
            let price = html_product
                .select(&Selector::parse(".price").unwrap())
                .next()
                .map(|price| price.text().collect::<String>());

            let product = Product { url, image, name, price };

            // Save to MongoDB
            save_product_to_mongo(&collection, product.clone()).await?;

            // Save to CSV
            save_product_to_csv(&mut writer, &product)?;
        }

        println!("Page {} scraped successfully.", page_number);
        page_number += 1;
    }

    // Flush the CSV writer
    writer.flush()?;

    println!("Scraping completed.");
    Ok(())
}
