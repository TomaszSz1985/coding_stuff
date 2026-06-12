from textual.widgets import Static
from textual.reactive import reactive


class DetailPanel(Static):
    selected_disk = reactive(None)

    def render(self):
        if self.selected_disk:
            return (
                f"Name:   {self.selected_disk['name']}\n"
                f"Model:  {self.selected_disk['model']}\n"
                f"Size_gb:  {self.selected_disk['size_gb']}\n"
                f"temp_c:  {self.selected_disk['temp_c']}\n"
                f"health:  {self.selected_disk['health']}\n"
                f"smart_status:  {self.selected_disk['smart_status']}\n"
                f"reallocated_sectors:  {self.selected_disk['reallocated_sectors']}\n"
                f"io_read_mb:  {self.selected_disk['io_read_mb']}\n"
                f"io_write_mb:  {self.selected_disk['io_write_mb']}\n"
            )
        else:
            return "Select a disk"
