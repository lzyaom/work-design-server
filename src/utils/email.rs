use crate::error::AppError;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct EmailService {
    transport: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    from_address: String,
}

impl EmailService {
    pub fn new(
        smtp_host: &str,
        username: &str,
        password: &str,
        from_address: &str,
    ) -> Result<Self, AppError> {
        let creds = Credentials::new(username.to_string(), password.to_string());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)
            .map_err(|e| AppError::Configuration(format!("SMTP setup error: {}", e)))?
            .credentials(creds)
            .build();

        Ok(EmailService {
            transport: Arc::new(transport),
            from_address: from_address.to_string(),
        })
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppError> {
        let email = Message::builder()
            .from(
                self.from_address
                    .parse()
                    .map_err(|e| AppError::Configuration(format!("Invalid from address: {}", e)))?,
            )
            .to(to
                .parse()
                .map_err(|e| AppError::Configuration(format!("Invalid to address: {}", e)))?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body.to_string())
            .map_err(|e| AppError::Configuration(format!("Email build error: {}", e)))?;

        self.transport
            .send(email)
            .await
            .map_err(|e| AppError::External(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}
