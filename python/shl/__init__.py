"""Run secure and cross-platform shell commands."""

from __future__ import annotations

from ._command import Command
from ._completed_command import CompletedCommand
from ._errors import CommandError, CommandOutputError

sh = Command

__all__ = ["Command", "CommandError", "CommandOutputError", "CompletedCommand", "sh"]
