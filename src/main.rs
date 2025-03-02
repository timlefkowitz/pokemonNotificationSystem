use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::Client;
#[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // multiable links
    let product_links = vec![
        "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-binder-collection/417633.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/another-pokemon-product/123456.html",
        "https://www.gamestop.com/toys-games/trading-cards/products/yet-another-pokemon-item/789101.html",
    ];

    let sender_email = "blindfry@gmail.com";
    let password = "Applefor33!";
    let recipients = vec![
        "2102027013@vtext.com",
        "anotherperson@example.com",
        "thirdperson@example.com",
    ];


    // Construct message with all links
    let body = product_links.join("\n");

    // Iterate over recipients and send the message
    for recipient in &recipients {
        // Your email sending function here
        println!("Sending to: {}\nMessage:\n{}", recipient, body);
    }
    let sender_email = "blindfry@gmail.com";
        let password = "Applefor33!";
        let recipients = vec![
                                        "2102027013@vtext.com",
                                         "anotherperson@example.com",
                                        "thirdperson@example.com",
    ];

        // Create email message
        let email_message = Message::builder()
            .from(sender_email.parse()?)
            .to(recipient.parse()?)
            .subject("Product Availability")
            .body(body.to_string())?;

        // SMTP server (use Gmail's server in this example)
        let creds = Credentials::new(sender_email.to_string(), password.to_string());
        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .credentials(creds)


            .build();

        // Send email'
        mailer.send(&email_message)?;
        println!("Notification sent!");
        Ok(())
    }
