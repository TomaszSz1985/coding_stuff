mod astronomy;
use astronomy::{JulianDate, Observer};
use clap::Parser;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use std::thread;

/// Formats a decimal hour value as `HH:MM` (e.g. `7.5` → `"07:30"`).
fn format_time(hours: f64) -> String {
    let h = hours as u32;
    let m = ((hours - h as f64) * 60.0) as u32;
    format!("{:02}:{:02}", h, m)
}
#[derive(Debug)]
struct DayResult {
    day: u32,
    rise: f64,
    set: f64,
}

#[derive(Parser)]
struct Args {
    /// Latitude (e.g. 52.23)
    #[arg(short, long)]
    lat: f64,

    /// Longitude (e.g. 21.01)
    #[arg(short = 'o', long)]
    lon: f64,

    #[arg(short = 'z', long, default_value_t = 0)]
    offset: i32,

    /// Year
    #[arg(short, long, default_value_t = 2024)]
    year: i32,

    /// Month
    #[arg(short, long, default_value_t = 1)]
    month: u32,

    /// Day
    #[arg(short, long, default_value_t = 1)]
    day: u32,

    #[arg(short = 'f', long)]
    output: Option<String>,

    #[arg(short = 'a', long)]
    all_month: bool,
}

/// Returns the number of days in the given month (February is always treated as 28 days).
fn days_in_month(_year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

/// Holds all per-day results for a full month along with the observer that produced them.
struct MonthReport<'a> {
    observer: &'a Observer,
    results: Vec<DayResult>,
}

impl<'a> MonthReport<'a> {
    /// Prints the observer coordinates followed by sunrise/sunset for every day.
    fn print(&self) {
        println!(
            "Results for: lat={}, lon={}",
            self.observer.latitude, self.observer.longitude
        );
        print_results(&self.results);
    }
}

/// Prints sunrise (yellow) and sunset (red) for each `DayResult` in the slice.
fn print_results(results: &[DayResult]) {
    for r in results {
        println!(
            "Day {}: Sunrise: {}, Sunset: {}.",
            r.day,
            format_time(r.rise).yellow(),
            format_time(r.set).red()
        );
    }
}

fn main() {
    let args = Args::parse();
    let observer = Observer {
        latitude: args.lat,
        longitude: args.lon,
    };

    if args.all_month {
        let days = days_in_month(args.year, args.month);
        // let mut results: Vec<DayResult> = Vec::new();
        let results = Arc::new(Mutex::new(Vec::<DayResult>::new()));

        let year = args.year;
        let month = args.month;
        let offset = args.offset as f64;
        let lat = observer.latitude;
        let lon = observer.longitude;
        let mut handles = vec![];

        for day in 1..=days {
            let mut res = Arc::clone(&results);

            let handle = thread::spawn(move || {
                let obs = Observer {
                    latitude: lat,
                    longitude: lon,
                };

                let jd = match JulianDate::from_calendar(year, month, day) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };

                let times = match jd.sun_times(&obs) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                };
                res.lock().unwrap().push(DayResult {
                    day,
                    rise: times.rise + offset,
                    set: times.set + offset,
                });
            });

            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
        let mut sorted = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        sorted.sort_by_key(|r| r.day);
        let report = MonthReport {
    observer: &observer,
    results: sorted,
};
        report.print();
    } else {
        println!("Observer {:#?}!", observer);
        println!("Is valid: {}", observer.is_valid());
        let jd = match JulianDate::from_calendar(args.year, args.month, args.day) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        let dec = jd.solar_declination();
        println!("Solar declination: {:.2}°", dec);

        let times = match jd.sun_times(&observer) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        };

        let offset: f64 = args.offset as f64;
        println!(
            "Sunrise: {}, Sunset: {}",
            format_time(times.rise + offset).yellow(),
            format_time(times.set + offset).red()
        );
        println!(
            "{}",
            format!("Moon phase: {:.1}%", jd.moon_phase()).blue()
        );
        let text: String = format!(
            "Sunrise: {}, Sunset: {}",
            format_time(times.rise + offset).yellow(),
            format_time(times.set + offset).red()
        );
        if let Some(path) = args.output {
            match std::fs::write(path, text) {
                Ok(_) => println!("Saved to file."),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
