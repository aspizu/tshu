"""Run secure and cross-platform shell commands."""

from __future__ import annotations

from ._command import Command
from ._command_error import CommandError
from ._completed_command import CompletedCommand

sh = Command

__all__ = ["Command", "CommandError", "CompletedCommand", "sh"]
