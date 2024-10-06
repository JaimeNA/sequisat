
use std::fs;

pub struct Orbit {
    /// Angle between the equator and the orbit plane in rad
    pub inclination: f64,

    /// Angle between vernal equinox and the point where the orbit crosses the equatorial plane in rad
    pub right_ascension: f64,

    /// Shape of the orbit
    pub eccentricity: f64,

    /// Angle between the ascending node and the orbit's point of closest approach to the earth in rad
    pub argument_of_perigee: f64,

    /// Angle of the satellite location measured from perigee in rad
    pub mean_anomaly: f64,

    /// Mean number of orbits per day in rad.min⁻¹
    pub mean_motion: f64,
}

impl Orbit
{
    pub fn from_tle(file_path: &str) -> Self
    {
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let mut lines = contents.lines();

        let mut current_line = lines.next().unwrap();

        println!("Data obtained from TLE: "); // TODO: Implements security checks
        let mut columns = current_line.split_whitespace();
        columns.next();

        // ---------------- General Information ----------------
        println!("  ----- General Information ----- ");
        // Catalog number
        let mut current_column = columns.next().unwrap();

        println!("Satellite Catalog Number:  {}", &current_column[..5]);

        // Classification
        print!("Classification:             ");
        match current_column.chars().nth(5).unwrap()
        {
            'U' => println!("Unclassified"),
            'C' => println!("Classified"),
            'A' => println!("Secret"),
            _ => println!("Unknown"),
        };

        // Launch year
        current_column = columns.next().unwrap();
        let year_str = &current_column[..2];

        let mut year = year_str.parse::<i32>().unwrap();
        if (year < 57)
        {
            year += 2000; 
        }
        else
        {
            year += 1900;
        }
        
        println!("Launch Year:                {}", year);

        // Launch number of the year
        println!("Launch Number of the Year:  {}", &current_column[3..5]);
        // Launch piece
        println!("Launch Piece:               {}", current_column.chars().nth(5).unwrap());

        // ---------------- Epoch Orbit Information ----------------
        println!("\n  ----- Epoch Orbit Information ----- ");

        // Epoch Year
        current_column = columns.next().unwrap();
        let year_str = &current_column[..2];

        let mut year = year_str.parse::<i32>().unwrap();
        if (year < 57)
        {
            year += 2000; 
        }
        else
        {
            year += 1900;
        }
        
        println!("Epoch Year:                 {}", year);


        // Epoch Day of Year
        let day_str = &current_column[3..];
        let day = day_str.parse::<f64>().unwrap();

        println!("Epoch Day Of Year:          {}", day);

        // Ballistic Coefficient
        current_column = columns.next().unwrap();
        let ballistic_coefficient = current_column.parse::<f64>().unwrap();
        println!("Ballistic Coefficient:      {}", ballistic_coefficient);

        // Drag Term
        columns.next(); // Second derivatice of mean motion
        current_column = columns.next().unwrap();

        let temp = {if (current_column.chars().nth(0).unwrap() == '-')
        {
            format!("{}0.{}e{}", current_column.chars().nth(0).unwrap(), &current_column[1..6], &current_column[6..]) // Decimal point assumed
        }
        else
        {
            format!("0.{}e{}", &current_column[0..5], &current_column[5..])
        }};
        
        println!("Drag Term:                  {}", temp.parse::<f64>().unwrap());

        // Inclination (radians)
        current_line = lines.next().unwrap();
        columns = current_line.split_whitespace();

        columns.next();                         // Line number
        columns.next();                         // Catalog number

        current_column = columns.next().unwrap();
        let inclination = (current_column.parse::<f64>().unwrap() * core::f64::consts::PI) / 180.0;
        println!("Inclination(radians):       {}", inclination);

        

        Orbit{
            inclination: 0.0,
            right_ascension: 0.0,
            eccentricity: 0.0,
            argument_of_perigee: 0.0,
            mean_anomaly: 0.0,
            mean_motion: 0.0
        }
    }
}

pub struct Geopotential {
    /// Equatorial radius of the earth in km
    // aₑ
    pub ae: f64,

    /// square root of earth's gravitational parameter in earth radii³ min⁻²
    // kₑ
    pub ke: f64,

    /// un-normalised second zonal harmonic
    // J₂
    pub j2: f64,

    /// un-normalised third zonal harmonic
    // J₃
    pub j3: f64,

    /// un-normalised fourth zonal harmonic
    // J₄
    pub j4: f64,
}

pub enum HighAltitude {
    No {},
    // Yes {
    //     c5: f64,
    //     d2: f64,
    //     d3: f64,
    //     d4: f64,
    //     eta: f64,
    //     k7: f64,
    //     k8: f64,
    //     k9: f64,
    //     k10: f64,
    //     elliptic: Elliptic,
    // },
}

pub enum Method {
    NearEarth {
        a0: f64,
        k2: f64,
        k3: f64,
        k4: f64,
        k5: f64,
        k6: f64,
        high_altitude: HighAltitude,
    },
    // DeepSpace {
    //     eccentricity_dot: f64,
    //     inclination_dot: f64,
    //     solar_perturbations: third_body::Perturbations,
    //     lunar_perturbations: third_body::Perturbations,
    //     resonant: Resonant,
    // },
}

pub struct Constants {
    pub geopotential: Geopotential,
    pub right_ascension_dot: f64,
    pub argument_of_perigee_dot: f64,
    pub mean_anomaly_dot: f64,
    pub c1:         f64,
    pub c4:         f64,
    pub k0:         f64,
    pub k1:         f64,
    pub method:     Method,
    pub orbit_0:    Orbit,
}