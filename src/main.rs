use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::Client;
#[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Replace these with your details
        let body = "https://www.gamestop.com/toys-games/trading-cards/products/pokemon-trading-card-game-prismatic-evolutions-binder-collection/417633.html";
        let sender_email = "blindfry@gmail.com";
        let password = "Applefor33!";
        let recipient = "2102027013@vtext.com";

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

        // Send email
        mailer.send(&email_message)?;
        println!("Notification sent!");
        Ok(())
    }
