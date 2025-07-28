from __future__ import annotations


class CommandError(Exception):
    """Command return code was non-zero."""

    def __init__(
        self,
        returncode: int,
        stdout: bytes | None,
        stderr: bytes | None,
    ) -> None:
        super().__init__()
        self.returncode: int = returncode
        "Exit status of the child process."
        self.stdout: bytes | None = stdout
        "Captured standard output."
        self.stderr: bytes | None = stderr
        "Captured standard error."
