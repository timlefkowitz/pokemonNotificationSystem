use dotenvy::dotenv;
use reqwest::{Client, cookie::Jar};
use scraper::{Html, Selector};
use std::env;
use std::sync::Arc;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::{info, warn, error};
use mac_address::MacAddress;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let email = env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    let recipients = vec!["2102027013@vtext.com"];

    let jar = Arc::new(Jar::default());

    // Step 1: Define the known product URL for "Mario Kart 8 Deluxe"
    let product_url = "https://www.gamestop.com/video-games/nintendo-switch/products/mario-kart-8-deluxe---nintendo-switch/146184.html";
    info!("Checking stock for 'Mario Kart 8 Deluxe': {}", product_url);

    let mut in_stock_items = Vec::new();

    // Step 2: Check stock for the product
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 6] = rng.gen();
    let random_mac = MacAddress::new(random_bytes);
    info!("Checking stock for: {} (MAC: {})", product_url, random_mac);

    let client = Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("X-MAC-Address", random_mac.to_string().parse()?);
            headers
        })
        .build()?;

    let res = match client.get(product_url).send().await {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to fetch {}: {:?}", product_url, e);
            println!("❌ Out of stock: {} (Error fetching)", product_url);
            return Ok(());
        }
    };

    let body = match res.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("Failed to read response body for {}: {:?}", product_url, e);
            println!("❌ Out of stock: {} (Error reading)", product_url);
            return Ok(());
        }
    };

    // Debug: Log HTML length and a snippet
    info!("Fetched HTML length: {} bytes", body.len());
    let snippet = if body.len() > 200 { &body[0..200] } else { &body };
    info!("HTML snippet: {}", snippet);

    let document = Html::parse_document(&body);
    let selector = Selector::parse("button.js-add-to-cart.add-to-cart")?;
    let add_to_cart_button = document.select(&selector).next();

    match add_to_cart_button {
        Some(button) => {
            info!("Add to Cart button found: {:?}", button.value());
            println!("✅ In stock: {}", product_url);
            in_stock_items.push(product_url.to_string());
        }
        None => {
            warn!("No Add to Cart button found for {}", product_url);
            println!("❌ Out of stock: {}", product_url);
        }
    }

    // Step 3: Send email if in stock
    if !in_stock_items.is_empty() {
        let email_body = format!(
            "Hello,\n\nThese Mario Kart 8 Deluxe items are in stock:\n\n{}\n\nBest,\nGameStock Tracker",
            in_stock_items.join("\n")
        );
        send_email(&email, &password, &recipients, &email_body).await?;
    } else {
        info!("Mario Kart 8 Deluxe is not in stock.");
    }

    Ok(())
}

async fn send_email(
    sender_email: &str,
    password: &str,
    recipients: &[&str],
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new(sender_email.to_string(), password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    for recipient in recipients {
        let email_message = Message::builder()
            .from(sender_email.parse()?)
            .to(recipient.parse()?)
            .subject("Mario Kart 8 Deluxe In Stock!")
            .body(body.to_string())?;

        match mailer.send(&email_message) {
            Ok(_) => info!("✅ Email sent to {}", recipient),
            Err(e) => error!("❌ Failed to send email to {}: {:?}", recipient, e),
        }
    }

    Ok(())
}