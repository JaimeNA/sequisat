
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc, Duration};
mod sgp4;

use std::thread;
use std::time::Duration as StdDuration;

// ISS (ZARYA)             <- Test TLE data
// 1 25544U 98067A   24274.84428341  .00028761  00000-0  51008-3 0  9994
// 2 25544  51.6383 152.0493 0007453  48.7023  48.3483 15.49976714474935

// ISS (ZARYA)             <- Test TLE data
// 1 25544U 98067A   24275.87455365  .00025486  00000-0  45228-3 0  9992
// 2 25544  51.6386 146.9452 0007430  51.5965  38.1682 15.50028681475096

fn main() {

    // Data from TLE
    let i0: f64 = (51.6386 * core::f64::consts::PI) / 180.0;   // Converted to degrees
    let e0: f64 = 0.0007430;
    let n0: f64 = (15.50028681475096 * core::f64::consts::PI) / 720.0;
    let m0: f64 = (38.1682 * core::f64::consts::PI) / 180.0;
    let omega0: f64 = (146.9452 * core::f64::consts::PI) / 180.0;
    let w0: f64 = (51.5965 * core::f64::consts::PI) / 180.0;
    let bstar: f64 = 0.00045228;

    let mut iss = sgp4::SGP4::new();

    iss.initialize(n0, i0, e0, bstar, w0);

    // for hours in 0..24 {
    //     println!("t = {} min", hours * 60);
    //     iss.update_gravity_and_atm_drag(m0, omega0, (hours * 60) as f64);
    //     //println!("    r = {:?} km", prediction.position);
    //     //println!("    ṙ = {:?} km.s⁻¹", prediction.velocity);
    // }

    // Set the update interval (e.g., 1 second)
    let update_interval = StdDuration::from_secs(1);
   
    // Start the continuous update loop
    loop {
         // Get the current time (UTC)
         let current_time = Utc::now().naive_utc();
 
         // Calculate the time since the epoch in minutes
         let time_since_epoch = sgp4::SGP4::time_since_epoch_in_minutes();
 
         // Display the result
         println!("Time since epoch: {} minutes", time_since_epoch);
         iss.update_gravity_and_atm_drag(m0, omega0, time_since_epoch);
 
         // Wait for the update interval
         thread::sleep(update_interval);
    }
}
