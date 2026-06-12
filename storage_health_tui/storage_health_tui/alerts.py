from textual.widgets import ListView, ListItem, Label
from textual.widget import Widget
from textual.reactive import reactive


class AlertsPanel(Widget):
    disks: reactive = reactive([])

    def watch_disks(self, disks: list[dict]) -> None:
        list_view = self.query_one(ListView)
        list_view.clear()
        for disk in disks:
            if disk["temp_c"] > 50:
                list_view.append(
                    ListItem(Label(f"⚠ {disk['name']}: High temperature ({disk['temp_c']}°C)", style="yellow"))
                )
            if disk["reallocated_sectors"] > 0:
                list_view.append(
                    ListItem(
                        Label(f"⚠ {disk['name']}: Reallocated sectors({disk['reallocated_sectors']})", style="yellow")
                    )
                )
            if disk["smart_status"] == "FAILED":
                list_view.append(ListItem(Label(f"✖ {disk['name']}: SMART FAILED", style="red")))

    def compose(self):
        yield ListView()
