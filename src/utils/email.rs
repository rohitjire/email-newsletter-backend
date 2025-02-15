/// Module for sending emails using the Lettre crate.
///
/// Provides an asynchronous function to send emails via SMTP.
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

/// Sends an email using the provided recipient, subject, and body.
///
/// # Arguments
/// * `to` - The recipient email address.
/// * `subject` - The subject of the email.
/// * `body` - The email message body.
///
/// # Returns
/// * `Ok(())` if the email is sent successfully.
/// * `Err(Box<dyn std::error::Error>)` if there is an error.
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
