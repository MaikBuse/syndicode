use crate::application::ports::verification::{
    VerificationSendable, VerificationSendableError, VerificationSendableResult,
};
use crate::config::ServerConfig;
use crate::domain::user_verify::model::code::VerificationCode;
use lettre::message::{header::ContentType, Mailbox, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::PoolConfig;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

const LOGO_URL: &str =
    "https://raw.githubusercontent.com/MaikBuse/syndicode/refs/heads/main/images/logo.svg";
const BANNER_URL: &str =
    "https://raw.githubusercontent.com/MaikBuse/syndicode/refs/heads/main/images/hero.png";
const FOOTER_IMAGE_URL: &str =
    "https://raw.githubusercontent.com/MaikBuse/syndicode/refs/heads/main/images/gameplay/warfare.png";

const SENDER_NAME: &str = "Syndicode Verification";
const EMAIL_SUBJECT: &str = "Syndicode Account Verification Required";

pub struct EmailHandler {
    sender_mailbox: Mailbox,
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailHandler {
    pub fn new(config: Arc<ServerConfig>) -> anyhow::Result<Self> {
        // Handle potential parsing error for Mailbox
        let sender_mailbox: Mailbox = format!("{} <{}>", SENDER_NAME, config.email.sender_email)
            .parse()
            .unwrap();

        let sender_credentials = Credentials::new(
            config.email.smtp_username.clone(),
            config.email.smtp_password.clone(),
        );

        let pool_config = PoolConfig::new()
            .max_size(10) // Max number of connections in the pool
            .min_idle(2) // Keep at least 2 idle connections open
            .idle_timeout(Duration::from_secs(5 * 60)); // Close connections idle for 5 min

        // Build the Mailer with Pooling
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(config.email.smtp_server.as_str())
            .map_err(|err| VerificationSendableError::InitSMTP(err.to_string()))?
            .credentials(sender_credentials)
            .pool_config(pool_config)
            .build();

        Ok(Self {
            sender_mailbox,
            mailer,
        })
    }

    // Function to generate the styled verification code HTML
    // This makes it harder to just grab digits (e.g., `\d{6}`)
    fn generate_code_html(&self, code: &str) -> String {
        // Start with an empty String
        code.chars().fold(String::new(), |mut output, c| {
        // Use the write! macro to append the formatted string directly
        // to the `output` buffer. write! is efficient for this.
        // We ignore the Result because writing to a String should not fail.
        let _ = write!(
            &mut output,
            r#"<span style="display: inline-block; border: 1px solid #555; padding: 5px 8px; margin: 0 3px; background-color: #1a1a1a; color: #00ffff; font-size: 1.5em; font-weight: bold; font-family: 'Courier New', Courier, monospace; min-width: 20px; text-align: center;">{}</span>"#,
            c
        );
        // Return the modified string buffer for the next iteration
        output
    })
    }

    // Function to generate the full HTML body
    fn create_html_body(&self, verification_code: &str) -> String {
        let styled_code = self.generate_code_html(verification_code);
        // Get current year using the `time` crate
        let current_year = OffsetDateTime::now_utc().year(); // Gets year as i32

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Syndicode Email Verification</title>
    <style>
        body {{ margin: 0; padding: 0; background-color: #0a0a0a; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }}
        .container {{ max-width: 600px; margin: 20px auto; background-color: #121212; color: #e0e0e0; border: 1px solid #333; box-shadow: 0 0 15px rgba(0, 255, 255, 0.1); }}
        .header {{ padding: 20px; text-align: center; border-bottom: 1px solid #333; }}
        .header img.logo {{ max-width: 100px; height: auto; margin-bottom: 10px; }}
        .banner img {{ width: 100%; height: auto; display: block; }}
        .content {{ padding: 30px; line-height: 1.6; }}
        .content h1 {{ color: #00ffff; font-weight: normal; margin-top: 0; text-shadow: 0 0 5px rgba(0, 255, 255, 0.5);}}
        .code-container {{ margin: 25px 0; text-align: center; }}
        .code-label {{ display: block; margin-bottom: 10px; font-size: 0.9em; color: #aaa; text-transform: uppercase; letter-spacing: 1px; }}
        .footer {{ padding: 20px; text-align: center; font-size: 0.8em; color: #777; border-top: 1px solid #333; }}
        .footer a {{ color: #ff004f; text-decoration: none; }}
        .footer a:hover {{ text-decoration: underline; }}
        /* Code span styles are inline for better compatibility */
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <img src="{logo_url}" alt="Syndicode Logo" class="logo" width="160" height="160">
            <h2 style="color: #ff004f; margin: 0; font-weight: normal;">// Authentication Sequence Initiated //</h2>
        </div>
        <div class="banner">
            <img src="{banner_url}" alt="Syndicode Network">
        </div>
        <div class="content">
            <h1>Access Protocol: Verify Identity</h1>
            <p>Welcome, operative. Your registration request has been logged. To establish secure connection and activate your Syndicode account, please verify your designation using the following transmission sequence:</p>
            <div class="code-container">
                <span class="code-label">// Verification Sequence //</span>
                {styled_code}
            </div>
            <p>Enter this sequence in the verification terminal. This code is mission-critical and expires shortly. Do not compromise the sequence.</p>
            <p>If you did not initiate this request, disregard this transmission. Your security protocols remain active.</p>
            <p>Stay vigilant,<br/>The Syndicode Network</p>
        </div>
        <div class="footer">
            <img src="{footer_image_url}" alt="Syndicode Environment" style="max-width: 100%; height: auto; margin-bottom: 15px;">
            <p>© {current_year} Syndicode. All rights reserved. Secure connection established.</p>
        </div>
    </div>
</body>
</html>"#,
            logo_url = LOGO_URL,
            banner_url = BANNER_URL,
            footer_image_url = FOOTER_IMAGE_URL,
            styled_code = styled_code,
            current_year = current_year
        )
    }
}

#[tonic::async_trait]
impl VerificationSendable for EmailHandler {
    async fn send_verification_email(
        &self,
        recipient_email: String,
        recipient_name: String,
        verification_code: VerificationCode,
    ) -> VerificationSendableResult<()> {
        let html_body = self.create_html_body(verification_code.get_code());

        let recipient_mailbox: Mailbox = format!("{} <{}>", recipient_name, recipient_email)
            .parse::<Mailbox>()
            .map_err(|err| VerificationSendableError::ParseRecipient(err.to_string()))?;

        let email = Message::builder()
            .from(self.sender_mailbox.clone())
            .to(recipient_mailbox)
            .subject(EMAIL_SUBJECT)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html_body), // Pass the String body
            )
            .map_err(|err| VerificationSendableError::BuildEmail(err.to_string()))?;

        // Send the email
        tracing::debug!("Sending verification email to {}...", recipient_email);

        self.mailer
            .send(email)
            .await
            .map_err(|err| VerificationSendableError::SendEmail(err.to_string()))?;

        tracing::debug!("Email sent successfully!");
        Ok(())
    }
}
