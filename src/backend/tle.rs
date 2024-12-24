use std::fs;

pub struct TLE {

    /// Catalog Number
    pub catalog_number: i32,

    /// Classification
    pub classification: String,

    /// Launch Year
    pub launch_year: i32,

    /// Launch Piece
    pub launch_piece: char,

    /// Epoch Year
    pub epoch_year: i32,

    /// Epoch Day Of Year
    pub epoch_day: f64,

    /// Ballistic Coefficient
    pub ballistic_coefficient: f64,

    /// B star
    pub  drag_term: f64,

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
    pub mean_motion: f64
}

impl TLE
{
    pub fn new(file_path: &str) -> Self
    {
        let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

        let mut lines = contents.lines();

        let mut current_line = lines.next().unwrap();

        let mut columns = current_line.split_whitespace(); // TODO: Implements security checks
        columns.next();

        // ---------------- General Information ----------------
        // Catalog number
        let mut current_column = columns.next().unwrap();
        let catalog_number = (&current_column[..5]).parse::<i32>().unwrap();

        // Classification 
        let classification = match current_column.chars().nth(5).unwrap()
        {
            'U' =>  "Unclassified",
            'C' =>  "Classified",
            'A' =>  "Secret",
            _ =>    "Unknown",
        };

        // Launch year
        current_column = columns.next().unwrap();
        let year_str = &current_column[..2];

        let mut launch_year = year_str.parse::<i32>().unwrap();
        if launch_year < 57
        {
            launch_year += 2000; 
        }
        else
        {
            launch_year += 1900;
        }
        
        // Launch piece
        let launch_piece = current_column.chars().nth(5).unwrap();

        // ---------------- Epoch Orbit Information ----------------

        // Epoch Year
        current_column = columns.next().unwrap();
        let year_str = &current_column[..2];

        let mut epoch_year = year_str.parse::<i32>().unwrap();
        if epoch_year < 57
        {
            epoch_year += 2000; 
        }
        else
        {
            epoch_year += 1900;
        }

        // Epoch Day of Year
        let day_str = &current_column[2..];
        let epoch_day = day_str.parse::<f64>().unwrap();

        // Ballistic Coefficient
        current_column = columns.next().unwrap();
        let ballistic_coefficient = current_column.parse::<f64>().unwrap();

        // Drag Term
        columns.next(); // Second derivatice of mean motion
        current_column = columns.next().unwrap();

        let temp = {
        if current_column.chars().nth(0).unwrap() == '-'
        {
            format!("-0.{}e{}", &current_column[1..6], &current_column[6..]) // Decimal point assumed
        }
        else
        {
            format!("0.{}e{}", &current_column[0..5], &current_column[5..])
        }};
        let drag_term = temp.parse::<f64>().unwrap();

        // Inclination (radians)
        current_line = lines.next().unwrap();
        columns = current_line.split_whitespace();

        columns.next();                         // Line number
        columns.next();                         // Catalog number

        current_column = columns.next().unwrap();
        let inclination = (current_column.parse::<f64>().unwrap() * core::f64::consts::PI) / 180.0;

        // Right ascension (radians)
        current_column = columns.next().unwrap();
        let right_ascension = (current_column.parse::<f64>().unwrap() * core::f64::consts::PI) / 180.0;

        // Eccentricity
        current_column = columns.next().unwrap();
        let eccentricity_str = format!("0.{}", current_column);
        let eccentricity = eccentricity_str.parse::<f64>().unwrap();

        // Argument of Perigee
        current_column = columns.next().unwrap();
        let argument_of_perigee = (current_column.parse::<f64>().unwrap() * core::f64::consts::PI) / 180.0;

        
        // Mean Anomaly (radians)
        current_column = columns.next().unwrap();
        let mean_anomaly = (current_column.parse::<f64>().unwrap() * core::f64::consts::PI) / 180.0;

        // Mean Motion (radians/min)
        current_column = columns.next().unwrap();
        let mean_motion = (current_column.parse::<f64>().unwrap() * 2.0*core::f64::consts::PI) / 1440.0;
        
        TLE{
            catalog_number: catalog_number,
            classification: classification.to_string(),
            launch_year: launch_year,
            launch_piece: launch_piece,
            epoch_year: epoch_year,
            epoch_day: epoch_day,
            ballistic_coefficient: ballistic_coefficient,
            drag_term:  drag_term,
            inclination: inclination,
            right_ascension: right_ascension,
            eccentricity: eccentricity,
            argument_of_perigee: argument_of_perigee,
            mean_anomaly: mean_anomaly,
            mean_motion: mean_motion
        }
    }

    pub fn print_data(&self)
    {
        println!("TLE Data: ");
        println!("  ----- General Information ----- ");
        println!("Satellite Catalog Number:   {}", self.catalog_number);
        println!("Classification:             {}", self.classification);
        println!("Launch Year:                {}", self.launch_year);
        println!("Launch Piece:               {}", self.launch_piece);

        println!("\n  ----- Epoch Orbit Information ----- ");
        println!("Epoch Year:                 {}", self.epoch_year);
        println!("Epoch Day Of Year:          {}", self.epoch_day);
        println!("Ballistic Coefficient:      {}", self.ballistic_coefficient);
        println!("Drag Term:                  {}", self.drag_term);
        println!("Inclination(rads):          {}", self.inclination);
        println!("Right Ascension(rads):      {}", self.right_ascension);
        println!("Eeccentricity:              {}", self.eccentricity);
        println!("Argument of Perigee(rads):  {}", self.argument_of_perigee);
        println!("Mean Anomaly(rads):         {}", self.mean_anomaly);
        println!("Mean Motion(radians/min):   {}", self.mean_motion);
    }

    // Getters

    pub fn get_catalog_number(&self) -> i32
    {
        return self.catalog_number;
    }

    pub fn get_classification(&self) -> String
    {
        return self.classification.clone();
    }
    
    pub fn get_launch_year(&self) -> i32
    {
        return self.launch_year;
    }
    
    pub fn get_launch_piece(&self) -> char
    {
        return self.launch_piece;
    }
    
    pub fn get_epoch_year(&self) -> i32
    {
        return self.epoch_year;
    }
    
    pub fn get_epoch_day(&self) -> f64
    {
        return self.epoch_day;
    }
    
    pub fn get_ballistic_coefficient(&self) -> f64
    {
        return self.ballistic_coefficient;
    }
    
    pub fn get_drag_term(&self) -> f64
    {
        return self.drag_term;
    }
    
    pub fn get_inclination(&self) -> f64
    {
        return self.inclination;
    }
    
    pub fn get_right_ascension(&self) -> f64
    {
        return self.right_ascension;
    }
    
    pub fn get_eccetricity(&self) -> f64
    {
        return self.eccentricity;
    }
    
    pub fn get_argument_of_perigee(&self) -> f64
    {
        return self.argument_of_perigee;
    }
    
    pub fn get_mean_anomaly(&self) -> f64
    {
        return self.mean_anomaly;
    }

    pub fn get_mean_motion(&self) -> f64
    {
        return self.mean_motion;
    }
}


