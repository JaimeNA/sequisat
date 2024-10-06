
use chrono::prelude::*;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike, Utc, Duration};
mod sgp4;
mod sgp4_propagator;

use sgp4_propagator::TLE;
use sgp4_propagator::Orbit;

use std::thread;
use std::time::Duration as StdDuration;

fn main() {

    let tle = TLE::from_file("noaa.tle");
    tle.print_data();
    let epoch_year = tle.epoch_year;
    let epoch_day = tle.epoch_day;

    let orbit_0 = Orbit::from_tle(tle);
    let mut iss = sgp4::SGP4::new(orbit_0);

    iss.set_constants();

    iss.print_data();

    // Set the update interval (e.g., 1 second)
    let update_interval = StdDuration::from_secs(1);

    // Start the continuous update loop
    loop {
        // Get the current time (UTC)
        let current_time = Utc::now().naive_utc();

        // Calculate the time since the epoch in minutes
        let time_since_epoch = time_since_epoch_in_minutes(epoch_year, epoch_day);

        // Display the result
        println!("Time since epoch: {} minutes", time_since_epoch);
        iss.update_gravity_and_atm_drag(time_since_epoch);

        // Wait for the update interval
        thread::sleep(update_interval);
    }
}


    // Function to calculate the time difference between two NaiveDateTime in minutes
    pub fn time_since_epoch_in_minutes(epoch_year :i32, epoch_day :f64) -> f64 {

        let day_of_year = epoch_day as u32;

        // Convert the day of the year to a NaiveDate
        let tle_date = NaiveDate::from_yo(epoch_year, day_of_year);

        // Calculate the time from the fractional day part (fraction of 24 hours)
        let seconds_in_day = 86400.0 * (epoch_day - day_of_year as f64);
        let tle_time = NaiveTime::from_num_seconds_from_midnight(seconds_in_day as u32, 0);

        // Create a full TLE epoch DateTime in UTC
        let tle_datetime = Utc
            .from_utc_datetime(&NaiveDate::and_time(&tle_date, tle_time))
            .with_timezone(&Utc);

        // Get the current time in UTC
        let now = Utc::now();

        // Calculate the delta in minutes
        let delta = now.signed_duration_since(tle_datetime).num_seconds() as f64 / 60.0;

        delta
    }

