mod tests {
    use simple_smtp_sender::EmailConfig;

    #[test]
    fn email_config_test() {
        let config = EmailConfig::new(
            "smtp.example.com",
            "your@email.com",
            "your_username",
            "your_password",
        );
        assert_eq!(config.server, "smtp.example.com");
        assert_eq!(config.sender_email, "your@email.com");
        assert_eq!(config.username, "your_username");
        assert_eq!(config.password, "your_password");
    }

    #[test]
    fn email_config_from_env_test() {
        unsafe {
            std::env::set_var("EMAIL_SERVER", "smtp.env.com");
            std::env::set_var("EMAIL_SENDER_EMAIL", "example@example.com");
            std::env::set_var("EMAIL_USERNAME", "env_user");
            std::env::set_var("EMAIL_PASSWORD", "env_pass");
        }
        let config = EmailConfig::from_env();
        assert_eq!(config.server, "smtp.env.com");
        assert_eq!(config.sender_email, "example@example.com");
        assert_eq!(config.username, "env_user");
        assert_eq!(config.password, "env_pass");
    }

    #[test]
    fn email_config_from_hashmap_test() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("server".to_string(), "smtp.map.com".to_string());
        map.insert(
            "sender_email".to_string(),
            "example@example.com".to_string(),
        );
        map.insert("username".to_string(), "map_user".to_string());
        map.insert("password".to_string(), "map_pass".to_string());
        let config = EmailConfig::from(map);
        assert_eq!(config.server, "smtp.map.com");
        assert_eq!(config.sender_email, "example@example.com");
        assert_eq!(config.username, "map_user");
        assert_eq!(config.password, "map_pass");
    }

    #[test]
    fn email_config_display_test() {
        let config = EmailConfig::new(
            "smtp.example.com",
            "your@email.com",
            "your_username",
            "your_password",
        );
        let display = format!("{}", config);
        assert_eq!(
            display,
            "EmailConfig<server=smtp.example.com, sender_email=your@email.com, username=your_username, password=your_password>"
        );
    }

    #[test]
    fn email_client_default_test() {
        use simple_smtp_sender::EmailClient;
        let client = EmailClient::default();
        assert_eq!(client.recipient.len(), 0);
        assert_eq!(client.subject, "No subject".to_string());
        assert!(client.body.is_none());
        assert!(client.cc.is_none());
        assert!(client.bcc.is_none());
        assert!(client.attachment.is_none());
    }

    #[test]
    fn email_client_new_test() {
        use simple_smtp_sender::EmailClient;
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("server".to_string(), "smtp.map.com".to_string());
        map.insert(
            "sender_email".to_string(),
            "example@example.com".to_string(),
        );
        map.insert("username".to_string(), "map_user".to_string());
        map.insert("password".to_string(), "map_pass".to_string());
        let client = EmailClient::new(map);
        assert_eq!(client.clone().config.server, "smtp.map.com");
        assert!(client.recipient.is_empty());
        assert_eq!(client.subject, "No subject".to_string());
        assert!(client.body.is_none());
    }

    #[test]
    fn send_email_sync_test_wrong_pass() {
        use simple_smtp_sender::send_email_sync;
        let mut config = EmailConfig::from_env();
        config.password = "wrong_password".to_string();
        let recipient = vec!["guangyu.he@golden-tech.de".to_string()];
        let subject = "Test Email".to_string();
        let body = "Hello from Rust!".to_string();
        let result = send_email_sync(config, recipient, subject, body, None, None, None);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn send_email_async_test_wrong_pass() {
        use simple_smtp_sender::send_email_async;
        let mut config = EmailConfig::from_env();
        config.password = "wrong_password".to_string();
        let recipient = vec!["guangyu.he@golden-tech.de".to_string()];
        let subject = "Test Email Async".to_string();
        let body = "Hello from Rust!".to_string();
        let result = send_email_async(config, recipient, subject, body, None, None, None).await;
        assert!(result.is_err());
    }

    #[test]
    fn send_email_with_builder_rest() {
        use simple_smtp_sender::EmailClient;
        let config = EmailConfig::from_env();

        EmailClient::new(config)
            .recipient(vec!["guangyu.he@golden-tech.de".to_string()])
            .subject("Test Email Builder")
            .body("Hello from Rust Email Builder!")
            .send()
            .expect("Failed to send email using builder");
    }

    #[tokio::test]
    async fn send_email_with_builder_async_rest() {
        use simple_smtp_sender::EmailClient;
        let config = EmailConfig::from_env();

        EmailClient::new(config)
            .recipient(vec!["guangyu.he@golden-tech.de".to_string()])
            .subject("Test Email Builder")
            .body("Hello from Rust Email Builder!")
            .send_async()
            .await
            .expect("Failed to send email using builder");
    }
}
