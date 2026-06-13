# syswatch

A terminal system monitor written in Rust as a learning project.

Displays real-time CPU usage, RAM usage, and a list of running processes in a TUI (terminal user interface) that refreshes automatically.

## Usage

```
cargo run
```

Press `q` to quit.

## Example output

```
┌ CPU ────────────────────────────────┐
│ ████████░░░░░░░░░░░░░░░  32.4%      │
└─────────────────────────────────────┘
┌ RAM ────────────────────────────────┐
│ ████████████████░░░░░░░  5821 MB    │
└─────────────────────────────────────┘
┌ Processes ──────────────────────────┐
│ Process                             │
│ firefox 1234                        │
│ code 5678                           │
│ ...                                 │
└─────────────────────────────────────┘
```

## What I learned

- `ratatui` — TUI framework: Layout, Gauge, Table widgets, double-buffered rendering
- `crossterm` — terminal raw mode, non-blocking keyboard event polling
- `sysinfo` — reading CPU usage, RAM stats, and process list from the OS
- Event loop pattern — draw → poll input → update state → repeat
- `event::poll(Duration)` — waiting for input with a timeout (non-blocking)
- `&mut` references — passing mutable state through function boundaries
- `HashMap` iteration — `.iter().take(n).map().collect()`
- `clamp(0.0, 1.0)` — guarding against NaN/out-of-range values

## Dependencies

| Crate     | Purpose                        |
|-----------|--------------------------------|
| ratatui   | TUI widgets and rendering      |
| crossterm | terminal control + keyboard    |
| sysinfo   | system metrics (CPU, RAM, PIDs)|

## Project structure

```
src/
├── main.rs     — event loop, terminal setup and teardown
├── app.rs      — App struct holding all application state
├── system.rs   — reads data from sysinfo into App
└── ui.rs       — renders widgets using ratatui
```
