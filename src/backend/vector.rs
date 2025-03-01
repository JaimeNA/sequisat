#[derive(Clone)]
pub struct PositionVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PositionVector {

    const SEMI_MAYOR_AXIS: f64 = 6378.137; // Equatorial radius
    const SEMI_MINOR_AXIS: f64 = 6356.752; // Polar radius

    pub fn new(x:f64, y: f64, z: f64) -> Self
    {
        Self {
            x, 
            y,
            z
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn sum(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    
    pub fn sub(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    // Getters and Setters

    pub fn set_x(&mut self, x: f64)
    {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f64)
    {
        self.y = y;
    }

    pub fn set_z(&mut self, z: f64)
    {
        self.z = z;
    }

    pub fn get_x(&self) -> f64 
    {
        self.x
    }

    pub fn get_y(&self) -> f64 
    {
        self.y
    }

    pub fn get_z(&self) -> f64 
    {
        self.z
    }

    /* 
     * The geodetic vetor should have the followiung format: (latitude, longitude, height)
     * while the ecef will be: (x, y, z)
    */
    pub fn geodetic_to_ecef(&self) -> PositionVector
    {
        // https://en.wikipedia.org/wiki/Geographic_coordinate_conversion
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html

        // Compute ECEF coordinates

        let n = self.prime_vertical_radius(self.x);

        PositionVector::new(
            (self.get_z() + n)*self.get_y().cos()*self.get_x().cos(), 
            (self.get_z() + n)*self.get_y().sin()*self.get_x().cos(), 
            (self.get_z() + n)*self.get_x().sin()
        )
    }

    pub fn ecef_to_geodetic(&self) -> PositionVector
    {
        // https://en.wikipedia.org/wiki/Geodetic_coordinates

        let p = (self.get_x().powi(2) + self.get_y().powi(2)).sqrt();

        let lon = self.get_y().atan2(self.get_x());
        let lat = (self.get_z() / p).atan();

        let h = p / lat.cos();
        
        PositionVector::new(lat, lon, h - self.prime_vertical_radius(lat))
    }

    // Takes the sat vector as geodetic and using the client vector its computes the ENU coordinates
    pub fn ecef_to_enu(client_ecef: &PositionVector, satellite_ecef: &PositionVector) -> PositionVector
    {
        // https://gssc.esa.int/navipedia/index.php/Transformations_between_ECEF_and_ENU_coordinates
        
        let client_geodetic = client_ecef.ecef_to_geodetic();

        let mut p = satellite_ecef.clone();   // TODO: Better naming 
    
        p.sub(&client_ecef);
    
        let p_module = (p.get_x().powi(2) + p.get_y().powi(2) + p.get_z().powi(2)).sqrt();
    
        let p_normalized = PositionVector::new(p.get_x() / p_module, p.get_y() / p_module, p.get_z() / p_module); // TODO: implement as part of PositionVector
    
        // Apply rotation matrix
        let lat = client_geodetic.get_x();
        let lon = client_geodetic.get_y();
    
        let p_enu = PositionVector::new(-p_normalized.get_x()*lon.sin() + p_normalized.get_y()*lon.cos(),
                                -p_normalized.get_x()*lon.cos()*lat.sin() - p_normalized.get_y()*lon.sin()*lat.sin() + p_normalized.get_z()*lat.cos(),
                                p_normalized.get_x()*lon.cos()*lat.cos() + p_normalized.get_y()*lon.sin()*lat.cos() + p_normalized.get_z()*lat.sin());
    
        return p_enu.clone(); // (e, n, u)
    }

    pub fn enu_to_azimuth_and_elevation(&self) -> PositionVector {
        // https://gssc.esa.int/navipedia/index.php/Transformations_between_ECEF_and_ENU_coordinates

        let p_module = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
        let p_enu_normalized = PositionVector::new(self.get_x() / p_module, self.get_y() / p_module, self.get_z() / p_module);

        let p_spheric = PositionVector::new(self.get_x().atan2(self.get_y()),  self.get_z().asin(), 0.0);
        
        PositionVector::new(p_spheric.get_x(), p_spheric.get_y(), 0.0) // (a, e, -)
    }

    // Computes the prime vertical radius of curvature
    fn prime_vertical_radius(&self, latitude: f64) -> f64
    {
        let a = Self::SEMI_MAYOR_AXIS;
        let b = Self::SEMI_MINOR_AXIS;

        let e_squared = 1.0 - (b*b)/(a*a);

        let temp_1 = (latitude.sin()).powi(2);
        let temp_2 = (1.0 - e_squared*temp_1).sqrt();

        a / temp_2
    }
}