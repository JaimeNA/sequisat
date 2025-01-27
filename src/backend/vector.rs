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
            self.get_x()*self.get_y().cos()*self.get_z().sin(), 
            self.get_x()*self.get_y().sin()*self.get_z().sin(), 
            self.get_x()*self.get_z().cos()
        )
    }

    pub fn to_spheric(&self) -> Vector3
    {
        let radius = (self.get_x().powi(2) + self.get_y().powi(2) + self.get_z().powi(2)).sqrt();
        let longitude = self.get_y().atan2(self.get_x());


        let latitude = (self.get_z() / (self.get_x().powi(2) + self.get_y().powi(2)).sqrt()).atan(); // NOTE: Spherical coordinates differ from the mathematical ones

        Vector3::new(radius, longitude, latitude)
    }
}