# traid

A RAID 5 array simulator that works with local binary files as virtual disks. Built as a Rust learning project.

## Usage

```
traid init  --disks <n> --block-size <bytes> --disk-size <bytes>
traid write --data <hex string>
traid read  --offset <bytes> --len <bytes>
traid fail  --disk <index>
traid rebuild --disk <index>
traid status
```

Example:
```
traid init --disks 3 --block-size 4 --disk-size 16
traid write --data deadbeef
traid read  --offset 0 --len 4
traid fail  --disk 0
traid read  --offset 0 --len 4   # degraded read, data recovered from parity
traid rebuild --disk 0
```

## What I learned

- **Custom error types with `thiserror`** — defining an enum that wraps `std::io::Error` and `serde_json::Error` with `#[from]`, and implementing `Display` via the `#[error("...")]` macro
- **Serialization with `serde` / `serde_json`** — deriving `Serialize`/`Deserialize` on a config struct and persisting it to a JSON file
- **File I/O with seek** — using `OpenOptions`, `Seek`, `SeekFrom`, `Read`, and `Write` traits to do random-access reads and writes on binary disk images
- **RAID 5 XOR parity** — implementing stripe layout, parity calculation, degraded-mode reads (recovering missing data from surviving disks + parity), and full disk rebuild
- **Library vs binary crate** — separating logic into `src/lib.rs` so it can be imported by integration tests, while `src/main.rs` only handles the CLI layer
- **Integration tests** — writing tests in `tests/integration_tests.rs` that exercise the full init → write → fail → read → rebuild cycle, using a `Mutex` to serialize tests that share files on disk
- **`clap` subcommands** — modeling commands as an enum with `#[derive(Subcommand)]` and per-variant argument structs
