# storage-health-tui

A terminal UI for monitoring disk health in real time. Built as a Python learning project using Textual and Rich.

![layout: disk table | detail panel / alerts bar]

## Features

- Disk table with health color coding (green / yellow / red)
- Detail panel showing full SMART data for a selected disk
- Alerts bar for high temperature, reallocated sectors, and SMART failures
- Live refresh every 5 seconds
- Keyboard shortcuts: `r` to refresh manually, `q` to quit

## Requirements

- Linux
- `smartmontools` installed (`sudo pacman -S smartmontools` on Arch/Manjaro)
- Python 3.11+
- Root privileges (smartctl requires sudo)

## Installation

```bash
pip install uv          # if not already installed
git clone https://github.com/TomaszSz1985/coding_stuff.git
cd coding_stuff/storage_health_tui
uv venv
source .venv/bin/activate
uv pip install -e .
```

## Usage

```bash
sudo .venv/bin/storage-health-tui
```

Or without installation:

```bash
sudo .venv/bin/python -m storage_health_tui.main
```

## Project structure

```
storage_health_tui/
├── main.py           — App, layout, keybindings, live refresh
├── disk_provider.py  — Real disk data via lsblk + smartctl (subprocess)
├── disk_table.py     — DiskTable widget with DataTable and health coloring
├── detail_panel.py   — DetailPanel widget showing selected disk details
├── alerts.py         — AlertsPanel with scrollable ListView
└── mock_data.py      — Fake disk data for development without root
```