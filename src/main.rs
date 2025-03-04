use dotenvy::dotenv;
use reqwest::{Client, cookie::Jar};
use scraper::{Html, Selector};
use std::env;
use tokio;
use std::sync::Arc;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load environment variables

    let email = env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");

    let product_links = vec![
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-binder-collection/417633.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-booster-bundle/418865.html",
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
        "https://www.gamestop.com/video-games/playstation-5/products/astro-bot---playstation-5/415222.html",
    ];

    let recipients = vec![
        "2102027013@vtext.com",
        "2108034785@txt.att.net",
        "2108754912@tmomail.net",
    ];

    // Create a cookie jar to store cookies
    let jar = Arc::new(Jar::default());

    // Create the reqwest client with cookie support
    let client = Client::builder()
        .cookie_provider(Arc::clone(&jar))
        .build()?;

    let mut in_stock_items = Vec::new();

    for link in &product_links {
        let res = client.get(*link).send().await?.text().await?;
        let document = Html::parse_document(&res);

        let selector = Selector::parse("#add-to-cart-buttons > div.atc-btns-wrapper > div > button.js-add-to-cart.add-to-cart.btn.btn-primary.add-to-cart-redesign.all").unwrap();
        let add_to_cart_button = document.select(&selector).next();

        println!("{}", res);

        if add_to_cart_button.is_some() {
            println!("✅ In stock: {}", link);
            in_stock_items.push(link.to_string());
        } else {
            println!("❌ Out of stock: {}", link);
        }
    }

    // Send email if any items are in stock
    if !in_stock_items.is_empty() {
        let email_body = format!(
            "Hello,\n\nThese items are in stock:\n\n{}\n\nBest,\nPokemon Tracker",
            in_stock_items.join("\n")
        );

        send_email(&email, &password, &recipients, &email_body).await?;
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
            .subject("Pokemon Product In Stock!")
            .body(body.to_string())?;

        match mailer.send(&email_message) {
            Ok(_) => println!("✅ Email sent to {}", recipient),
            Err(e) => println!("❌ Failed to send email to {}: {:?}", recipient, e),
        }
    }

    Ok(())
}
