use dotenvy::dotenv;
use std::env;
use reqwest;
use scraper::{Html, Selector};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    let email = env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let product_links = vec![
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-binder-collection/417633.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/another-pokemon-product/123456.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/yet-another-pokemon-item/789101.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-blister---two-pack/418758.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-elite-trainer-box/417631.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-super-premium-collection/422620.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-mini-tin-styles-may-vary/418756.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-booster-bundle/418865.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-surprise-box-styles-may-vary/418757.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-poster-collection/417632.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-tech-sticker-collection/417634.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-scarlet-and-violet-prismatic-evolutions-accessory-pouch/419125.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-blooming-waters-premium-collection/418863.html",
    ];

    let mut in_stock_links = Vec::new();

    for link in &product_links {
        if check_stock(link).await? {
            in_stock_links.push(link.to_string());
        }
    }

    if !in_stock_links.is_empty() {
        send_email(&email, &password, &in_stock_links)?;
    } else {
        println!("No items in stock.");
    }

    Ok(())
}

// ✅ Check if the product is in stock
async fn check_stock(url: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse(".add-to-cart").unwrap();

    let in_stock = document.select(&selector).next().is_some();

    if in_stock {
        println!("✅ In stock: {}", url);
    } else {
        println!("❌ Out of stock: {}", url);
    }

    Ok(in_stock)
}

// ✅ Send an email notification
fn send_email(email: &str, password: &str, links: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let recipients = vec![
        "2102027013@vtext.com",
        "2108034785@txt.att.net",
        "2108754912@tmomail.net",
    ];

    let body = format!(
        "Hello,\n\nThe following items are now in stock:\n\n{}\n\nBest,\nPokemon Tracker",
        links.join("\n")
    );

    let creds = Credentials::new(email.to_string(), password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    for recipient in &recipients {
        let email_message = Message::builder()
            .from(email.parse()?)
            .to(recipient.parse()?)
            .subject("Pokemon Product In Stock!")
            .body(body.clone())?;

        match mailer.send(&email_message) {
            Ok(_) => println!("✅ Email sent to {}", recipient),
            Err(e) => println!("❌ Failed to send email to {}: {:?}", recipient, e),
        }
    }

    Ok(())
}
