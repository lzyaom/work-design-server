use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

use crate::error::AppError;

pub struct EmailService {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
}

impl EmailService {
    pub fn new(
        smtp_host: &str,
        smtp_username: &str,
        smtp_password: &str,
        from_address: String,
    ) -> Result<Self, AppError> {
        let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

        let smtp_transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
            .credentials(creds)
            .build();

        Ok(Self {
            smtp_transport,
            from_address,
        })
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        html_content: &str,
    ) -> Result<(), AppError> {
        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_content.to_string())?;

        self.smtp_transport.send(email).await?;
        Ok(())
    }
}
