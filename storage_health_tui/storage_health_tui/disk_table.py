from textual.widgets import DataTable
from textual.app import ComposeResult
from textual.widget import Widget

from .disk_provider import get_disks
from rich.text import Text


class DiskTable(Widget):
    COLUMNS = [
        "name",
        "model",
        "size_gb",
        "temp_c",
        "health",
        "smart_status",
        "reallocated_sectors",
        "io_read_mb",
        "io_write_mb",
    ]

    def compose(self) -> ComposeResult:
        yield DataTable()

    def check_health(self, value, status) -> Text:
        result = None
        if value < 50 or status == "FAILED":
            result = Text(f"{value}", style="red")
        elif value >= 70:
            result = Text(f"{value}", style="green")
        elif value >= 50:
            result = Text(f"{value}", style="yellow")

        return result

    def on_mount(self) -> None:
        table = self.query_one(DataTable)
        table.cursor_type = "row"
        table.add_columns(*self.COLUMNS)
        for row in get_disks():
            health_cell = self.check_health(row["health"], row["smart_status"])
            table.add_row(
                row["name"],
                row["model"],
                row["size_gb"],
                row["temp_c"],
                health_cell,
                row["smart_status"],
                row["reallocated_sectors"],
                row["io_read_mb"],
                row["io_write_mb"],
            )

    def reload(self, disks: list[dict]) -> None:
        table = self.query_one(DataTable)
        table.clear()
        for row in disks:
            health_cell = self.check_health(row["health"], row["smart_status"])
            table.add_row(
                row["name"],
                row["model"],
                row["size_gb"],
                row["temp_c"],
                health_cell,
                row["smart_status"],
                row["reallocated_sectors"],
                row["io_read_mb"],
                row["io_write_mb"],
            )


if __name__ == "__main__":
    app = DiskTable()
    app.run()
