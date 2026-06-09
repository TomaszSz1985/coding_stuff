use std::f64::consts::PI;

/// Geographic position of the observer used as input for astronomical calculations.
#[derive(Debug)]
pub struct Observer {
    /// Latitude in decimal degrees, positive north (−90 to +90).
    pub latitude: f64,
    /// Longitude in decimal degrees, positive east (−180 to +180).
    pub longitude: f64,
}

/// UTC sunrise and sunset times for a given date and observer.
pub struct SunTimes {
    /// Sunrise time as a decimal hour (e.g. 7.5 = 07:30 UTC).
    pub rise: f64,
    /// Sunset time as a decimal hour.
    pub set: f64,
}

/// Wrapper around a Julian Day Number (continuous count of days since noon 1 Jan 4713 BC).
pub struct JulianDate(pub f64);

/// Sine of an angle given in degrees.
fn sin_deg(deg: f64) -> f64 {
    deg.to_radians().sin()
}

/// Cosine of an angle given in degrees.
fn cos_deg(deg: f64) -> f64 {
    deg.to_radians().cos()
}

/// Arcsine returning the result in degrees.
fn asin_deg(x: f64) -> f64 {
    x.asin().to_degrees()
}

impl JulianDate {
    /// Converts a proleptic Gregorian calendar date to a Julian Day Number.
    ///
    /// Returns `Err` if `month` is outside 1–12 or `day` is outside 1–31.
    pub fn from_calendar(year: i32, month: u32, day: u32) -> Result<JulianDate, String> {
        // formula: JD = 367*Y - INT(7*(Y+INT((M+9)/12))/4) + INT(275*M/9) + D + 1721013.5
        // simplified: month must be 1-12, day 1-31
        // if invalid, return Err("...")
        // if ok, return Ok(JulianDate(...))
        if month < 1 || month > 12 {
            return Err(String::from("Month must be 1-12"));
        }
        if day < 1 || day > 31 {
            return Err(String::from("Day must be 1-31"));
        }

        let y = year;
        let m = month as i32;
        let d = day as i32;

        let jd = 367 * y - 7 * (y + (m + 9) / 12) / 4 + (275 * m / 9) + d;

        Ok(JulianDate(jd as f64 + 1721013.5))
    }

    /// Returns the Sun's declination in degrees for this Julian Date.
    ///
    /// Uses a low-precision approximation (accurate to ~0.01°) based on mean anomaly
    /// and ecliptic longitude.
    pub fn solar_declination(&self) -> f64 {
        let n = self.0 - 2451545.0;

        // mean solar anomaly (in degrees)
        let mean_anomaly = (357.529 + 0.98560028 * n) % 360.0;

        // mean ecliptic longitude
        let mean_longitude = (280.459 + 0.98564736 * n) % 360.0;

        // ecliptic longitude of the Sun (approximation)
        let lambda =
            mean_longitude + 1.915 * sin_deg(mean_anomaly) + 0.020 * sin_deg(2.0 * mean_anomaly);

        // declination
        asin_deg(0.39779 * sin_deg(lambda))
    }

    /// Computes UTC sunrise and sunset times for the given observer on this Julian Date.
    ///
    /// Returns `Err` when the Sun does not cross the horizon (polar day / polar night).
    pub fn sun_times(&self, observer: &Observer) -> Result<SunTimes, String> {
        let dec = self.solar_declination();
        let lat = observer.latitude;

        // hour angle of sunrise/sunset
        let cos_h = -(sin_deg(dec) * sin_deg(lat)) / (cos_deg(dec) * cos_deg(lat));

        if cos_h < -1.0 || cos_h > 1.0 {
            return Err(String::from(
                "The Sun does not rise or set on this day",
            ));
        }

        let h = cos_h.acos().to_degrees(); // hour angle in degrees

        // local noon in UTC hours
        let noon = 12.0 - observer.longitude / 15.0;

        let rise = noon - h / 15.0;
        let set = noon + h / 15.0;

        Ok(SunTimes { rise, set })
    }

    /// Returns the current moon illumination as a percentage (0 = new moon, 100 = full moon).
    ///
    /// Based on the synodic period of 29.53 days anchored to a known new-moon Julian Date.
    pub fn moon_phase(&self) -> f64 {
        let known_phase = 2451549.5;
        let phase = (self.0 - known_phase) % 29.53059;
        (1.0 - (phase / 29.53059 * 2.0 * PI).cos()) / 2.0 * 100.0
    }
}

impl Observer {
    /// Returns `true` if both latitude (−90 to +90) and longitude (−180 to +180) are in range.
    pub fn is_valid(&self) -> bool {
        self.latitude >= -90.0
            && self.latitude <= 90.0
            && self.longitude >= -180.0
            && self.longitude <= 180.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julian_date() {
        let result: JulianDate = JulianDate::from_calendar(2000, 01, 01).unwrap();
        assert!((result.0 - 2451544.5).abs() < 0.001)
    }

    #[test]
    fn test_is_valid() {
        let observer1 = Observer {
            latitude: 52.23,
            longitude: 21.01,
        };
        let observer2 = Observer {
            latitude: 200.0,
            longitude: 0.0,
        };
        assert!(observer1.is_valid());
        assert!(!observer2.is_valid());
    }

    #[test]
    fn test_moon_phase() {
        let phase1: JulianDate = JulianDate::from_calendar(2000, 01, 06).unwrap();
        assert!(phase1.moon_phase().abs() < 2.0);
        let phase2: JulianDate = JulianDate::from_calendar(2026, 05, 31).unwrap();
        assert!((phase2.moon_phase() - 100.0).abs() < 2.0);
    }
}
