//! Email Service Module
//!
//! This module provides functionality for sending newsletter emails
//! using the `lettre` crate for SMTP transport.
//!
//! ## Features
//! - Reads an HTML email template from a file.
//! - Replaces placeholders with actual content.
//! - Sends emails using SMTP with authentication.

use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::fs;
use std::path::Path;

/// Sends a newsletter email to a subscriber.
///
/// This function reads an email template from a file, replaces placeholders with actual values,
/// and sends an email using an SMTP server.
///
/// # Arguments
/// * `email` - The recipient's email address.
/// * `title` - The title of the newsletter article.
/// * `snippet` - A short snippet of the article.
/// * `article_link` - A URL to the full article.
/// * `unsubscribe_link` - A URL for the recipient to unsubscribe.
///
/// # Returns
/// * `Ok(())` on success.
/// * `Err(String)` if an error occurs while sending the email.
pub async fn send_newsletter_email(email: &str, title: &str, snippet: &str, article_link: &str, unsubscribe_link: &str) -> Result<(), String> {
     // Read the email template
     let template_path = Path::new("src/templates/email_template.html");
     let template_content = fs::read_to_string(template_path)
         .map_err(|err| format!("Failed to read email template: {}", err))?;
 
     // Replace placeholders with actual values
     let email_body = template_content
         .replace("{{ title }}", title)
         .replace("{{ snippet }}", snippet)
         .replace("{{ article_link }}", article_link)
         .replace("{{ unsubscribe_link }}", unsubscribe_link);

    // Construct the email message
    let email = Message::builder()
        .from("username".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("📢 New Article Notification")
        .header(ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    // Set up SMTP transport
    let mailer = SmtpTransport::starttls_relay("smtp.mailersend.net")
        .unwrap()
        .credentials(Credentials::new("username".to_string(),
         "password".to_string()))
        .port(587)
        .build();
    // Send the email
    mailer.send(&email).map_err(|e| e.to_string())?;
    Ok(())
}