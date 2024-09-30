
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc, Duration};
mod sgp4;

use std::thread;
use std::time::Duration as StdDuration;
// ISS (ZARYA)             <- Test TLE data
// 1 25544U 98067A   24272.20796705  .00058591  00000+0  10319-2 0  9996
// 2 25544  51.6377 165.1137 0006922  38.3252  99.3437 15.49843852474523

fn main() {

    // Data from TLE
    let i0: f64 = (51.6377 * core::f64::consts::PI) / 180.0;   // Converted to degrees
    let e0: f64 = 0.0006922;
    let n0: f64 = (15.49843852 * core::f64::consts::PI) / 720.0;
    let m0: f64 = (99.3437 * core::f64::consts::PI) / 180.0;
    let omega0: f64 = (165.1137 * core::f64::consts::PI) / 180.0;
    let w0: f64 = (38.3252 * core::f64::consts::PI) / 180.0;
    let bstar: f64 = 0.010319;

    let mut iss = sgp4::SGP4::new();
    iss.initialize(n0, i0, e0, bstar, w0);
    
    for hours in 0..24 {
        println!("t = {} min", hours * 60);
        iss.update_gravity_and_atm_drag(m0, omega0, (hours * 60) as f64);
        //println!("    r = {:?} km", prediction.position);
        //println!("    ṙ = {:?} km.s⁻¹", prediction.velocity);
    }
}
