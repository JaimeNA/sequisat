

use crate::orbit::Orbit;
use crate::tle::TLE;
use crate::propagator::SGP4;

use chrono::{Utc, TimeZone, NaiveDate, NaiveTime};

pub struct Satellite 
{
    propagator: SGP4,
    tle: TLE,
    points: Vec<(f64, f64)>
}  

impl Satellite
{
    pub fn new(tle_path: &str) -> Self
    {
        let tle = TLE::new(tle_path);
        let orbit = Orbit::new(&tle);
        let mut propagator = SGP4::new(orbit);

        propagator.calculate_constants();

        Satellite
        {
            propagator: propagator,
            tle: tle,
            points: Vec::new()
        }
    }

    pub fn print(&self)
    {
        self.tle.print_data();
    }

    pub fn get_points(&self) -> &Vec<(f64, f64)>
    {
        return &self.points;
    }

    pub fn get_trajectory(&mut self)
    {
        self.points.clear();

        for i in 0..2400
        {
            self.propagator.propagate(self.time_since_epoch_in_minutes() + i as f64);
            self.points.push((self.getLongitude() * (180.0/3.14159), self.getLatitude() * (180.0/3.14159)));
        }
    }

    pub fn getAltitude(&self) -> f64
    {
        return self.propagator.getAltitude();
    }

    pub fn getLatitude(&self) -> f64
    {
        return self.propagator.getLatitude();
    }

    pub fn getLongitude(&self) -> f64
    {
        return self.propagator.getLongitude();
    }

    pub fn update_position(&mut self)
    {
        self.propagator.propagate(self.time_since_epoch_in_minutes());
    }

    // Function to calculate the time difference between two NaiveDateTime in minutes
    pub fn time_since_epoch_in_minutes(&self) -> f64 {

        let epoch_year = self.tle.epoch_year; // TODO: fix
        let epoch_day = self.tle.epoch_day;

        let day_of_year = epoch_day as u32;

        // Convert the day of the year to a NaiveDate
        let tle_date = NaiveDate::from_yo_opt(epoch_year, day_of_year);

        // Calculate the time from the fractional day part (fraction of 24 hours)
        let seconds_in_day = 86400.0 * (epoch_day - day_of_year as f64);
        let tle_time = NaiveTime::from_num_seconds_from_midnight_opt(seconds_in_day as u32, 0);

        // Create a full TLE epoch DateTime in UTC
        let tle_datetime = Utc
            .from_utc_datetime(&NaiveDate::and_time(&tle_date.unwrap(), tle_time.unwrap()))
            .with_timezone(&Utc);

        // Get the current time in UTC
        let now = Utc::now();

        // Calculate the delta in minutes
        let delta = now.signed_duration_since(tle_datetime).num_seconds() as f64 / 60.0;

        delta
    }
}