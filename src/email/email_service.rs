use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::fs;
use std::path::Path;


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

    let email = Message::builder()
        .from("MS_Qi9HwZ@trial-vywj2lp8qekg7oqz.mlsender.net".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("📢 New Article Notification")
        .header(ContentType::TEXT_HTML)
        .body(email_body)
        .map_err(|e| e.to_string())?;

    let mailer = SmtpTransport::starttls_relay("smtp.mailersend.net")
        .unwrap()
        .credentials(Credentials::new("MS_Qi9HwZ@trial-vywj2lp8qekg7oqz.mlsender.net".to_string(),
         "mssp.RuCBk2w.7dnvo4ded5nl5r86.oS6avvF".to_string()))
        .port(587)
        .build();

    //     let mailer = SmtpTransport::relay("smtp.gmail.com")
    //     .unwrap()
    //     .credentials(Credentials::new("rohitjire55".into(), "bgec ctjp ymnf wnyu".into()))
    //    // .port(2525)
    //     .build();

    mailer.send(&email).map_err(|e| e.to_string())?;
    Ok(())
}