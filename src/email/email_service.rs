use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};


pub async fn send_newsletter_email(email: &str, title: &str, snippet: &str, article_link: &str, unsubscribe_link: &str) -> Result<(), String> {
    let html_body = format!(
        r#"
        <html>
            <body>
                <h2>New Article: {}</h2>
                <p>{}...</p>
                <a href="{}">Read More</a>
                <br><br>
                <a href="{}" style="color: red;">Unsubscribe</a>
            </body>
        </html>
        "#,
        title, snippet, article_link, unsubscribe_link
    );

    let email = Message::builder()
        .from("MS_Qi9HwZ@trial-vywj2lp8qekg7oqz.mlsender.net".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("New Article Notification")
        .header(ContentType::TEXT_HTML)
        .body(html_body)
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