"""Entry point — uruchom: python -m storage_health_tui.main"""

from textual.app import App, Binding
from textual.widgets import DataTable
from .disk_table import DiskTable
from .detail_panel import DetailPanel
from .alerts import AlertsPanel
from .disk_provider import get_disks
from textual.containers import Horizontal


class StorageHealthApp(App):
    """Main TUI application — do zaimplementowania."""

    BINDINGS = [
        Binding("q", "quit", "Quit"),
        Binding("r", "refresh", "Refresh"),
    ]

    DEFAULT_CSS = """
    Horizontal {
        height: 1fr;
    }
    DiskTable {
    width: 2fr;
    height: 100%;
    }
    DetailPanel {
        width: 1fr;
        height: 100%;
        border: solid green;
    }
    AlertsPanel {
        height: 5;
        border: solid red;
    }
    """

    def action_refresh(self) -> None:
        self.refresh_data()

    def on_data_table_row_selected(self, event: DataTable.RowSelected) -> None:
        panel = self.query_one(DetailPanel)
        panel.selected_disk = self.disks[event.cursor_row]

    def compose(self):
        with Horizontal():
            yield DiskTable(classes="box")
            yield DetailPanel(classes="box")
        yield AlertsPanel()

    def on_mount(self) -> None:
        self.disks = get_disks()

        self.set_interval(5, self.refresh_data)

    def refresh_data(self):
        self.disks = get_disks()

        self.query_one(DiskTable).reload(self.disks)
        self.query_one(AlertsPanel).disks = self.disks
        self.query_one(DetailPanel).refresh()


def main():
    StorageHealthApp().run()


if __name__ == "__main__":
    main()
