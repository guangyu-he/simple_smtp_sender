from typing import List, Awaitable

class EmailConfig:
    """
    Configuration for sending emails.
    """

    def __new__(cls, server: str, sender_email: str, username: str, password: str):
        """
        Create a new EmailConfig object.

        Args:
            server: SMTP server URL
            sender_email: Email address of the sender
            username: Login username
            password: Login password

        Returns:
            EmailConfig object

        Note: This method is used to create a new EmailConfig object.
        """
        ...

def send_email(
    config: EmailConfig,
    recipient: List[str],
    subject: str,
    body: str,
    cc: List[str] | None = None,
    bcc: List[str] | None = None,
    attachment: str | None = None,
) -> None:
    """
    Send an email.

    Args:
        config: EmailConfig object containing server configuration
        recipient: Email address of the recipient
        subject: Email subject
        body: Email body
        cc: Email address of the CC recipient (optional)
        bcc: Email address of the BCC recipient (optional)
        attachment: Path to the file to attach to the email (optional)

    Returns:
        None if the email is sent successfully, otherwise error message
    """
    ...

def async_send_email(
    config: EmailConfig,
    recipient: List[str],
    subject: str,
    body: str,
    cc: List[str] | None = None,
    bcc: List[str] | None = None,
    attachment: str | None = None,
) -> Awaitable[None]:
    """
    Asynchronously send an email.

    Args:
        config: EmailConfig object containing server configuration
        recipient: Email address of the recipient
        subject: Email subject
        body: Email body
        cc: Email address of the CC recipient (optional)
        bcc: Email address of the BCC recipient (optional)
        attachment: Path to the file to attach to the email (optional)

    Returns:
        None if the email is sent successfully, otherwise error message
    """
    ...
