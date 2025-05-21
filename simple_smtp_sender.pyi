class EmailConfig:
    def __new__(cls, server: str, sender_email: str, username: str, password: str): ...


def send_email(config: EmailConfig, recipient: str, subject: str, body: str, cc: str | None = None,
               bcc: str | None = None,
               attachment: str | None = None) -> str: ...
