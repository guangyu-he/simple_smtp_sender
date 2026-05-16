from typing import List, Self, Dict

from pydantic import BaseModel

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

    @classmethod
    def load_from_env(cls) -> Self:
        """
        Load EmailConfig from environment variables.

        Returns:
            EmailConfig object
        """
        ...

    @classmethod
    def load_from_map(cls, config_map: Dict[str, str]) -> Self:
        """
        Load EmailConfig from a dictionary.

        Args:
            config_map: Dictionary containing configuration parameters
        """
        ...

    @classmethod
    def load_from_pydantic(cls, pydantic_obj: "BaseModel") -> Self:
        """
        Load EmailConfig from a Pydantic BaseModel.

        Requirements:
            `pydantic` must be installed in the active Python environment and
            this code must be executed inside that environment (e.g. the
            project's virtualenv). If `pydantic` cannot be imported, a
            ``RuntimeError`` is raised with the underlying import error.

        Args:
            pydantic_obj: Pydantic BaseModel containing configuration parameters

        Returns:
            EmailConfig instance populated from the Pydantic BaseModel.

        Raises:
            RuntimeError: If ``pydantic`` cannot be imported from the current
                Python environment.
            TypeError: If ``pydantic_obj`` is not an instance of
                ``pydantic.BaseModel``.
            ValueError: If the model's fields cannot be deserialized into an
                ``EmailConfig`` (e.g. missing or mistyped fields).
        """
        ...

    def to_dict(self) -> Dict[str, str]:
        """
        Convert EmailConfig to a Python dictionary.

        Returns:
            Dictionary containing the configuration parameters
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

async def async_send_email(
    config: EmailConfig,
    recipient: List[str],
    subject: str,
    body: str,
    cc: List[str] | None = None,
    bcc: List[str] | None = None,
    attachment: str | None = None,
) -> None:
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
