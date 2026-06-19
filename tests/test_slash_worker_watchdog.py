import io
from typing import Any

import psutil
from rich.panel import Panel
from rich.text import Text

from tui_gateway import slash_worker


class _FakeSlashCli:
    console: Any

    def __init__(self):
        self.console = None

    def process_command(self, command):
        self.console.print(Panel(Text("Previous Conversation"), title="Resume"))


def test_run_strips_rich_color_escapes_from_slash_output():
    cli_obj = _FakeSlashCli()

    output = slash_worker._run(cli_obj, "/resume")  # type: ignore[arg-type]

    assert "\x1b[" not in output
    assert "Previous Conversation" in output
    assert "Resume" in output


def test_is_orphaned_true_when_ppid_changes():
    # Our parent went away and we were reparented to a subreaper/init.
    assert slash_worker._is_orphaned(1234, 1.0, getppid=lambda: 999999) is True


def test_is_orphaned_true_when_parent_create_time_mismatch():
    # Same ppid but a different create_time means the PID was reused.
    me = psutil.Process()
    assert slash_worker._is_orphaned(me.pid, 0.0, getppid=lambda: me.pid) is True


def test_is_orphaned_false_when_parent_alive_and_matches():
    me = psutil.Process()
    assert (
        slash_worker._is_orphaned(me.pid, me.create_time(), getppid=lambda: me.pid) is False
    )
