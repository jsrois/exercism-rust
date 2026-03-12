// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

const SECONDS_PER_YEAR: f64 = 60.0 * 60.0 * 24.0 * 365.25;
impl Duration {
    fn in_earth_years(&self) -> f64 {
        self.seconds as f64 / SECONDS_PER_YEAR
    }
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ( $( ($p:ident, $c:expr)),*  ) => {
        $(
pub struct $p;
impl Planet for $p {
    fn years_during(d: &Duration) -> f64 {
        let orbital_period_in_earth_years = $c;
        d.in_earth_years() / orbital_period_in_earth_years
    }
}
        )*
    };
}
planet!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
