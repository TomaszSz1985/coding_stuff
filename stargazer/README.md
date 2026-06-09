# stargazer

A CLI tool for calculating sunrise/sunset times and moon phase for a given location and date. Built as a Rust learning project.

## Usage

```
stargazer --lat <latitude> --lon <longitude> --year <year> --month <month> --day <day>
```

Add `--all-month` (`-a`) to print results for every day of the given month in parallel.  
Add `--offset <hours>` (`-z`) to apply a UTC offset.  
Add `--output <file>` (`-f`) to save the result to a file.

Example:
```
stargazer --lat 52.23 --lon 21.01 --year 2024 --month 6 --day 21
```

## What I learned

- **Modules and structs** — splitting code into `mod astronomy` with its own public types (`Observer`, `JulianDate`, `SunTimes`)
- **Error handling** — using `Result<T, String>` and `Result<T, E>` with `match` and `?` propagation
- **CLI parsing with `clap`** — derive macros for argument definitions including doc-comment help strings
- **Colored terminal output** — using the `colored` crate to style stdout
- **Multithreading** — spawning one thread per day with `thread::spawn`, sharing results through `Arc<Mutex<Vec<_>>>`, joining handles, and using `Arc::try_unwrap` to reclaim the value
- **Julian Date astronomy** — implementing solar declination, hour-angle sunrise/sunset formula, and a moon phase approximation from scratch
- **Unit tests** — writing `#[test]` functions inside `mod tests` with `assert!` and floating-point tolerance checks
