from __future__ import annotations

from enum import IntEnum
from typing import TYPE_CHECKING, cast

from .tshu import *  # noqa: F403

if TYPE_CHECKING:
    from collections.abc import Generator
    from pathlib import Path
    from string.templatelib import Template
    from typing import Any

    from ._completed_command import CompletedCommand


class _Enc(IntEnum):
    RETURNCODE = 1
    OUTPUT = 2
    TEXT = 3
    BYTES = 4
    JSON = 5
    YAML = 6
    TOML = 7


class Command[T = int]:
    """Awaitable shell command.

    By default, awaiting a command returns the exit-code. Use one of the methods such
    as `.output()` to modify the return value.

    Examples:
        >>> returncode = await sh(t"echo hello world")
        hello world
        >>> returncode
        0

    """

    quiet: bool = False
    "Suppress stdout and stderr from displayed in the terminal. Globally affects all command invocations."

    check: bool = True
    "Raise exception when command return code is non-zero. Globally affects all command invocations."

    cwd: str | Path | None = None
    "Shell's working directory. (Defaults to current working directory) Globally affects all command invocations."

    def __init__(
        self,
        command: Template,
        *,
        quiet: bool | None = None,
        check: bool | None = None,
        input: str | bytes | None = None,
        cwd: str | Path | None = None,
        env: dict[str, str] | None = None,
    ) -> None:
        """Construct a command.

        Args:
            command: t-string template for command.
            quiet: Suppress stdout and stderr from displayed in the terminal.
            check: Raise exception when command return code is non-zero.
            input: Pass standard input to command.
            cwd: Shell's working directory.
            env: Dictionary of environment variables.

        """
        self._template: Template = command
        self._quiet: bool = self.quiet if quiet is None else quiet
        self._check: bool = self.check if check is None else check
        self._input: str | bytes | None = input
        cwd = self.cwd if cwd is None else cwd
        self._cwd: str | None = None if cwd is None else str(cwd)
        self._env: dict[str, str] | None = env
        self._enc: _Enc = _Enc.RETURNCODE

    def output(self) -> Command[CompletedCommand]:
        r"""Capture stdout and stderr as bytes.

        Examples:
            >>> await sh(t"echo hello").output()
            CompletedCommand(returncode=0, stdout=b'hello\n', stderr=b'')

        """
        self._enc = _Enc.OUTPUT
        return cast("Any", self)

    def text(self) -> Command[str]:
        """Stdout returns as string.

        Examples:
            >>> await sh(t"echo hello").text()
            hello

        """
        self._enc = _Enc.TEXT
        return cast("Any", self)

    def bytes(self) -> Command[str]:
        r"""Stdout returns as bytes.

        Examples:
            >>> await sh(t"echo hello").bytes()
            b'hello\n'

        """
        self._enc = _Enc.BYTES
        return cast("Any", self)

    def json(self) -> Command[Any]:
        """Stdout returns as JSON.

        Examples:
            >>> await sh(t"echo {json.dumps({"hello": "world"})}").json()
            {'hello': 'world'}

        """
        self._enc = _Enc.JSON
        return cast("Any", self)

    def yaml(self) -> Command[Any]:
        """Stdout returns as YAML."""
        self._enc = _Enc.YAML
        return cast("Any", self)

    def toml(self) -> Command[Any]:
        """Stdout returns as TOML."""
        self._enc = _Enc.TOML
        return cast("Any", self)

    def __await__(self) -> Generator[None, None, T]:
        return _execute_command(self).__await__()  # pyright: ignore[reportUnknownMemberType, reportUnknownVariableType, reportUndefinedVariable]  # noqa: F405
