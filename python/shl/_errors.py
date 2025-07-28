from __future__ import annotations


class CommandError(Exception):
    """Command return code was non-zero."""

    def __init__(self, returncode: int) -> None:
        super().__init__()
        self.returncode: int = returncode
        "Exit status of the child process."


class CommandOutputError(CommandError):
    """Command return code was non-zero. Stdout and stderr were captured."""

    def __init__(self, returncode: int, stdout: bytes, stderr: bytes) -> None:
        super().__init__(returncode)
        self.stdout: bytes = stdout
        "Captured standard output."
        self.stderr: bytes = stderr
        "Captured standard error."
