
use super::orbit::Orbit;
use super::tle::TLE;
use super::propagator::SGP4;

use chrono::{Utc, TimeZone, NaiveDate, NaiveTime, Timelike};
use julian::Calendar;

pub struct Satellite 
{
    propagator: SGP4,
    tle: TLE,
    points: Vec<(f64, f64)>,
    gst: f64
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
            points: Vec::new(),
            gst: 0.0
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

    pub fn get_julian_day() -> f64
    {
        let cal = Calendar::GREGORIAN;
        let date = cal.now().unwrap();

        let jd0 = date.0.julian_day_number() as f64 - 0.5;

        let now = Utc::now();
        let days_since_jd0 = now.num_seconds_from_midnight() as f64 / (3600.0*24.0);

        return jd0 as f64 + days_since_jd0;
    }

    pub fn get_trajectory(&mut self)
    {
        self.points.clear();
        
        for i in -60..60
        {
            self.gst = self.get_gst(Self::get_julian_day() + (i as f64 / (60.0*24.0)));
            self.propagator.propagate(self.time_since_epoch_in_minutes() + i as f64);
            self.points.push((self.get_longitude() * (180.0/core::f64::consts::PI), self.get_latitude() * (180.0/core::f64::consts::PI)));
        }
    }

    fn get_gst(&self, julian_time: f64) -> f64
    {
        let j2000 = 2451545.0;  // Epoch of reference

        let jd0 = julian_time.floor() + 0.5;

        let days_since_jd0 = julian_time - jd0;
        let hours_since_jd0 = days_since_jd0 * 24.0;  // H

        let days_since_j2000  = julian_time - j2000;    // D
        let centuries_since_j2000  = days_since_j2000  / 36525.0;   // T
        let whole_days_since_j2000  = jd0 - j2000;

        let gmst = 6.697374558 + 0.06570982441908*whole_days_since_j2000  + 1.00273790935*hours_since_jd0 + 0.000026*centuries_since_j2000.powi(2);

        let gmst_normalized = gmst.rem_euclid(24.0);

        let gmst_rads = gmst_normalized * (core::f64::consts::PI/12.0);

        return gmst_rads;
    }

    pub fn get_altitude(&self) -> f64
    {
        return self.propagator.get_altitude();
    }

    pub fn get_latitude(&self) -> f64
    {
        return self.propagator.get_latitude();
    }

    pub fn get_longitude(&self) -> f64
    {
        return (self.propagator.get_longitude() - self.gst + core::f64::consts::PI).rem_euclid(2.0*core::f64::consts::PI) - core::f64::consts::PI; // Normalize to range
    }

    pub fn get_tle(&self) -> &TLE
    {
        return &self.tle;
    }

    pub fn update_position(&mut self)
    {
        self.gst = self.get_gst(Self::get_julian_day());

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

    //TODO: Work with vectors!
}