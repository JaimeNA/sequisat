
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc, Duration};
mod sgp4;
mod sgp4_propagator;

use sgp4_propagator::Orbit;

use std::thread;
use std::time::Duration as StdDuration;

// ISS (ZARYA)             <- Test TLE data
// 1 25544U 98067A   24279.46943132  .00032942  00000-0  59769-3 0  9995
// 2 25544  51.6363 129.1425 0009147  39.7917 320.3740 15.49242162475659

fn main() {

    // Data from TLE
    let i0: f64 = (51.6363 * core::f64::consts::PI) / 180.0;   // Converted to degrees
    let e0: f64 = 0.0009147;
    let n0: f64 = (15.49242162475659 * core::f64::consts::PI) / 720.0;
    let m0: f64 = (320.3740 * core::f64::consts::PI) / 180.0;
    let omega0: f64 = (129.1425 * core::f64::consts::PI) / 180.0;
    let w0: f64 = (39.7917 * core::f64::consts::PI) / 180.0;
    let bstar: f64 = 0.00059769;

    let orbit_0 = Orbit::from_tle("noaa.tle");

    let mut iss = sgp4::SGP4::new();

    iss.initialize(n0, i0, e0, bstar, w0);

}
