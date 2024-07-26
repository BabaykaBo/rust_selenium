use std::error::Error;
use std::process::{Command, Child};
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

struct Product {
    name: String,
    price: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Start chromedriver
    let mut chromedriver_process: Child = Command::new("chromedriver")
        .spawn()
        .expect("Failed to start chromedriver");
    
    // Wait for a moment to ensure chromedriver starts properly
    sleep(Duration::from_secs(2)).await;

    // Define the browser options
    let caps = DesiredCapabilities::chrome();

    // Initialize a driver to control a Chrome instance with the specified options
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Visit the target page
    driver.goto("https://scrapingclub.com/exercise/list_infinite_scroll/").await?;

    // Where to store the scraping data
    let mut products: Vec<Product> = Vec::new();

    // Select all product cards on the page
    let product_html_elements = driver.find_all(By::Css(".post")).await?;

    // Iterate over them and apply the scraping logic
    for product_html_element in product_html_elements {
        let name = product_html_element
            .find(By::Css("h4"))
            .await?
            .text()
            .await?;
        let price = product_html_element
            .find(By::Css("h5"))
            .await?
            .text()
            .await?;

        // Create a new Product object and add it to the vector
        let product = Product { name, price };
        products.push(product);
    }

    // Log the scraped products
    for product in &products {
        println!("Price: {}\nName: {}\n", product.price, product.name)
    }

    // Close the browser and release its resources
    driver.quit().await?;

    // Stop chromedriver
    chromedriver_process.kill().ok();
    chromedriver_process.wait().ok();

    // create the CSV output file
    let path = std::path::Path::new("target/product.csv");
    let mut writer = csv::Writer::from_path(path)?;

    // add the header row to the CSV file
    writer.write_record(&["name", "price"])?;

    // write products
    for product in products {
        writer.write_record(&[product.name, product.price])?;
    }

    // free up the writer resources
    writer.flush().unwrap();


    Ok(())
}
