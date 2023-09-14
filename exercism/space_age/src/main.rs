// Given an age in seconds, calculate how old someone would be on:
// Mercury: orbital period 0.2408467 Earth years
// Venus: orbital period 0.61519726 Earth years
// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
// Mars: orbital period 1.8808158 Earth years
// Jupiter: orbital period 11.862615 Earth years
// Saturn: orbital period 29.447498 Earth years
// Uranus: orbital period 84.016846 Earth years
// Neptune: orbital period 164.79132 Earth years

#[derive(Debug)]
pub struct Duration;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        unimplemented!("s, measured in seconds: {s}")
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury {
    seconds: i32
}
pub struct Venus {
    seconds: i32
}
pub struct Earth {
    seconds: i32
}
pub struct Mars {
    seconds: i32
}
pub struct Jupiter {
    seconds: i32
}
pub struct Saturn {
    seconds: i32
}
pub struct Uranus {
    seconds: i32
}
pub struct Neptune {
    seconds: i32
}

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}

fn main() {
    println!("Hello, world!");
}
