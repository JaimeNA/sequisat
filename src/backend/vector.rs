#[derive(Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {

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


    pub fn to_cartesian(&self) -> Vector3
    {
        Vector3::new(
            self.get_z()*self.get_x().cos()*self.get_y().cos(), 
            self.get_z()*self.get_x().sin()*self.get_y().cos(), 
            self.get_z()*self.get_y().sin()
        )
    }

    pub fn to_geodetic(&self) -> Vector3
    {
        // https://en.wikipedia.org/wiki/Geodetic_coordinates

        let p = (self.get_x().powi(2) + self.get_y().powi(2)).sqrt();

        let h = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
        let gamma = self.get_y().atan2(self.get_x());
        let phi = (self.get_z() / p).atan(); // TODO: make another vector type just for this coords

        Vector3::new(gamma, phi, h)
    }
}