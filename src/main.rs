use std::error::Error;
use thirtyfour::prelude::*;


struct Product {
    name: String,
    price:String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // define the browser options
    // let mut caps = DesiredCapabilities::chrome();
    let caps = DesiredCapabilities::chrome();
    
    // to run Chrome in headless mode
    // comment out in development
    // initialize a driver to control a Chrome instance
    // with the specified options
    // caps.add_arg("--headless=new")?; 
    
    
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // visit the target page
    driver.goto("https://scrapingclub.com/exercise/list_infinite_scroll/").await?;
    
    // retrieve the source HTML of the target page
    // and print it
    //let html = driver.source().await?;
    //println!("{html}");

    // where to store the scraping data
    let mut products: Vec<Product> = Vec::new();

    // select all product cards on the page
    let product_html_elements = driver.find_all(By::Css(".post")).await?;

    // iterate over them and apply the scraping logic
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

        // create a new Product object and
        // add it to the vector
        let product = Product { name, price };
        products.push(product);
    }

    // log the scraped products
    for product in products {
        println!("Price: {}\nName: {}\n", product.price, product.name)
    }
    
    // close the browser and release its resources
    driver.quit().await?;

    Ok(())
}
