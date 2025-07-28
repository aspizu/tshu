from __future__ import annotations

from dataclasses import dataclass


@dataclass
class CompletedCommand:
    returncode: int
    "Exit status of the child process. Typically, an exit status of 0 indicates that it ran successfully."
    stdout: bytes
    "Captured standard output."
    stderr: bytes
    "Captured standard error."
