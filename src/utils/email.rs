use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("no-reply@example.com".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    let mailer = SmtpTransport::relay("smtp.example.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}
