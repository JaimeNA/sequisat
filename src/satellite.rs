
use crate::orbit::Orbit;
use crate::tle::TLE;
use crate::propagator::SGP4;

use chrono::{Utc, TimeZone, NaiveDate, NaiveTime, Timelike};
use julian::{Calendar, Month};

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

        let JD0 = date.0.julian_day_number() as f64 - 0.5;

        let now = Utc::now();
        let days_since_JD0 = (now.num_seconds_from_midnight() as f64 / (3600.0*24.0));

        return JD0 as f64 + days_since_JD0;
    }

    pub fn get_trajectory(&mut self)
    {
        self.points.clear();
        
        for i in 0..180
        {
            self.gst = self.getGST(Self::get_julian_day() + (i as f64 / (60.0*24.0)));
            self.propagator.propagate(self.time_since_epoch_in_minutes() + i as f64);
            self.points.push((self.getLongitude() * (180.0/3.14159), self.getLatitude() * (180.0/3.14159)));
        }
    }

    const G: f64 = 6.697374558;
    pub fn getGST(&self, julian_time: f64) -> f64
    {
        let J2000 = 2451545.0;

        let JD0 = julian_time.floor();

        let days_since_JD0 = julian_time - JD0;
        let hours_since_JD0 = days_since_JD0 * 24.0;  // H

        let days_since_J2000  = julian_time - J2000;    // D
        let centuries_since_J2000  = days_since_J2000  / 36525.0;   // T
        let whole_days_since_J2000  = JD0 - J2000;

        //let GMST = 6.697374558 + 0.06570982441908*whole_days_since_J2000  + 1.00273790935*hours_since_JD0 + 0.000026*centuries_since_J2000 .powi(2);
        let GMST0 = 100.46061837 + 36000.770053608*centuries_since_J2000 + 0.000387933*centuries_since_J2000.powi(2) - (1.0/38710000.0)*centuries_since_J2000.powi(3);

        let GMST = GMST0 + 360.98564736629*days_since_JD0;

        let gmst_sec_normalized = GMST.rem_euclid(360.0);
        let GMST_rads = gmst_sec_normalized * (3.14159/180.0);

        return GMST_rads;
    }

    // pub fn getGST(&self) -> f64
    // {
    //     let JD_REF = 2451545.0;
    //     let cal = Calendar::GREGORIAN;
    //     let date = cal.now().unwrap();

    //     let JD0 = date.0.julian_day_number() as f64 + 0.5;

    //     let now = Utc::now();
    //     let days_since_JD0 = now.num_seconds_from_midnight() as f64 / (3600.0*24.0);
    //     let hours_since_JD0 = now.num_seconds_from_midnight() as f64 / 3600.0;  // H

    //     let days_since_JD_REF = (JD0 + days_since_JD0) - JD_REF;    // D
    //     let centuries_since_JD_REF = days_since_JD_REF / 36525.0;   // T
    //     let whole_days_since_JD_REF = JD0 - JD_REF;

    //     let GMST = 6.697374558 + 0.06570982441908*whole_days_since_JD_REF + 1.00273790935*hours_since_JD0 + 0.000026*centuries_since_JD_REF.powi(2);

    //     let minutes_per_day = 24.0 * 60.0;

    //     let GMST_rads = GMST * (2.0*3.14159) / minutes_per_day;

    //     return GMST_rads;
    // }

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
        return (self.propagator.getLongitude() + self.gst + 3.14159).rem_euclid(2.0*3.14159) - 3.14159; // Normalize to range
    }

    pub fn update_position(&mut self)
    {
        let cal = Calendar::GREGORIAN;
        let date = cal.now().unwrap();

        let JD0 = date.0.julian_day_number() as f64 + 0.5;

        let now = Utc::now();
        let days_since_JD0 = now.num_seconds_from_midnight() as f64 / (3600.0*24.0);

        self.gst = self.getGST(Self::get_julian_day());

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